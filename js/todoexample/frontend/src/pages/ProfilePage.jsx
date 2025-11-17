import React, { useEffect, useState } from 'react';

function ProfilePage() {
    const [data, setData] = useState(null);
    const [error, setError] = useState('');
    const API_URL = process.env.REACT_APP_API_URL;

    useEffect(() => {
        fetch(`${API_URL}/api/auth/profile`, {
            method: 'GET',
            credentials: 'include',
        })
            .then(res => res.json())
            .then(data => {
                if (data.message === 'Non connecté') {
                    setError('Vous devez vous connecter.');
                } else {
                    setData(data);
                }
            })
            .catch((err) => setError('Erreur serveur' + err.message));
    }, []);

    if (error) return <p>{error}</p>;
    if (!data) return <p>Chargement...</p>;

    return (
        <div>
            <h2>Bienvenue sur votre profil</h2>
            <p>ID utilisateur : {data.userId}</p>
            <a href='/todo'>liste de todo</a>
            <a href='/'>se relog</a>
        </div>
    );
}

export default ProfilePage;
