const express = require('express');
const { login, profile } = require('../controllers/authController');
const authMiddleware = require('../middlewares/authMiddleware');
const router = express.Router();

// Route POST /api/auth/login
router.post('/login', login);

// Route protégée par le middleware
router.get('/profile', authMiddleware, profile);

module.exports = router;
