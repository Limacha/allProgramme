const app = require('./app');


const host = (process.env.host != null) ? process.env.host : "127.0.0.1";
//const frontendPort = (process.env.frontendPort != null) ? process.env.frontendPort : 3000;
const apiPort = (process.env.apiPort != null) ? process.env.apiPort : 5000;

// Lance le serveur
app.listen(apiPort, host, () => {
    console.log(`Server running on http://${host}:${apiPort}`);
});
