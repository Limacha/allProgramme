const express = require('express');
const { update, list } = require('../controllers/todoController');
const router = express.Router();


router.post('/update', update);


router.get('/list', list);

module.exports = router;
