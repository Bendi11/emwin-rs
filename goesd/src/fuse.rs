use std::{collections::HashMap, ffi::OsStr, time::{SystemTime, Duration}, io::Write, path::PathBuf, sync::Arc};

use fuser::{Filesystem, FileAttr, FileType};
use goes_cfg::Config;
use goes_sql::GoesSqlContext;
use tokio::runtime::Runtime;


/// FUSE Filesystem state for EMWIN file reception
pub struct EmwinFS {
    inodes: HashMap<u64, EmwinFSEntry>,
    ino: u64,
    rt: Arc<Runtime>,
    ctx: Arc<GoesSqlContext>,
    cfg: Arc<Config>,
}

impl EmwinFS {
    const INIT_SIZE: u64 = 1024;
    
    /// Create a new FUSE filesystem spawning parser threads to the given runtime
    pub fn new(rt: Arc<Runtime>, ctx: Arc<GoesSqlContext>, cfg: Arc<Config>) -> Self {
        Self {
            inodes: HashMap::new(),
            ino: 0,
            rt,
            ctx,
            cfg,
        }
    }
}

struct EmwinFSEntry {
    bytes: Vec<u8>,
    name: PathBuf,
}

impl Filesystem for EmwinFS {
    fn mknod(
            &mut self,
            req: &fuser::Request<'_>,
            _parent: u64,
            name: &OsStr,
            _mode: u32,
            _umask: u32,
            _rdev: u32,
            reply: fuser::ReplyEntry,
        ) {
        let name = PathBuf::from(name); 
        let file = EmwinFSEntry { bytes: Vec::with_capacity(Self::INIT_SIZE as usize), name };
        let ino = self.ino;
        self.ino += 1;
        self.inodes.insert(ino, file);

        let attr = FileAttr {
            ino,
            size: Self::INIT_SIZE,
            blocks: 1,
            atime: SystemTime::UNIX_EPOCH,
            mtime: SystemTime::UNIX_EPOCH,
            ctime: SystemTime::UNIX_EPOCH,
            crtime: SystemTime::UNIX_EPOCH,
            kind: FileType::RegularFile,
            perm: 0o220,
            nlink: 0,
            uid: req.uid(),
            gid: req.gid(),
            rdev: 0,
            blksize: Self::INIT_SIZE as u32,
            flags: 0,
        };

        reply.entry(&Duration::from_secs(0), &attr, 0);
    }
    
    fn write(
            &mut self,
            _req: &fuser::Request<'_>,
            ino: u64,
            _fh: u64,
            offset: i64,
            data: &[u8],
            _write_flags: u32,
            _flags: i32,
            _lock_owner: Option<u64>,
            reply: fuser::ReplyWrite,
        ) {
        match self.inodes.get_mut(&ino) {
            Some(entry) => {
                if offset != entry.bytes
                    .len()
                    .saturating_sub(1) as i64 {
                    log::error!("Failed to write to in-memory file: offset was not equal to last byte, got {}", offset);
                    return reply.written(0)
                }

                match entry
                    .bytes
                    .write(data) {
                    Err(e) => {
                        log::error!("Failed to write {} bytes to buffer: {}", data.len(), e);
                        return reply.error(5)
                    },
                    Ok(n) => {
                        reply.written(n as u32);
                    }
                }
            },
            None => {
                reply.error(2);
            }
        } 
    }

    fn release(
            &mut self,
            _req: &fuser::Request<'_>,
            ino: u64,
            _fh: u64,
            _flags: i32,
            _lock_owner: Option<u64>,
            _flush: bool,
            reply: fuser::ReplyEmpty,
        ) {
        match self.inodes.remove(&ino) {
            Some(entry) => {
                let ctx = self.ctx.clone();
                let cfg = self.cfg.clone();
                self.rt.spawn(async move {
                    let text = match std::str::from_utf8(&entry.bytes) {
                        Ok(text) => text,
                        Err(e) => {
                            log::error!("Failed to convert written bytes to utf-8: {}", e);
                            return
                        }
                    };
                    crate::dispatch::emwin_dispatch(entry.name, text, ctx, cfg).await;
                });

                return reply.ok()
            },
            None => {
                return reply.error(2)
            }
        }
    }
}
