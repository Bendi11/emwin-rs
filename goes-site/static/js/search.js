'use strict';

const query = {
    limit: 10,
    page: 0,
    rets: ["path", "datetime"]
};

function update(form) {
    delete query.latest;
    query.acronym = form['acronym-select'].value;
    query.channel = form['channel-select'].value;
    query.satellite = form['satellite-select'].value;
    query.sector = form['sector-select'].value;
    query.from = form['from-input'].value;
    query.to = form['to-input'].value;
    query.from = (query.from.length === 0) ? null : query.from;
    query.to = (query.to.length === 0) ? null : query.to;

    if(query.from === null && query.to === null) {
        delete query.from;
        delete query.to;
        query.latest = null;
    }
}

document.addEventListener("DOMContentLoaded", function() {
    const acryonym_sel = document.getElementById('acronym-select');
    const channel_sel = document.getElementById('channel-select');
    const form = document.getElementById('search-form');
    const copy_btn = document.getElementById('copy-btn');
    const results = document.getElementById('search-results');

    acryonym_sel.addEventListener('change', () => {
        if(["L1b", "CLOUD_MOISTURE_IMAGERY", "DERIVED_MOTION_WIND"].includes(acryonym_sel.value)) {
            channel_sel.value = "FULL_COLOR_LINES";
            channel_sel.disabled = false;
        } else {
            channel_sel.value = null;
            channel_sel.disabled = true;
        }
    });

    function imgelem(data) {
        const col = document.createElement('div');
        col.setAttribute('class', 'col-sm-6');

        const center = document.createElement('center');

        const img = document.createElement('img');
        img.setAttribute('class', 'img-responsive');
        img.setAttribute('src', data.path);

        center.appendChild(img);
        col.appendChild(center);
        return col;
    }

    form.addEventListener('submit', (e) => {
        e.preventDefault();
        update(form.elements);
        fetch('/search/img/multi', {
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(query),
        })
            .catch(console.error)
            .then(resp => resp.json())
            .then(data => {
                for(let i = 0; i < data.length; i += 1) {
                    let elem;
                    if(i + 2 < data.length) {
                        const elem = document.createElement('div');
                        elem.setAttribute('class', 'row');

                        elem.appendChild(imgelem(data[i]));
                        i += 1;
                        elem.appendChild(imgelem(data[i]));
                    } else {
                        elem = imgelem(data[i]);
                    }

                    results.appendChild(elem);
                }
            });

    });

    copy_btn.addEventListener('click', () => {
        update(form.elements);

        const {limit: _limit, page: _page, rets: _rets, ...single} = query;

        const textarea = document.createElement("textarea");
        textarea.textContent = `${location.origin}/search/img/single/${btoa(JSON.stringify(single))}`;
        textarea.style.position = "fixed";
        document.body.appendChild(textarea);
        textarea.select();
        try {
            document.execCommand("copy");
        }
        catch (ex) {
            console.warn("Copy to clipboard failed.", ex);
            return prompt("Copy to clipboard: Ctrl+C, Enter", text);
        }
        finally {
            document.body.removeChild(textarea);
        }
    });
})

