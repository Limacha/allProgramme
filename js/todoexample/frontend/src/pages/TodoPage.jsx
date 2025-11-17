import React, { useEffect, useState } from 'react';
import TodoForm from '../components/TodoForm';

function TodoPage() {
    const [id, setId] = useState(null);
    const [todos, setTodos] = useState([]);
    const [error, setError] = useState('');
    const [showPopup, setShowPopup] = useState(false);
    const API_URL = process.env.REACT_APP_API_URL;

    useEffect(() => {
        //verification du profil
        fetch(`${API_URL}/api/auth/profile`, {
            method: 'GET',
            credentials: 'include',
        })
            .then(res => res.json())
            .then(data => {
                if (data.message === 'Non connecté') {
                    setError('Vous devez vous connecter.');
                } else {
                    setId(data.userId);
                    //chargement des todos
                    loadTodos(id);
                }
            })
            .catch(() => setError('Erreur serveur'));
    }, []);

    //chargements des taches
    const loadTodos = () => {
        fetch(`${API_URL}/api/todo/list`, {
            method: 'GET',
            credentials: 'include',
        })
            .then(res => res.json())
            .then(data => {
                if (data.message) {
                    setTodos(data.tasks);
                } else {
                    setTodos([]);
                }
            })
            .catch(() => setError('Erreur serveur'));
    };

    //cocher/decocher une tache
    const handleToggle = async (index) => {
        const updatedTodos = [...todos];
        //inverse la valeur
        updatedTodos[index].fini = !updatedTodos[index].fini;

        await fetch(`${API_URL}/api/todo/update`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            credentials: 'include',
            body: JSON.stringify({
                todos: updatedTodos,
            }),
        });
        //refresh la liste
        loadTodos();
    };

    //supresion des tache
    const handleDelete = async (index) => {
        const updatedTodos = todos.filter((_, i) => i !== index);

        await fetch(`${API_URL}/api/todo/update`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            credentials: 'include',
            body: JSON.stringify({
                todos: updatedTodos,
            }),
        });

        //refresh la liste
        loadTodos();
    };

    //ajout d'une tache
    const handleAddTodo = async (newTodo) => {
        const updated = [...todos, newTodo];

        await fetch(`${API_URL}/api/todo/update`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            credentials: 'include',
            body: JSON.stringify({
                todos: updated,
            }),
        });

        setShowPopup(false);

        //refresh la liste
        loadTodos();
    };

    console.log(id, todos);
    if (error) return <p>{error}</p>;
    if (!id || !todos) return <p>Chargement...</p>;

    return (
        <div style={{ padding: '20px' }}>
            <a href='/'>se relog</a>
            <a href='/profile'>profil</a>
            <h2>Bienvenue sur la liste Todo</h2>
            <p>ID utilisateur : {id}</p>

            <button onClick={() => setShowPopup(true)}>+ Ajouter une tâche</button>

            {todos.length === 0 ? (
                <p>Aucune tâche pour le moment</p>
            ) : (
                <ul>
                    {todos.map((todo, i) => (
                        <li key={i} style={{ margin: '8px 0' }}>
                            <input
                                type="checkbox"
                                checked={todo.fini}
                                onChange={() => handleToggle(i)}
                            />
                            <span
                                style={{
                                    textDecoration: todo.fini ? 'line-through' : 'none',
                                    marginLeft: '8px',
                                }}
                            >
                                {todo.name}
                            </span>
                            <button
                                onClick={() => handleDelete(i)}
                                style={{ marginLeft: '10px', color: 'red' }}
                            >
                                Supprimer
                            </button>
                        </li>
                    ))}
                </ul>
            )}

            {showPopup && (
                <div
                    style={{
                        position: 'fixed',
                        top: 0,
                        left: 0,
                        width: '100%',
                        height: '100%',
                        backgroundColor: 'rgba(0,0,0,0.5)',
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center',
                    }}
                >
                    <div style={{ background: 'white', padding: 20, borderRadius: 8 }}>
                        <h3>Ajouter une tâche</h3>
                        <TodoForm onAdd={handleAddTodo} onCancel={() => setShowPopup(false)} />
                    </div>
                </div>
            )}
        </div>
    );
}

export default TodoPage;
