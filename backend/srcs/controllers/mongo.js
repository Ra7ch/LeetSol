const mongoose = require('mongoose');
const path = require('path');
const env = require('dotenv');

env.config();

mongoose.connect(`mongodb://${process.env.MONGO_INITDB_ROOT_USERNAME}:${process.env.MONGO_INITDB_ROOT_PASSWORD}@mongo:27017/auditSolana?authSource=admin`)
.then(() => console.log('Connected to MongoDB container'))
.catch(err => console.error('Failed to connect to MongoDB:', err));

const AuditReportSchema = new mongoose.Schema({
    contractName: String,
    filePath: String,
    report: Object,
    createdAt: { type: Date, default: Date.now }
});

const AuditReport = mongoose.model('AuditReport', AuditReportSchema);

const saveAuditReport = async (filePath, auditReport) => {
    const report = new AuditReport({
        contractName: path.basename(filePath),
        filePath: filePath, 
        report: auditReport
    });
    
    try {
        await report.save();
        console.log('Audit report saved successfully');
    } catch (err) {
        console.error('Failed to save audit report:', err);
    }
};

module.exports = { saveAuditReport };
