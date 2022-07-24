import { serve, file } from 'bun';

const text = new Array(8 * 1024).fill('X').join('');

function handle(request) {
    if(request.url.includes('/sample.txt')) {
        return new Response(text)
    }
    if(request.url.includes('/sample.jpg')) {
        return new Response(file('../../htdocs/sample.jpg'));
    }
}

console.log('Server started on http://127.0.0.1:44300');
const server = serve({
    fetch(request) {
        return handle(request);
    },
    development: false,
    certFile: '../../localhost.crt',
    keyFile: '../../localhost.key',
    port: 44300,
});