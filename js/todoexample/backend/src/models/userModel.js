const fs = require('fs');
const path = require('path');

//chemin de la bdd
const filePath = path.join(__dirname, '../../database/users.json');

/**
 * lecture de la bdd et obtient tout les users dispo
 * @returns renvoie les utilisateurs de la bdd
 */
function getUsers() {
    const data = fs.readFileSync(filePath);
    return JSON.parse(data);
}

/**
 * obtient un utilisateur depuis sont id
 * @param {string} id l'id de l'utilisateur
 * @returns renvoie l'utilisateur trouver
 */
async function findUserById(id) {
    const users = getUsers();
    return users[id];
}

/**
 * obtient un utilisateur depuis sont nom
 * @param {string} name nom de l'utilisateur
 * @returns renvoie l'utilisateur trouver
 */
async function findUserByName(name) {
    const users = getUsers();
    for (const userId of Object.keys(users)) {
        const user = users[userId];
        if (user.name === name) {
            user.id = userId;
            console.log(user);
            return user;
        }
    }
    return null; // pas trouvé
}


module.exports = { findUserById, findUserByName };