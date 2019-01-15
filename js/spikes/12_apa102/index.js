var hooloovoo = require('hooloovoo')

hooloovoo.setup(64,20);
//hooloovoo.set_clock(20);

var intervalId;

process.on('SIGINT', function () {
  console.log('bye bye.'+intervalId);
  clearInterval(intervalId);
  hooloovoo.fill_RGB(1,2,3);
  setTimeout(function() {process.exit(0)}, 1000);
  //process.nextTick(function () { process.exit(0); });
});

function rnd() {
  return Math.floor(Math.random()*255);
}

function doStuff(){
  for(i=0; i<64; i++){
     hooloovoo.set_pixel_BGRb(i, rnd(), rnd(), rnd(), rnd())  
  }
}

intervalId = setInterval(doStuff, 50);
//console.log(intervalId);
//clearInterval(intervalId);
