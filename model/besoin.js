const mongoose = require('mongoose');
const Schema = mongoose.Schema;

let Besoin = new Schema({
  id: String,
  POST_NAME: String,
  client: String,
  experience: String,
  MAX_SALARY: Number,
  START_DATE: Date,
  CREATION_DATE: Date,
  MANAGER_NAME: [],
  CR_NAME: String,
  REF_OFFRE: String,
  TECHNO_ENVIRONNEMENT: String,
  STATUS_NAME: String,
  candidatAfectedList: []
}, {
  collection: 'besoins'
})

module.exports = mongoose.model('Besoin', Besoin)
