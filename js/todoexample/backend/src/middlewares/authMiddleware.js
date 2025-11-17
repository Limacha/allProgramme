function authMiddleware(req, res, next) {
    if (!req.session || !req.session.userId) {
        return res.status(401).json({ message: 'Non connecté' });
    }
    next();
}

module.exports = authMiddleware;