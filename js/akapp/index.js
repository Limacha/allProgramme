//#region importe
const { exec } = require('child_process');
const { app, BrowserWindow } = require('electron');

const constante = require(__dirname + "/assets/scripts/const.js");
const package = require(__dirname + "/package.json");
/*
let argv = new Map();
// print process.argv
process.argv.forEach(function (val, index, array) {
    console.log(index + ': ' + val);
    let ar = val.split(':');
    if (ar.length === 2) {
        argv.set(ar[0], ar[1]);
    }
});
*/
const display = require(__dirname + "/assets/scripts/display.js");

//#endregion

exec(`title ${package.name} ${package.version}`);


constante.db.connect(function (err) { if (err) throw err; console.log("Connecte a la base de donnees MySQL!"); });
constante.db.end(() => {
    console.log("connection end");
});