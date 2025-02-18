//#region import
const { ReadUTF8File, ReadNoFormatFile, VerifJavaScriptError, ParseUrl } = require("./fonction.js");
const pack = require("./package.json");
const { ApiController } = require("./serveur/controller/apiController.js");
const { ErrorController } = require("./serveur/controller/errorController.js");
const { GlobalController } = require("./serveur/controller/globalController.js");
const { LaunchServ, RemoveListener, CloseServer } = require("./serveur/server");

const args = process.argv.slice(2); // Get all arguments after 'node index.js'

const host = (args[0] != null) ? args[0] : "127.0.0.1";
const port = (args[1] != null) ? args[1] : 3000;
//#endregion

//lance le serveur
const server = LaunchServ(port, host);

//#region erreur
//errHandler
process.on("unhandledRejection", ErrorHandler);
process.on("uncaughtException", ErrorHandler);

/**
 * gere se qui se passe quand une erreur est generer
 * @param {*} error l'erreur en question
 * @param {*} obj un obj a afficher en console
 * @param {*} server un server a fermer
 */
async function ErrorHandler(error, obj, server) {
    console.log("\x1b[41m");
    console.error("[ERROR] :" + error);
    console.log(obj);
    //console.log("\x1b[31m");
    CloseServer(server);
    console.error("[ERROR] la gestion de l'errer est fini\x1b[0m");
}

//#endregion