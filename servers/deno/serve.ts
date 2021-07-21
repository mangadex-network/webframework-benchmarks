import { readableStreamFromReader } from "https://deno.land/std/io/mod.ts";
const text = new Array(8 * 1024).fill('X').join('');

async function serveText(connection: any) {
    const response = new Response(text, { status: 200 });
    connection.respondWith(response);
}

async function serveImage(connection: any) {
    const file = await Deno.open('../../htdocs/sample.jpg');
    const stream = readableStreamFromReader(file);
    const response = new Response(stream, { status: 200 });
    await connection.respondWith(response);
    //file.close();
}

async function handle(connection: Deno.Conn) {
    for await (const hconnection of Deno.serveHttp(connection)) {
        if(hconnection.request.url.includes('/sample.txt')) {
            serveText(hconnection);
        }
        if(hconnection.request.url.includes('/sample.jpg')) {
            serveImage(hconnection);
        }
    }
}

const server = Deno.listen({ port: 8080/*, alpnProtocols: [ 'h2' ]*/ });
console.log('Server started on http://127.0.0.1:8080');

for await (const connection of server) {
    handle(connection);
}