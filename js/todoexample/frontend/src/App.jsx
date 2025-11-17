import { BrowserRouter, Routes, Route } from 'react-router-dom';
import LoginPage from './pages/LoginPage';
import ProfilePage from './pages/ProfilePage';
import TodoPage from './pages/TodoPage';

function App() {
    return (
        <BrowserRouter>
            <Routes>
                <Route path="/" element={<LoginPage />} />
                <Route path="/profile" element={<ProfilePage />} />
                <Route path="/todo" element={<TodoPage />} />
            </Routes>
        </BrowserRouter>
    );
}


export default App;
