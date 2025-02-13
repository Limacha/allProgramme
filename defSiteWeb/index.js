//#region import
const { ReadUTF8File, ReadNoFormatFile, isJavaScriptValid } = require("./fonction.js");
const pack = require("./package.json");
const { LaunchServ, RemoveListener } = require("./serveur/server");

const args = process.argv.slice(2); // Get all arguments after 'node index.js'

const host = (args[0] != null) ? args[0] : "127.0.0.1";
const port = (args[1] != null) ? args[1] : 3000;
//#endregion

//lance le serveur
const server = LaunchServ(port, host);


//#region erreur
//errHandler
process.on("unhandledRejection", errorHandler);
process.on("uncaughtException", errorHandler);

/**
 * gere se qui se passe quand une erreur est generer
 * @param {*} error l'erreur en question
 */
async function errorHandler(error) {
    console.error("[ERROR] :", error);
    //ferme le serveur si existe
    if (server) {
        server.close();
    }
}

//#endregion