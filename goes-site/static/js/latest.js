'use strict';

function dt(str) {
    const utc_epoch = Date.parse(str);
    const time = new Date();
    time.setTime(utc_epoch);
    return time.toLocaleString();
}

document.addEventListener("DOMContentLoaded", function(){
    fetch('/search/img/multi', {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            latest: null,
            acronym: 'CLOUD_MOISTURE_IMAGERY',
            channel: 'FULL_COLOR',
            sector: 'FULL_DISK',
            limit: 1,
            page: 0,
            rets: ["path", "datetime"]
        })
    })
        .catch(console.log)
        .then(resp => resp.json())
        .then(data => {
            document
                .getElementById('fd_fc_img')
                .setAttribute('src', `/assets/${data[0].path}`);

            document.getElementById('fd_fc_last_update').innerHTML = dt(data[0].datetime);
        });

    fetch(
        '/search/img/multi', {
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                latest: null,
                sector: 'FULL_DISK',
                limit: 1,
                page: 0,
                rets: ["path", "datetime"]
            })
        }
    )
        .catch(console.log)
        .then(resp => resp.json())
        .then(data => {
            document
                .getElementById('fd_img')
                .setAttribute('src', `/assets/${data[0].path}`);

            document.getElementById('fd_last_update').innerHTML = dt(data[0].datetime);
        });
});

