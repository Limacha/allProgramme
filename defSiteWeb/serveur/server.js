const http = require("http");
const url = require("url");

const { ReadUTF8File, VerifJavaScriptError } = require("../fonction.js");

/**
 * fait un serveur qui tourne sur host:port avec
 * ./client/views/base.html
 * ./client/views/header.html
 * ./client/views/404.html
 * ./client/views/footer.html
 * et ReadUTF8File; verifJavaScriptError dans "../fonction.js"
 * requis par rapport a l'endroit ou il est call
 * @param {number} port le port sur le quel ecouter
 * @param {string} host l'ip sur la quel ecouter
 * @returns le serveur generer avec on(error) et on(request)
 */
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
        let parsedUrl = url.parse(req.url, true);
        console.log(parsedUrl);
        try {
            //lecture de tout les fichier requis
            let baseHtml = await ReadUTF8File("./client/views/base.html");
            let headerHtml = await ReadUTF8File("./client/views/header.html");
            let mainHtml = "";
            try {
                mainHtml = await ReadUTF8File("./client/views/" + parsedUrl.pathname + ".html");
            } catch {
                mainHtml = await ReadUTF8File("./client/views/404.html");
            }

            let footerHtml = await ReadUTF8File("./client/views/footer.html");

            let script = "";
            try {
                script = await ReadUTF8File("./client/js/" + parsedUrl.pathname + ".js");
                const error = VerifJavaScriptError(script);
                if (error) {
                    script = "console.log('script as an error: " + error + "');";
                }
            } catch {
                script = "console.log('script not found at: ./client/js/" + parsedUrl.pathname + ".js');";
            }

            let css = "";
            try {
                css = await ReadUTF8File("./client/css/" + parsedUrl.pathname + ".css");
            } catch {
                try {
                    css = await ReadUTF8File("./client/css/404.css");
                } catch {
                    css = "";
                }
            }

            //remplacement dans le fichier envoie au serveur
            baseHtml = baseHtml.replace("$header$", headerHtml);
            baseHtml = baseHtml.replace("$main$", mainHtml);
            baseHtml = baseHtml.replace("$footer$", footerHtml);

            baseHtml = baseHtml.replace("$script$", script);

            // /* $css$ */ car sinon erreur qui sa fiche
            baseHtml = baseHtml.replace("/* $css$ */", css);

            //evoie des fichier
            res.writeHead(200);
            res.write(baseHtml);
        } catch (err) {
            //affiche l'erreur dans la console du serv et l'envoie au serv
            console.log(err);
            res.writeHead(404);
            res.write(err);
        } finally {
            //dans les deux cas dis que la reponse est fini
            res.end();
        }
        if (!res.closed) {
            res.end();
        }
    });
    //renvoie le serveur pour l'utiliser autre part
    return server;
}


/**
 * retire tous les listeners lier au nom
 * @param {*} server le server sur le quel retirer les listeners
 * @param {String} name le nom du listeners
 */
function RemoveListener(server, name) {
    server.removeAllListeners(name);
}

/**
 * simule une erreur sur le server
 * @param {http.Server} server le server sur le quel retirer les listeners
 * @param {Number} delay entier qui sera le temp a attendre
 */
function SimulateError(server, delay) {
    if (Number.isInteger(delay)) {
        setTimeout(() => {
            server.emit('error', new Error('Erreur simulée pour tester'));
        }, delay);
    }
}

/**
 * ferme le server http et toute les connections lier
 * @param {http.Server} server 
 */
function CloseServer(server) {
    server.close();
    server.closeAllConnections();
}

module.exports = { LaunchServ, RemoveListener, SimulateError, CloseServer };