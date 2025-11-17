const User = require('../models/userModel');

// Contrôleur de login
async function login(req, res) {
    console.log("login", req.body);
    const { name, password } = req.body; // recup des datas

    const user = await User.findUserByName(name); // obtention de l'user

    // identifiant incorect
    if (!user || user.password !== password) {
        return res.status(401).json({ message: 'nom ou mot de passe incorrect' });
    }

    // OK
    req.session.userId = user.id;

    return res.json({ message: 'Login réussi !' });
}

function profile(req, res) {
    if (!req.session.userId) {
        return res.status(401).json({ message: 'Non connecté' });
    }

    return res.json({
        message: 'Profil accessible',
        userId: req.session.userId
    });
}


module.exports = { login, profile };