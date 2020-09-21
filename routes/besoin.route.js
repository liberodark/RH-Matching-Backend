const express = require('express');
const besoinRoute = express.Router();

// besoin model
let Besoin = require('../model/besoin');

// Add besoin
besoinRoute.route('/add-besoin').post((req, res, next) => {
    Besoin.create(req.body, (error, data) => {
    if (error) {
      return next(error)
    } else {
      res.json(data)
    }
  })
});

// Get all besoin
besoinRoute.route('/besoins').get((req, res) => {
  Besoin.find((error, data) => {
    if (error) {
      return next(error)
    } else {
      res.json(data)
    }
  })
});

/*besoinRoute.route('/besoins').get((req, res) => {
  Besoin.find().
  populate('candidat').
  exec(function (err, besoin) {
    if (err) return handleError(err);
    console.log('The besoin is %s', besoin.candidat.name);
    // prints "The author is Ian Fleming"
  })
});*/

// Get besoins by Status
besoinRoute.route('/besoins/:status').get((req, res) => {
  Besoin.find((error, data) => {
    if (error) {
      return next(error)
    } else {
      var response = [];  
        response = data.filter(function(besoin){
          if(besoin.STATUS_NAME === req.params.status){
            return besoin;
          }
        });
      res.json(response)
    }
  })
});

// Get besoins by reference
besoinRoute.route('/besoins/ByReference/:reference').get((req, res) => {
  Besoin.find((error, data) => {
    if (error) {
      return next(error)
    } else {
      var response = [];  
        response = data.filter(function(besoin){
          if(besoin.REF_OFFRE === req.params.reference){
            return besoin;
          }
        });
      res.json(response)
    }
  })
});

// Get single besoin
besoinRoute.route('/read-besoin/:id').get((req, res) => {
  Besoin.findById(req.params.id, (error, data) => {
    if (error) {
      return next(error)
    } else {
      res.json(data)
    }
  })
});


// Update besoin
besoinRoute.route('/update-besoin/:id').put((req, res, next) => {
  Besoin.findByIdAndUpdate(req.params.id, {
    $set: req.body
  }, (error, data) => {
    if (error) {
      return next(error);
    } else {
      res.json(data)
      console.log('besoin successfully updated!')
    }
  })
});

// Delete besoin
besoinRoute.route('/delete-besoin/:id').delete((req, res, next) => {
  Besoin.findByIdAndRemove(req.params.id, (error, data) => {
    if (error) {
      return next(error);
    } else {
      res.status(200).json({
        msg: data
      })
    }
  })
});

module.exports = besoinRoute;