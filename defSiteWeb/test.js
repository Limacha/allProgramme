// index.js
const args = process.argv.slice(2); // Get all arguments after 'node index.js'
const variable = args[0]; // Get the first argument
console.log('Variable:', variable);
/* fonctione a ajouter

const http = require('http');
const url = require('url');

const server = http.createServer((req, res) => {
    // Parse the URL to get the query params if needed
    const queryParams = url.parse(req.url, true).query;

    // Check if the request is a GET to display cookies
    if (req.method === 'GET' && req.url === '/check-cookie') {
        // Get the cookies from the request headers
        const cookies = parseCookies(req.headers.cookie);

        res.writeHead(200, { 'Content-Type': 'text/plain' });
        res.end(`Cookies received: ${JSON.stringify(cookies)}`);
    }

    // Check if the request is a POST to set cookies
    if (req.url === '/set-cookie') {
        // Set a cookie with HttpOnly flag
        res.writeHead(200, {
            'Content-Type': 'text/plain',
            'Set-Cookie': 'username=JohnDoe; HttpOnly; Max-Age=3600', // 1 hour expiration
        });
        res.end('Cookie has been set');
    }
});

// Helper function to parse cookies from the request
const parseCookies = (cookieHeader) => {
    const cookies = {};
    if (cookieHeader) {
        cookieHeader.split(';').forEach(cookie => {
            const [name, value] = cookie.trim().split('=');
            cookies[name] = value;
        });
    }
    return cookies;
};

server.listen(3000, () => {
    console.log('Server running on http://localhost:3000');
});
*/