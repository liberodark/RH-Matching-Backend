const mongoose = require('mongoose');
const Schema = mongoose.Schema;

let Dashboard = new Schema({
    id: String,
    DashboardBesoin:[String],
    DashboardCandidat:[String]
}, {
  collection: 'candidats'
});

module.exports = mongoose.model('Dashboard',  Dashboard);