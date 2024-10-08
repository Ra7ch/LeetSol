const express = require('express');
const dotenv = require('dotenv');
const fs = require('fs');
const auditRoutes = require('./routes/audit');
const path = require('path');
const multer = require('multer');
const app = express();

const cors = require('cors');
app.use(cors());

dotenv.config();

app.use(express.json());
app.use('/audit', auditRoutes);

app.get('/', (req, res) => {
    res.send('Hello World');
});

app.get('/about', (req, res) => {
    res.send('About Us');
});

const PORT = process.env.PORT || 3000;
app.listen(PORT, () => {
    console.log(`Server running on port ${PORT}`);
});