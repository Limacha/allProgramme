const { app, BrowserWindow } = require('electron');
const mysql = require('mysql');

const db = mysql.createConnection({ host: "localhost", user: "root", password: "root", database: "yoch" });
db.connect(function (err) { if (err) throw err; console.log("Connecte a la base de donnees MySQL!"); });
db.end(() => {
    console.log("connection end");
});


const createWindow = () => {
    const win = new BrowserWindow({
        width: 800,
        height: 600,
        title: "tchat",
        autoHideMenuBar: false
    });

    win.loadFile('assets/index.html');
}

app.whenReady().then(() => {
    createWindow()

    app.on('activate', () => {
        if (BrowserWindow.getAllWindows().length === 0) {
            createWindow()
        }
    })
})

app.on('window-all-closed', () => {
    if (process.platform !== 'darwin') {
        app.quit()
    }
})