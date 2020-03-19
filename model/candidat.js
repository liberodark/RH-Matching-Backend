const mongoose = require('mongoose');
const Schema = mongoose.Schema;

let Candidat = new Schema({
    id: String,
    first_name: String,
    last_name: String,
    statusCandidat: String,
    mail_adresse: String,
    status_date: Date,
    telephone_number: String,
    post_name: String,
    contacter_via: String,
    client: String,
    experience: String,
   MANAGER_NAME: String,
    CR_NAME: String,
    ko_tag: String,
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