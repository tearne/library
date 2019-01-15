var http = require('http');
var mongoose = require('mongoose');
var express = require('express');

var app = express();
var config = require('./config.js');

var standardGreeting = 'Hello World!';

var greetingSchema = mongoose.Schema({
    sentence: String
});

var Greeting = mongoose.model('Greeting', greetingSchema);

mongoose.connect(config.dbpath);

var db = mongoose.connection;
db.on('error', console.error.bind(console, 'connection error:'));
db.once('open', function() {
    console.info("DB connected without screw-up");
    var greeting;
    Greeting.find(function(err, greetings) {
        if (err) console.info(err);
        console.info("Greetings = " + greetings)
        //if( !greetings ){     
            greeting = new Greeting({
                sentence: standardGreeting
            });
            greeting.save();
            console.info("Saved " + greeting)
        //} 
    });
});

app.get('/', function(req, res) {
    Greeting.findOne(function(err, greeting) {
        if (err) res.send("An error!");
        else res.send(greeting.sentence);
    });
});

app.use(function(err, req, res, next) {
    if (req.xhr) {
        res.send(500, 'Something went wrong!');
    }
    else {
        next(err);
    }
});

console.log('starting Express');
app.listen(80);
console.log('Listening on 8080');