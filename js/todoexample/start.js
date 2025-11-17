const { spawn } = require('child_process'); //sous-process
const path = require('path');

const args = process.argv.slice(2);

const host = args[0] || '127.0.0.1';
const frontendPort = args[1] || 3000;
const apiPort = args[2] || 5000;

// chemins vers les dossiers
const apiPath = path.join(__dirname, 'backend');
const frontendPath = path.join(__dirname, 'frontend');

//#region lancement des process
// lance le backend
const backend = spawn(
    'npm', // commande
    ['start'], // arguments
    {
        cwd: apiPath, // dossier racine
        stdio: 'inherit', // logs dans ce terminal
        env: { ...process.env, host: host, frontPort: frontendPort, apiPort: apiPort } //copie de var d'env et set les variable importantes
    }
);

// lance le frontend
const frontend = spawn(
    'npm', // commande
    ['start'], // arguments
    {
        cwd: frontendPath, // dossier racine
        stdio: 'ignore', //lods dans ce terminal
        env: { ...process.env, host: host, frontPort: frontendPort, apiPort: apiPort, REACT_APP_API_URL: `http://${host}:${apiPort}` } //copie de var d'env et set les variable importantes
    }
);
//#endregion

//#region fermeture des process
let backendRunning = true;
let frontendRunning = true;

backend.once('close', (code) => {
    console.log(`Backend arrêté avec le code ${code}`);
    backendRunning = false;
    if (frontendRunning) {
        frontend.kill();
    }
});

frontend.once('close', (code) => {
    console.log(`Frontend arrêté avec le code ${code}`);
    frontendRunning = false;
    if (backendRunning) {
        backend.kill();
    }
});
//#endregion