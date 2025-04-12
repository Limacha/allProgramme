const path = require('path');
const readline = require('readline');
const { exec } = require('child_process');
const constante = require(__dirname + "/assets/scripts/const.js");
const package = require(__dirname + "/package.json");
const user = require(__dirname + "/assets/json/user.json");
const mysql = require('mysql');

const db = mysql.createConnection({ host: "localhost", user: "root", password: "root", database: "yoch" });
db.connect(function (err) { if (err) throw err; console.log("Connecte a la base de donnees MySQL!"); });
db.end(() => {
    console.log("connection end");
});

exec(`title ${package.name} ${package.version}`);

console.log(user);

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: true
});


rl.question('What is your pseudo? ', (pseudo) => {
    console.log('Your are: ' + pseudo);
    rl.close();
});

