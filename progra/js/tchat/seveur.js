const net = require('net');
const fs = require('fs');
const path = require('path');
const constante = require(path.join(__dirname + "/script/const.js"));

const clients = [];
const accountsFile = path.join(__dirname, 'pass.json');

let accounts = {};

function loadAccountsFile() {
    if (fs.existsSync(accountsFile)) {
        try {
            accounts = JSON.parse(fs.readFileSync(accountsFile, 'utf8'));
            console.log('Les comptes ont été rechargés depuis pass.json');
        } catch (err) {
            console.error(`Erreur lors du chargement de pass.json : ${err.message}`);
        }
    }
}

loadAccountsFile();

const userSessions = new Map();

const server = net.createServer((socket) => {
    let pseudo = '';
    let step = 'login';
    let loginAttempt = '';

    socket.write(clearScreen);
    socket.write(moveCursorHome);
    socket.write(`${magenta}Console: ${PENCIL}Bienvenue dans le chat! Veuillez vous connecter.${reset}(${PENCIL}${itallic}pour creer un compte ajoutez ${reset}${green}${bold}@liveweeeb ${reset}${PENCIL}${itallic}sur discord${reset})${reset}\n`);

    function requestLogin() {
        socket.write(`${magenta}Console: ${cyan}Login: ${reset}`);
        step = 'login';
    }

    function requestPassword() {
        socket.write(`${magenta}Console: ${cyan}Mot de passe: ${reset}`);
        step = 'password';
        isPasswordEntry = true;
    }

    function authenticate(login, password) {
        if (accounts[login]) {
            if (accounts[login] === password) {
                pseudo = login;
                if (userSessions.has(pseudo)) {
                    const previousSocket = userSessions.get(pseudo);
                    previousSocket.write(`${magenta}Console: ${cyan}Vous avez été déconnecté car une nouvelle connexion a été détectée.${reset}\n`);
                    previousSocket.end();
                }
                userSessions.set(pseudo, socket);
                clients.push({ socket, pseudo });
                socket.write(`${magenta}Console: ${cyan}Vous êtes maintenant connecté sous le pseudo : ${jsp}${pseudo}${reset}\n`);
                step = "authenticated";
                broadcast(`${red}${pseudo} ${yellow}a rejoint le chat!${reset}`, socket);
            } else {
                socket.write(`${magenta}Console: ${cyan}Mot de passe incorrect. Veuillez réessayer.${reset}\n`);
                requestPassword();
            }
        } else {
            socket.write(`${magenta}Console: ${cyan}Identifiant invalide. Veuillez réessayer.${reset}\n`);
            requestLogin();
        }
    }

    socket.on('data', (data) => {
        const message = data.toString().trim();

        if (step === 'login') {
            loginAttempt = message;
            if (accounts[loginAttempt]) {
                requestPassword();
            } else {
                socket.write(`${magenta}Console: ${cyan}Identifiant invalide. Veuillez réessayer.${reset}\n`);
                requestLogin();
            }
        } else if (step === 'password') {
            if (isPasswordEntry) {
                passwordAttempt += input; // Store the real password
                socket.write('*'.repeat(input.length)); // Print stars instead of the actual input
                authenticate(loginAttempt, passwordAttempt);
                isPasswordEntry = false;
            }


            //    authenticate(loginAttempt, message);
        } else if (step === 'authenticated') {
            if (!pseudo) {
                socket.write(`${magenta}Console: ${cyan}Erreur d'authentification. Reconnectez-vous.\n${reset}`);
                requestLogin();
            } else {
                if (message === '/liste') {
                    const userList = clients.map(client => client.pseudo).join(', ');
                    const userCount = clients.length;
                    socket.write(`${magenta}Console: ${cyan}Utilisateurs connectés (${userCount}): ${reset}${userList}\n`);
                } else if (message === '/whoami') {
                    socket.write(`${magenta}Console: ${cyan}Votre pseudo actuel est ${jsp}${pseudo}${reset}\n`);
                } else if (message === '/deco') {
                    socket.write(clearScreen);
                    socket.write(moveCursorHome);
                    socket.write(`${magenta}Console: ${cyan}Déconnexion de ${jsp}${pseudo}${reset}\n`);
                    handleDisconnect(socket, pseudo);
                    requestLogin();
                } else if (!message) {
                    socket.write(`Tu ne peux pas envoyer des message vide`);
                } else if (message === '/about') {
                    socket.write(` ${red}A propos de ce chat \n${yellow}Développer par ${cyan}${bold}liveweeeb ${reset}${yellow}& ${cyan}${bold}befaci02 ${yellow}\n${reset}${red}Version ${bold}1.0${reset}\n \n${itallic}Pour tout bug ajoutez ${green}@liveweeeb ${white}ou ${green}@test_befaci.coolate ${white}sur discord. `);
                } else {
                    broadcast(`${blue}${pseudo}: ${reset}${message}`, socket);
                }
            }
        }
    });

    socket.on('end', () => {
        handleDisconnect(socket, pseudo);
    });

    socket.on('error', (err) => {
        console.error(`Erreur de socket: ${err.message}`);
        handleDisconnect(socket, pseudo);
    });

    requestLogin();
});

function handleDisconnect(socket, pseudo) {
    const index = clients.findIndex(client => client.socket === socket);
    if (index !== -1) {
        clients.splice(index, 1);
    }
    if (pseudo && userSessions.has(pseudo)) {
        userSessions.delete(pseudo);
    }
    broadcast(`${red}${pseudo} ${yellow}a quitté le chat.${reset}`, socket);
}

function broadcast(msg, senderSocket) {
    for (let client of clients) {
        if (client.socket !== senderSocket) {
            client.socket.write(msg);
        }
    }
}

setInterval(() => {
    loadAccountsFile();
}, 10000);

server.listen(3000, () => {
    console.log('Serveur de chat lancé sur le port 3000');
});
