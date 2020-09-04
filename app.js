let express = require('express'),
  path = require('path'),
  mongoose = require('mongoose'),
  cors = require('cors'),
  bodyParser = require('body-parser'),
  dataBaseConfig = require('./database/db');

// Connecting mongoDB
mongoose.Promise = global.Promise;
mongoose.connect(dataBaseConfig.db, {
  useUnifiedTopology: true,
  useNewUrlParser: true,
  useCreateIndex: true,
  useFindAndModify: false
}).then(() => {
    console.log('Database connected sucessfully ')
  },
  error => {
    console.log('Could not connected to database : ' + error)
  }
)


// Set up express js port
const candidatRoute = require('./routes/candidat.route')
const besoinRoute = require('./routes/besoin.route')
const app = express();
app.use(bodyParser.json());
app.use(bodyParser.urlencoded({
  extended: false
}));
app.use(cors());
app.use(express.static(path.join(__dirname, '/var/www/rh-matching')));
app.use('/', express.static(path.join(__dirname, '/var/www/rh-matching')));
app.use('/api', candidatRoute);
app.use('/api', besoinRoute);

// Create port
const port = process.env.PORT || 4000;
const server = app.listen(port, () => {
  console.warn('Connected to port ' + port + 'server' + server)
})

// Find 404 and hand over to error handler
app.use((req, res, next) => {
  next(createError(404));
});

// error handler
app.use(function(err, req, res, next) {
  console.warn(next);
  console.error(err.message);
  if (!err.statusCode) err.statusCode = 500;
  res.status(err.statusCode).send(err.message);
});
