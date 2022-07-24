const fs = require('fs');
const { App } = require('uwebsockets');
const cluster = require('cluster');

const workers = 4;
const text = new Array(8 * 1024).fill('X').join('');

async function serveText(response, request) {
    response
        //.writeStatus('200 OK')
        //.writeHeader('Content-Type', 'text/plain')
        .end(text);
}

async function serveImage(response, request) {
    
    const data = fs.readFileSync('../../htdocs/sample.jpg');
    response.end(data);
    

    // https://github.com/sifrr/sifrr/blob/master/packages/server/sifrr-server/src/server/sendfile.ts#L129
    /*
    response.onAborted(() => response.aborted = true);
    const stream = fs.createReadStream('../../htdocs/sample.jpg');
    await new Promise((resolve, reject) => {
        //stream.on('open', () => {});
        stream.on('error', reject);
        // chunk.buffer.slice(chunk.byteOffset, chunk.byteOffset + chunk.byteLength);
        stream.on('data', chunk => response.write(chunk.buffer.slice(chunk.byteOffset, chunk.byteOffset + chunk.byteLength)));
        stream.on('end', resolve);
    });
    //stream.close();
    response.end();
    */

    /*
    response.onAborted(() => response.aborted = true);
    response.cork(() => {
        response.onAborted(() => response.aborted = true);
        const stream = fs.createReadStream('../../htdocs/sample.jpg');
        stream.on('open', () => {});
        stream.on('data', chunk => response.write(chunk));
        stream.on('end', () => response.end());
    });
    */
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
    App() // SSLApp()
        .get('/sample.txt', serveText)
        .get('/sample.jpg', serveImage)
        .listen(8080, token => console.log('Worker started on http://127.0.0.1:8080'));
}
