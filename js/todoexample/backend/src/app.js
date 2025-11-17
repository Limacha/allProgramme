const express = require('express');
const cors = require('cors'); // autorise les requete de l'exterieur
const session = require('express-session');

const authRoutes = require('./routes/authRoutes'); // routes d'authentification
const todoRoutes = require('./routes/todoRoutes'); // routes d'authentification

const frontendHost = process.env.host || '127.0.0.1';
const frontendPort = process.env.frontPort || 3000;
const frontendOrigin = `http://${frontendHost}:${frontendPort}`;

//#region init
const app = express();

app.use(cors({
    origin: frontendOrigin,
    credentials: true,
    methods: ['GET', 'POST', 'PUT', 'PATCH', 'DELETE', 'OPTIONS'],
    allowedHeaders: ['Content-Type']
}));
console.log(frontendOrigin);

app.use(session({
    secret: 'monSuperSecret',
    resave: false,
    saveUninitialized: false,
    cookie: { secure: false, sameSite: 'lax' }
}));

app.use(express.json());


//#endregion

//#region routes
app.use('/api/auth', authRoutes);
app.use('/api/todo', todoRoutes);
//#endregion

app.options('*', cors({
    origin: frontendOrigin,
    credentials: true,
    methods: ['GET', 'POST', 'PUT', 'PATCH', 'DELETE', 'OPTIONS'],
    allowedHeaders: ['Content-Type']
}));

module.exports = app;
