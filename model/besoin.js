const mongoose = require('mongoose');
const Schema = mongoose.Schema;

let Besoin = new Schema({
    id: String,
    post_name: String,
    client: String,
    experience:Number,
    max_salary:Number,
    start_date:Date,
    creation_date:Date,
   MANAGER_NAME: String,
    cr_name: String,
    REF_OFFRE: String,
    TECHNO_ENVIRONNEMENT: String,
    status_name: String,
    candidatAfectedList:[]
  }, {
    collection: 'besoins'
  })
  
  module.exports = mongoose.model('Besoin', Besoin)

 
  
  