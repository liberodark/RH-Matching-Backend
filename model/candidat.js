const mongoose = require('mongoose');
const Schema = mongoose.Schema;

let Candidat = new Schema({
    id: String,
    first_name: String,
    last_name: String,
    statusCandidat: String,
    MAIL_ADRESSE: String,
    status_date: Date,
    TELEPHONE_NUMBER: String,
    POST_NAME: String,
    CONTACTER_VIA: String,
    client: String,
    experience: String,
   MANAGER_NAME: String,
    CR_NAME: String,
    KO_TAG: String,
    REF_OFFRE: [],
    comment: String,
   mobility: String,
   DISPONIBILITY_DATE: Date,
   salary: Number,
  cv: String
  }, {
    collection: 'candidats'
  })
  
  module.exports = mongoose.model('Candidat', Candidat)