const mongoose = require('mongoose');
const Schema = mongoose.Schema;

let Besoin = new Schema({
    id: String,
    POST_NAME: String,
    client: String,
    experience:Number,
    max_salary:Number,
    start_date:Date,
    creation_date:Date,
   MANAGER_NAME: String,
    CR_NAME: String,
    REF_OFFRE: String,
    TECHNO_ENVIRONNEMENT: String,
    STATUS_NAME: String,
    candidatAfectedList:[]
  }, {
    collection: 'besoins'
  })
  
  module.exports = mongoose.model('Besoin', Besoin)

 
  
  