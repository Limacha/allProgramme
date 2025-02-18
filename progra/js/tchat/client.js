const net = require('net');
const readline = require('readline');
const { exec } = require('child_process');
const path = require('path');
const constante = require(path.join(__dirname + "/script/const.js"));


const client = net.createConnection({ host: '51.75.25.2', port: 2012 }, () => {
    console.log('Connect success')
});

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: true
});
exec('title Elios Chat 1.1.2');
let lineCount = 0;

process.on('SIGINT', () => {
    client.end();
    process.exit();
});

client.on('data', (data) => {
    const message = data.toString().trim();

    if (message.endsWith('Login:')) {
        rl.question(`${message} `, (login) => {
            client.write(login);
        });
    } else if (message.endsWith('Mot de passe:')) {
        rl.question(`${message} `, (password) => {
            client.write(password);
        });
    } else {
        process.stdout.write('\n');
        console.log(message);
        process.stdout.write('> ');
    }
});

client.on('end', () => {
    console.log('Déconnecté du serveur');
    console.log(`${itallic}Pour tout bug ajoutez ${green}@liveweeeb ${white}ou ${green}@test_befaci.coolate ${white}sur discord.`)
    console.log('Appuyez sur une touche pour réessayer...');

    process.stdin.once('keypress', () => {
        console.log('Tentative de reconnexion...');

        client.connect({ host: '51.75.25.2', port: 2012 });
    });
});

client.on('error', (err) => {
    console.error(`Erreur de connexion`);

    console.log(`${itallic}Pour tout bug ajoutez ${green}@liveweeeb ${white}ou ${green}@test_befaci.coolate ${white}sur discord.`)
    console.log('Appuyez sur une touche pour réessayer...');

    process.stdin.once('keypress', () => {
        console.log('Tentative de reconnexion...');

        client.connect({ host: '51.75.25.2', port: 2012 });
    });
});

rl.on('line', (input) => {
    client.write(input);
});