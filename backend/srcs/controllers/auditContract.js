const path = require('path');
const fs = require('fs');
const axios = require('axios');
const async = require('async');
const { performRustAudit } = require('./auditTools');
const { saveAuditReport } = require('./mongo');

const auditContract = async (req, res) => {
    if (!req.file || req.file.length === 0) {
        return res.status(400).json({ error: 'Please upload a contract file.' });
    }

    const filePath = req.file.path; 
    const fileType = path.extname(req.file.originalname).toLowerCase(); 
    const auditReport = null;

    const generatedFileName = req.file.filename;

    const rustSharedVolumePath = '/usr/src/app/uploads';

    const rustFilePath = path.join(rustSharedVolumePath, generatedFileName);

    try {
        const fileContent = await fs.promises.readFile(filePath, 'utf8');

        if (fileType === '.rs') {
            const auditReport = await performRustAudit(rustFilePath);

            if (!auditReport || auditReport.length === 0) {
                return res.status(200).send('<p style="color: green; font-weight: bold;">No vulnerabilities found.</p>');
            } else {
                const reportHtml = auditReport.map(item => `<p style="color: green; font-weight: bold;">- ${item}</p>`).join('');
                return res.status(200).send(`${reportHtml}`);
            }
        } else {
            return res.status(400).json({ error: 'Unsupported file type' });
        }
    } catch (error) {
        console.error('Error processing file:', error);
        return res.status(500).json({ error: 'Failed to process file' });
    } finally {

        saveAuditReport (filePath, auditReport);

        fs.unlink(filePath, (err) => {
            if (err) {
                console.error('Failed to delete file:', err);
            } else {
                console.log('Temporary file deleted');
            }
        });
    }

};

module.exports = { auditContract };
