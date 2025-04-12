const { app, BrowserWindow } = require('electron');

const createWindow = () => {
    const win = new BrowserWindow({
        width: 800,
        height: 600,
        title: "akapp menu",
        icon: 'C:\Users\Nico\progra\js\akapp\assets\Arflaka.ico',
        autoHideMenuBar: false
    });

    win.loadFile('views/index.html');
};

app.whenReady().then(() => {
    createWindow()

    app.on('activate', () => {
        if (BrowserWindow.getAllWindows().length === 0) {
            createWindow()
        }
    })
});

app.on('window-all-closed', () => {
    if (process.platform !== 'darwin') {
        app.quit()
    }
});

module.exports = { app };