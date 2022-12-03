'use strict';

const query = {
    limit: 10,
    page: 0,
    rets: ["path", "datetime"]
};

function update(form) {
    query.acronym = form['acronym-select'];
    query.channel = form['channel-select'];
    query.satellite = form['satellite-select'];
    query.sector = form['sector-select'];
    query.from = form['from-input'];
    query.to = form['to-input'];
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

    form.addEventListener('submit', () => {
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
        navigator
            .clipboard
            .write(`${location.origin}/search/img/single/${btoa(JSON.stringify(query))}`)
            .then(
                () => {},
                (e) => alert(`Failed to copy to clipboard: ${e}`)
            );
    });
})

