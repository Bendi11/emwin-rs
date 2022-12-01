'use strict';

fetch('/nav.html')
    .then(resp => { return resp.text(); })
    .then(data => { document.querySelector('header').innerHTML = data; })
