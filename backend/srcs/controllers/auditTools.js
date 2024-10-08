const path = require('path');
const axios = require('axios');

const fileFilter = (req, file, cb) => {
    console.log("checking file");
    const allowedTypes = ['.rs'];
    const ext = path.extname(file.originalname).toLowerCase();
    if (allowedTypes.includes(ext)) {
        cb(null, true);
    } else {
        cb(new Error('Only .rs files are allowed!'), false);
    }
};

const performRustAudit = async (contractCode) => {
    try {
        const response = await axios.post('http://rust-audit-service:8080/audit', {
            contract_path: contractCode
        });
        return response.data.report;
    } catch (error) {
        console.error('Error auditing Rust contract:', error);
        throw new Error('Rust audit service failed');
    }
};

module.exports = { performRustAudit, fileFilter };