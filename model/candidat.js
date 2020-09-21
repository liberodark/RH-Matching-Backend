const mongoose = require('mongoose');
const Schema = mongoose.Schema;

let Candidat = new Schema({
  id: String,
  FIRST_NAME: String,
  LAST_NAME: String,
  statusCandidat: String,
  MAIL_ADRESSE: String,
  STATUS_DATE: Date,
  TELEPHONE_NUMBER: String,
  POST_NAME: String,
  CONTACTER_VIA: String,
  client: String,
  experience: String,
  MANAGER_NAME: [String],
  CR_NAME: String,
  KO_TAG: String,
  REF_OFFRE: [String],
  comment: String,
  mobility: String,
  DISPONIBILITY_DATE: Date,
  salary: Number,
  cv: String,
  fullOffers:[]
}, {
  collection: 'candidats'
})

module.exports = mongoose.model('Candidat', Candidat)
