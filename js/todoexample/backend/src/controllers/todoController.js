const Todo = require('../models/todoModel');

async function update(req, res) {
    console.log("todo update", req.session.userId);
    if (!req.session.userId) {
        return res.status(401).json({ message: 'Non connecté' });
    }

    const { todos } = req.body;

    Todo.setUserTodo(req.session.userId, todos);

    return res.json({ message: 'update effectuer !' });
}

function list(req, res) {
    console.log("todo list");
    if (!req.session.userId) {
        return res.status(401).json({ message: 'Non connecté' });
    }
    console.log(req.session.userId);
    const tasks = Todo.getUserTodo(req.session.userId);

    return res.json({
        message: 'Profil accessible',
        tasks: tasks
    });
}


module.exports = { update, list };