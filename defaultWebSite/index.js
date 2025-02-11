//const http = require("http");
require("./serveur/server.js");
const pack = require("./package.json");
const { LaunchServ, SetOn } = require("./serveur/server");

const args = process.argv.slice(2); // Get all arguments after 'node index.js'

const host = (args[0] != null) ? args[0] : "127.0.0.1";
const port = (args[1] != null) ? args[1] : 3000;

let server = LaunchServ(port, host);

/*setTimeout(() => {
    server.emit('error', new Error('Erreur simulée pour tester'));
}, 2000);*/

//errHandler
process.on("unhandledRejection", errorHandler);
process.on("uncaughtException", errorHandler);

async function errorHandler(error) {
    console.error("[ERROR] :", error);
    server.close();
}