const path = require('path');
const readline = require('readline');
const { exec } = require('child_process');
const constante = require(path.join(__dirname + "/script/const.js"));
console.log(constante.consoleColor.orange + constante + constante.consoleColor.reset);

exec('title test Chat 0.0.1');

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: true
});


rl.question('What is your age? ', (age) => {
    console.log('Your age is: ' + age);
});