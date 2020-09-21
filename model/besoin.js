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
  MANAGER_NAME: [String],
  CR_NAME: String,
  REF_OFFRE: String,
  TECHNO_ENVIRONNEMENT: String,
  STATUS_NAME: String,
  // candidatAfectedList: [{ type: Schema.Types.ObjectId, ref: 'Candidat' }]
  candidatAfectedList: [String]
}, {
  collection: 'besoins'
})

module.exports = mongoose.model('Besoin', Besoin)
