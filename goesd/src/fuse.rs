use std::{
    collections::HashMap,
    ffi::OsStr,
    io::Write,
    path::PathBuf,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use fuser::{FileAttr, FileType, Filesystem};
use goes_sql::GoesSqlContext;
use tokio::runtime::Runtime;

/// FUSE Filesystem state for EMWIN file reception
pub struct EmwinFS {
    inodes: HashMap<u64, EmwinFSEntry>,
    ino: u64,
    rt: Arc<Runtime>,
    ctx: Arc<GoesSqlContext>,
}

const TTL: Duration = Duration::from_secs(1);

impl EmwinFS {
    const INIT_SIZE: u64 = 1024;
    const ROOT_ATTR: FileAttr = FileAttr {
        ino: 1,
        size: 0,
        blocks: 0,
        atime: UNIX_EPOCH, // 1970-01-01 00:00:00
        mtime: UNIX_EPOCH,
        ctime: UNIX_EPOCH,
        crtime: UNIX_EPOCH,
        kind: FileType::Directory,
        perm: 0o755,
        nlink: 2,
        uid: 501,
        gid: 20,
        rdev: 0,
        flags: 0,
        blksize: 512,
    };


    /// Create a new FUSE filesystem spawning parser threads to the given runtime
    pub fn new(rt: Arc<Runtime>, ctx: Arc<GoesSqlContext>) -> Self {
        Self {
            inodes: HashMap::new(),
            ino: 2,
            rt,
            ctx,
        }
    }
}

struct EmwinFSEntry {
    bytes: Vec<u8>,
    name: PathBuf,
    uid: u32,
    gid: u32,
}

impl EmwinFSEntry {
    pub fn attr(&self, ino: u64) -> FileAttr {
        FileAttr {
            ino,
            size: EmwinFS::INIT_SIZE,
            blocks: 1,
            atime: SystemTime::UNIX_EPOCH,
            mtime: SystemTime::UNIX_EPOCH,
            ctime: SystemTime::UNIX_EPOCH,
            crtime: SystemTime::UNIX_EPOCH,
            kind: FileType::RegularFile,
            perm: 0o220,
            nlink: 0,
            uid: self.uid,
            gid: self.gid,
            rdev: 0,
            blksize: EmwinFS::INIT_SIZE as u32,
            flags: 0,
        }
    }
}

impl Filesystem for EmwinFS {
    fn lookup(&mut self, _req: &fuser::Request<'_>, _parent: u64, name: &OsStr, reply: fuser::ReplyEntry) {
        for (ino, entry) in self.inodes.iter() {
            if name == entry.name {
                return reply.entry(&TTL, &entry.attr(*ino), 1);
            }
        }

        reply.error(2)
    }

    fn getattr(&mut self, _req: &fuser::Request<'_>, ino: u64, reply: fuser::ReplyAttr) {
        if ino == 1 {
            return reply.attr(&TTL, &Self::ROOT_ATTR)
        } else {
            return reply.error(2)
        }
    }

    fn create(
            &mut self,
            req: &fuser::Request<'_>,
            _parent: u64,
            name: &OsStr,
            _mode: u32,
            _umask: u32,
            _flags: i32,
            reply: fuser::ReplyCreate,
        ) {
        log::error!("create {}", name.to_string_lossy());
        let name = PathBuf::from(name);
        let file = EmwinFSEntry {
            bytes: Vec::with_capacity(Self::INIT_SIZE as usize),
            name,
            uid: req.uid(),
            gid: req.gid(),
        };
        let ino = self.ino;
        let attr = file.attr(ino);
        self.ino += 1;
        self.inodes.insert(ino, file);

        reply.created(&TTL, &attr, 0, 0, 0);
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
        log::error!("write to {}", ino);
        match self.inodes.get_mut(&ino) {
            Some(entry) => {
                /*if offset != entry.bytes.len().saturating_sub(1) as i64 {
                    log::error!("Failed to write to in-memory file: offset was not equal to last byte, got {}", offset);
                    return reply.written(0);
                }*/

                match entry.bytes.write(data) {
                    Err(e) => {
                        log::error!("Failed to write {} bytes to buffer: {}", data.len(), e);
                        return reply.error(5);
                    }
                    Ok(n) => {
                        reply.written(n as u32);
                    }
                }
            }
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
                self.rt.spawn(async move {
                    let text = match std::str::from_utf8(&entry.bytes) {
                        Ok(text) => text,
                        Err(e) => {
                            log::error!("Failed to convert written bytes to utf-8: {}", e);
                            return;
                        }
                    };
                    

                    log::error!("release {}: \n{}", ino, text);

                    crate::dispatch::emwin_dispatch(entry.name, text, ctx).await;
                });

                return reply.ok();
            }
            None => return reply.error(2),
        }
    }
}
