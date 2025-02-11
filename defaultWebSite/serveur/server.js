const http = require("http");
const url = require("url");

const { ReadHtmlFile } = require("../fonction.js");


function LaunchServ(port, host) {
    var server = http.createServer();

    server.listen(port, host, () => {
        console.log(`HTTP Server running on http://${host}:${port}`);
    });

    server.on('error', (err) => {
        console.log("[server] [ERROR]" + err)
        server.close();
    });

    server.on('request', async (req, res) => {
        console.log("----------new request----------");
        //console.log(req);
        let parsedUrl = url.parse(req.url, true);
        console.log(parsedUrl);
        try {
            let baseHtml = await ReadHtmlFile("./client/views/base.html");
            let headerHtml = await ReadHtmlFile("./client/views/header.html");
            let mainHtml = "";
            try {
                mainHtml = await ReadHtmlFile("./client/views/" + parsedUrl.pathname + ".html");
            } catch {
                mainHtml = await ReadHtmlFile("./client/views/404.html");
            }
            let footerHtml = await ReadHtmlFile("./client/views/footer.html");

            baseHtml = baseHtml.replace("$header$", headerHtml);
            baseHtml = baseHtml.replace("$main$", mainHtml);
            baseHtml = baseHtml.replace("$footer$", footerHtml);

            res.writeHead(200);
            res.write(baseHtml);
        } catch (err) {
            console.log(err);
            res.writeHead(404);
            res.write(err);
        } finally {
            res.end();
        }
    });

    return server;
}

/**
 * retire tous les listeners lier au nom
 * @param {http.Server} server 
 * @param {String} name 
 */
function RemoveListener(server, name) {
    server.removeAllListeners(name);
}

module.exports = { LaunchServ, RemoveListener };