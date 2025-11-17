import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';

function LoginForm() {
    const [name, setName] = useState(''); // Stocke le nom
    const [password, setPassword] = useState(''); // Stocke le mot de passe
    const [error, setError] = useState('');
    const navigate = useNavigate();
    const API_URL = process.env.REACT_APP_API_URL;


    const handleLogin = async (e) => {
        e.preventDefault();

        try {
            const response = await fetch(`${API_URL}/api/auth/login`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                credentials: 'include',
                body: JSON.stringify({ name, password }),
            });

            const data = await response.json();

            if (response.ok) {
                navigate('/profile');
            } else {
                setError(data.message || 'Erreur de connexion');
            }
        } catch (err) {
            setError('Erreur serveur');
        }
    };

    return (
        <form onSubmit={handleLogin}>
            <p>{`${API_URL}/api/auth/login`}</p>
            <input type="text" placeholder="Nom" onChange={(e) => setName(e.target.value)} />
            <input type="password" placeholder="Mot de passe" onChange={(e) => setPassword(e.target.value)} />
            <button type="submit">Se connecter</button>
            {error && <p style={{ color: 'red' }}>{error}</p>}
        </form>
    );
};

export default LoginForm;
