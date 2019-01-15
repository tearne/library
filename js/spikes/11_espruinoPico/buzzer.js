var BUZZER = B4;

function freq(f) { 
  if (f===0) digitalWrite(BUZZER,0);
  else analogWrite(BUZZER, 0.5, { freq: f } );
}

var s = atob("f4GBgoODh5GtnmgkE1m435g/MobgzYJbd41YHkHD/7UoEZPysyscl/K5PSqLy5tLToqdfHGdrXI1V7vUeiVIqceEUG2kmWVfj6qIWFuKpZFpXXiXknNthZB9cH6LgG5vgJKSdWF7oZVoXHuRioJ8c3iJjHtweomLf3JygpCHdHOCiYJ8fYCBg4ODgn53d4CGiIZ8cHGAjo1+dn6Jh3pzeoWHfXd8hYd8d3+Gg3t1e4F9d36KiHp0fYZ+cneLlol3cnyEgXt8g4WBeXV8hoqDdnOCjoNxc4aRhnd3f4J6cXmNlIJxd4mKeXJ5iI2Denh9fX1/f4B/g4N2bnuSlYJzdoOIgXp7g4V+dnmDhoF9gIF+eHqDioJyb3+Oi3xzeIOHgXp7gIB+fH+DgHt8hIh/cnOEjoVzb32Mi3pxeYeLf3N2hIqBdXaBhoB7fYSGfXZ7hYeAeXl9goJ9e36BgYGCgoF8e4GDgHp6f4ODgHo=");
var w = new Waveform(s.length);
w.buffer.set(s);

analogWrite(BUZZER, 0.5, {freq:20000}); 
w.startOutput(BUZZER,4000);
