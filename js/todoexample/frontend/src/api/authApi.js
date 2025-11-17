/**
 * envoie la demande de co et obtient la reponse
 * @param {string} name nom de l'user
 * @param {string} password mot de passe de l'user
 * @returns la reponse du serv
 */
export async function loginUser(name, password) {
    const response = await fetch('http://localhost:5000/api/auth/login', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name, password })
    });

    return response.json(); // Retourne la réponse du backend
}


export async function getProfile() {
    const response = await fetch('http://localhost:5000/api/auth/profile', {
        method: 'GET',
        credentials: 'include'
    });

    return response.json();
}