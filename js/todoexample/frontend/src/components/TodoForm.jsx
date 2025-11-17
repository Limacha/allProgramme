import React, { useState } from 'react';

function TodoForm({ onAdd, onCancel }) {
    const [name, setName] = useState('');
    const [error, setError] = useState('');
    const API_URL = process.env.REACT_APP_API_URL;

    const handleSubmit = async (e) => {
        e.preventDefault();

        if (!name.trim()) {
            setError('Veuillez entrer un nom de tâche');
            return;
        }

        try {
            onAdd({ name, fini: false });
        } catch {
            setError('Erreur serveur');
        }
    };

    return (
        <form onSubmit={handleSubmit}>
            <input
                type="text"
                placeholder="Nom de la tâche"
                value={name}
                onChange={(e) => setName(e.target.value)}
            />
            <button type="submit">Ajouter</button>
            <button type="button" onClick={onCancel} style={{ marginLeft: '10px' }}>
                Annuler
            </button>
            {error && <p style={{ color: 'red' }}>{error}</p>}
        </form>
    );
}

export default TodoForm;
