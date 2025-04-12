const mysql = require('mysql');

const consoleColor = {
    reset: "\x1b[0m",
    red: "\x1b[31m",
    green: "\x1b[32m",
    yellow: "\x1b[33m",
    blue: "\x1b[34m",
    magenta: "\x1b[35m",
    bold: "\x1b[1m",
    cyan: "\x1b[36m",
    white: "\x1b[37m",
    orange: "\u001b[38;2;253;182;0m",
    clearScreen: "\x1b[2J",
    moveCursorHome: "\x1b[0;0H",
    jsp: "\x1b[34;219m",
    itallic: "\x1b[3m"
};

const dbConnection = {
    host: "127.0.0.1",
    user: "root",
    password: "root"
};
const db = mysql.createConnection(dbConnection);

module.exports = { consoleColor, dbConnection, db };