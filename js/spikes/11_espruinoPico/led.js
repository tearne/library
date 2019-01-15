function start(pin, period) {
  var on = false;
  setInterval(function() {
    on = !on;
    digitalWrite(pin, on);
  }, period);
}

start(LED1, 200);
start(LED2, 210);
