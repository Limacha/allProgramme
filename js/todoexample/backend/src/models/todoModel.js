const fs = require('fs');
const path = require('path');

//chemin de la bdd
const filePath = path.join(__dirname, '../../database/todo.json');

/**
 * lecture de la bdd et obtient tout les taches de l'utilisateur
 * @param {string} id l'id de l'utilisateur
 * @returns renvoie les taches trouver de la bdd
 */
function getUserTodo(id) {
    const data = JSON.parse(fs.readFileSync(filePath, 'utf-8'));
    //console.log("model:", id, data, data[id]);
    return data[id] || [];
}


/**
 * met a jour les taches d'un utilisateur
 * @param {string} id l'id de l'utilisateur
 * @param {{name: string, finish: boolean}[]} tasks Les tâches de l’utilisateur
 */
async function setUserTodo(id, tasks) {
    const data = JSON.parse(fs.readFileSync(filePath, 'utf-8'));

    if (!Array.isArray(tasks)) throw new TypeError("tasks doit etre un tableau");

    // Création automatique du profil utilisateur si inexistant
    if (!data[id]) data[id] = [];

    // Mise à jour des tâches
    data[id] = tasks;

    //ecriture de la bdd
    fs.writeFileSync(filePath, JSON.stringify(data));

    console.log(`task update ${id} :`, tasks);
}


module.exports = { getUserTodo, setUserTodo };