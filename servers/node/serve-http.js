const fs = require('fs');
const http = require('http');
const cluster = require('cluster');

const workers = 4;
const text = new Array(8 * 1024).fill('X').join('');

async function serveText(response) {
    response.writeHead(200);
    response.end(text);
}

async function serveImage(response) {
    var stream = fs.createReadStream('../../htdocs/sample.jpg');
    stream.on('open', () => stream.pipe(response));
    //stream.on('error', error => response.end(error));
    //stream.on('end', () => stream.close());
}

async function handle(request, response) {
    if(request.url.includes('/sample.txt')) {
        serveText(response);
    }
    if(request.url.includes('/sample.jpg')) {
        serveImage(response);
    }
}

if(cluster.isMaster) {
    for (let i = 0; i < workers; i++) {
        cluster.fork();
    }
    // Re-spawn worker on death
    cluster.on('exit', (worker, code, signal) => {
        cluster.fork();
    });
} else {
    http.createServer(handle).listen(8080);
    console.log('Worker started on http://127.0.0.1:8080');
}
