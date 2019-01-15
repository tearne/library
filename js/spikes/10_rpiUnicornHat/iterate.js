var ws281x = require('rpi-ws281x-native');

var NUM_LEDS = parseInt(process.argv[2], 10) || 10,
    pixelData = new Uint32Array(NUM_LEDS);

ws281x.init(NUM_LEDS);

// ---- trap the SIGINT and reset before exit
process.on('SIGINT', function () {
  ws281x.reset();
  process.nextTick(function () { process.exit(0); });
});


// ---- animation-loop
var offset = 0;
setInterval(function () {
  var i=NUM_LEDS;
  while(i--) {
      pixelData[i] = 0;
  }
  pixelData[offset] = 0xffffff;

  offset = (offset + 1) % NUM_LEDS;
  ws281x.render(pixelData);
}, 30);

console.log('Press <ctrl>+C to exit.');
