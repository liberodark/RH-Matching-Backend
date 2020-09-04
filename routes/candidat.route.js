const express = require('express');
const candidatRoute = express.Router();

// candidat model
let Candidat = require('../model/candidat');

// Add candidat
candidatRoute.route('/add-candidat').post((req, res, next) => {
  Candidat.create(req.body, (error, data) => {
    if (error) {
      return next(error);
    } else {
      res.json(data);
    }
  })
});

// Get all candidat
candidatRoute.route('/candidats').get((req, res) => {
  Candidat.find((error, data) => {
    if (error) {
      return next(error);
    } else {
      res.json(data);
    };
  })
})

// Get single candidat
candidatRoute.route('/read-candidat/:id').get((req, res) => {
  Candidat.findById(req.params.id, (error, data) => {

    if (error) {
      return next(error);
    } else {
      res.json(data);
    };
  })
})

// Get besoins by status
candidatRoute.route('/candidats/:status').get((req, res) => {
  Candidat.find((error, data) => {
    if (error) {
      return next(error)
    } else {
      var response = [];
      response = data.filter(function(candidat) {
        if (candidat.statusCandidat === req.params.status) {
          return candidat;
        }
      });
      res.json(response);
    }
  })
})

// Update candidat
candidatRoute.route('/update-candidat/:id').put((req, res, next) => {
  Candidat.findByIdAndUpdate(req.params.id, {
    $set: req.body
  }, (error, data) => {
    if (error) {
      return next(error);
    } else {
      res.json(data);
      console.log('candidat successfully updated!');
    }
  })
})

// Delete candidat
candidatRoute.route('/delete-candidat/:id').delete((req, res, next) => {
  Candidat.findByIdAndRemove(req.params.id, (error, data) => {
    if (error) {
      return next(error);
    } else {
      res.status(200).json({
        msg: data
      })
    }
  })
})

module.exports = candidatRoute;
