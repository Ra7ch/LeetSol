const express = require('express');
const multer = require('multer');
const path = require('path');
const { auditContract } = require('../controllers/auditContract');
const { fileFilter } = require('../controllers/auditTools');
const router = express.Router();

const storage = multer.diskStorage({
    destination: (req, file, cb) => {
        cb(null, '/usr/src/app/uploads');
    },
    filename: (req, file, cb) => {
        cb(null, Date.now() + path.extname(file.originalname));
    }
});

const upload = multer({ storage: storage, fileFilter: fileFilter }).single('contractFile');

console.log("upload", upload);


router.post('/upload', (req, res) => {
    upload(req, res, function (err) {
        if (err instanceof multer.MulterError) {
            return res.status(400).json({ error: 'A Multer error occurred when uploading.' });
        } else if (err) {
            return res.status(400).json({ error: err.message });
        }
        auditContract(req, res);
    });
});

module.exports = router;
