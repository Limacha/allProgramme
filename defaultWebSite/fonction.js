const fs = require('fs');
/**
 * lis un fichier avec le format utf-8
 * @param {String} url 
 * @returns les donner du fichier
 */
function ReadHtmlFile(url) {
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

module.exports = { ReadHtmlFile };