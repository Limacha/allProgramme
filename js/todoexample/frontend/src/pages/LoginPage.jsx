import React from 'react';
import LoginForm from '../components/LoginForm';

function LoginPage() {
    return (
        <div style={{ display: 'flex', justifyContent: 'center', marginTop: '50px' }}>
            <LoginForm />
            <a href='/todo'>liste de todo</a>
            <a href='/profile'>profil</a>
        </div>
    );
};

export default LoginPage;
