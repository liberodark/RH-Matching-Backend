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
    manager_name: String,
    cr_name: String,
    ko_tag: String,
    ref_offre: [],
    comment: String,
   mobility: String,
   disponibility_date: Date,
   salary: Number,
  cv: String
  }, {
    collection: 'candidats'
  })
  
  module.exports = mongoose.model('Candidat', Candidat)