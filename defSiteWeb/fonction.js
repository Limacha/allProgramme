const fs = require('fs');
const vm = require('vm');

/**
 * lis un fichier avec le format utf-8
 * @param {String} url 
 * @returns les donner du fichier
 */
function ReadUTF8File(url) {
    return new Promise((resolve, reject) => {
        fs.readFile(url, "utf-8", (err, data) => {
            if (err) {
                reject("Document introuvable: " + url);
            } else {
                resolve(data);
            }
        });
    });
}

/**
 * lis un fichier sans fomat
 * @param {String} url 
 * @returns les donner du fichier
 */
function ReadNoFormatFile(url) {
    return new Promise((resolve, reject) => {
        fs.readFile(url, (err, data) => {
            if (err) {
                reject("Document introuvable: " + url);
            } else {
                resolve(data);
            }
        });
    });
}

/**
 * verifier si un script contient une erreur
 * @param {string} code le code a verifier
 * @returns l'erreur si il y en a une
 */
function verifJavaScriptError(code) {
    try {
        // Vérifier si c'est du JavaScript valide en compilant le code
        new vm.Script(code);

        // Si aucune erreur n'est levée, on peut tester l'exécution dans une sandbox
        const sandbox = {};
        vm.createContext(sandbox); // Crée un contexte isolé
        vm.runInContext(code, sandbox); // Exécute le code dans le contexte sandboxé
    } catch (error) {
        return error;
    }
}

module.exports = { ReadUTF8File, ReadNoFormatFile, verifJavaScriptError };