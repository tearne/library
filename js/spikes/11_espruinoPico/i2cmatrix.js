I2C1.setup({scl:B6, sda:B7});
I2C1.writeTo(0x70, 0x21); // turn on oscillator
I2C1.writeTo(0x70, 0x81); // disp on
I2C1.writeTo(0x70, 0xE0 | 0); // brightness 15
// not sure about this, you may need 16,8,1
    var g = Graphics.createArrayBuffer(16,8,1); 
g.flip = function() {
  I2C1.writeTo(0x70, 0, g.buffer); 
};
g.drawLine(0,0,8,8);
g.flip();
