var morgan = require('morgan');
var express = require('express'),
    app = express();

app.use(morgan('combined'));

app.get('/kittens', function(req, res){
    res.send('Hello World');
});

app.listen(process.env.PORT);
console.log('Express server started on port %s', process.env.PORT);