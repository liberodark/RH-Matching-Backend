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
    manager_name: String,
    cr_name: String,
    ref_offre: String,
    techno_environnement: String,
    status_name: String,
    candidatAfectedList:[]
  }, {
    collection: 'besoins'
  })
  
  module.exports = mongoose.model('Besoin', Besoin)

 
  
  