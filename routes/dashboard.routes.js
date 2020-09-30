const express = require('express');
const dashboardRoute = express.Router();


// candidat model
let Dashboard = require('../model/dashboard');
const { response } = require('express');
//const Dashboard = require('../model/dashboard');


dashboardRoute.route('/add-dashboard').post((req, res, next) => {
    Dashboard.create(req.body, (error, data) => {
      if (error) {
        return next(error);
      } else {
        res.json(data);
      }
    })
  });

  dashboardRoute.route('/candidats').get((req, res) => {
    Dashboard.find((error, data) => {
      if (error) {
        return next(error);
      } else {
        res.json(data);
      }
    })
  });