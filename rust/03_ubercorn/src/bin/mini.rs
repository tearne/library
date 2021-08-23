use gpio_cdev::{AsyncLineEventHandle, Chip, EventRequestFlags, Line, LineHandle, LineRequestFlags, Lines};
// use rppal::gpio::{Gpio, InputPin, OutputPin, Trigger};
use spidev::{SpiModeFlags, Spidev, SpidevOptions};
use std::{io::Write, ops::Range};

//TODO
// Use: https://crates.io/crates/gpio-cdev
// Button debounce
// Why button Y not working?


// Useful:
// https://github.com/pimoroni/unicornhatmini-python/blob/master/library/unicornhatmini/__init__.py
// https://pinout.xyz/pinout/unicorn_hat_mini#

// SPI stuff
// https://raspberrypi.stackexchange.com/questions/121713/what-should-i-look-for-to-find-the-proper-gpio-chip-on-the-system
// https://github.com/dotnet/iot/blob/main/Documentation/raspi-spi.md
// https://github.com/raspberrypi/firmware/blob/7b99da75f55a5ad7d572ec4ebe4e8f9573deaee7/boot/overlays/README#L2437

// Holtek HT16D35
const CMD_SOFT_RESET:[u8;1] = [0xCC];
const CMD_GLOBAL_BRIGHTNESS:[u8;2] = [0x37, 0x01];
const CMD_COM_PIN_CTRL:[u8;2] = [0x41, 0xff];
const CMD_ROW_PIN_CTRL:[u8;5] = [0x42, 0xff, 0xff, 0xff, 0xff];
const CMD_WRITE_DISPLAY:[u8;2] = [0x80, 0x00];
// const CMD_READ_DISPLAY:[u8;1] = [0x81];
const CMD_SYSTEM_CTRL_OFF:[u8;2] = [0x35, 0x00];
const CMD_SYSTEM_CTRL_ON:[u8;2]= [0x35, 0x03];
const CMD_SCROLL_CTRL:[u8;2] = [0x20, 0x00];

const BUTTON_A: u32 = 5;
const BUTTON_B: u32 = 6;
const BUTTON_X: u32 = 16;
const BUTTON_Y: u32 = 20;
const BUTTONS: [u32;4] = [BUTTON_A, BUTTON_B, BUTTON_X, BUTTON_Y]; 

const LUT: [[usize; 3]; 119] = [[139, 138, 137], [223, 222, 221], [167, 166, 165], [195, 194, 193], [111, 110, 109], [55, 54, 53], [83, 82, 81], [136, 135, 134], [220, 219, 218], [164, 163, 162], [192, 191, 190], [108, 107, 106], [52, 51, 50], [80, 79, 78], [113, 115, 114], [197, 199, 198], [141, 143, 142], [169, 171, 170], [85, 87, 86], [29, 31, 30], [57, 59, 58], [116, 118, 117], [200, 202, 201], [144, 146, 145], [172, 174, 173], [88, 90, 89], [32, 34, 33], [60, 62, 61], [119, 121, 120], [203, 205, 204], [147, 149, 148], [175, 177, 176], [91, 93, 92], [35, 37, 36], [63, 65, 64], [122, 124, 123], [206, 208, 207], [150, 152, 151], [178, 180, 179], [94, 96, 95], [38, 40, 39], [66, 68, 67], [125, 127, 126], [209, 211, 210], [153, 155, 154], [181, 183, 182], [97, 99, 98], [41, 43, 42], [69, 71, 70], [128, 130, 129], [212, 214, 213], [156, 158, 157], [184, 186, 185], [100, 102, 101], [44, 46, 45], [72, 74, 73], [131, 133, 132], [215, 217, 216], [159, 161, 160], [187, 189, 188], [103, 105, 104], [47, 49, 48], [75, 77, 76], [363, 362, 361], [447, 446, 445], [391, 390, 389], [419, 418, 417], [335, 334, 333], [279, 278, 277], [307, 306, 305], [360, 359, 358], [444, 443, 442], [388, 387, 386], [416, 415, 414], [332, 331, 330], [276, 275, 274], [304, 303, 302], [337, 339, 338], [421, 423, 422], [365, 367, 366], [393, 395, 394], [309, 311, 310], [253, 255, 254], [281, 283, 282], [340, 342, 341], [424, 426, 425], [368, 370, 369], [396, 398, 397], [312, 314, 313], [256, 258, 257], [284, 286, 285], [343, 345, 344], [427, 429, 428], [371, 373, 372], [399, 401, 400], [315, 317, 316], [259, 261, 260], [287, 289, 288], [346, 348, 347], [430, 432, 431], [374, 376, 375], [402, 404, 403], [318, 320, 319], [262, 264, 263], [290, 292, 291], [349, 351, 350], [433, 435, 434], [377, 379, 378], [405, 407, 406], [321, 323, 322], [265, 267, 266], [293, 295, 294], [352, 354, 353], [436, 438, 437], [380, 382, 381], [408, 410, 409], [324, 326, 325], [268, 270, 269], [296, 298, 297]];
const BUF_SIZE: usize = 28 * 8;

struct UnicornMini {
    data_buf: [u8; BUF_SIZE * 2],
    spi: [Spidev; 2],
    buttons: [Line; 4],
}
impl UnicornMini {
    pub fn new() -> Self {
        // let gpio = Gpio::new().unwrap();

        fn get_spi(address: &str) -> Spidev {
            let mut spi = Spidev::open(address).unwrap();
            let options = SpidevOptions::new()
                .max_speed_hz(600_000)
                .bits_per_word(8)
                .mode(SpiModeFlags::SPI_MODE_0)
                // .mode(SpiModeFlags::SPI_LSB_FIRST)
                // .mode(SpiModeFlags::SPI_NO_CS)
                // .mode(SpiModeFlags::SPI_READY)
                .build();
            spi.configure(&options).expect("SPI config error");
            spi
        }
        

        let mut chip = Chip::new("/dev/gpiochip0").unwrap();
        // let a = chip.get_line(BUTTON_A).unwrap().request(LineRequestFlags::INPUT, 0, "read-button-A").unwrap();


        let mut um = UnicornMini {
            data_buf: [0; BUF_SIZE * 2],
            spi: [
                get_spi("/dev/spidev0.0"),
                get_spi("/dev/spidev0.1"),
            ],
            // button_lines: chip.get_lines(&BUTTONS).unwrap()
            buttons: [
                chip.get_line(BUTTON_A).unwrap(),//.request(LineRequestFlags::INPUT, 0, "read-button-A").unwrap(),
                chip.get_line(BUTTON_B).unwrap(),//.request(LineRequestFlags::INPUT, 0, "read-button-B").unwrap(),
                chip.get_line(BUTTON_X).unwrap(),//.request(LineRequestFlags::INPUT, 0, "read-button-X").unwrap(),
                chip.get_line(BUTTON_Y).unwrap(),//.request(LineRequestFlags::INPUT, 0, "read-button-Y").unwrap()
            ]
        };

        um.write_prefix(&CMD_SOFT_RESET, &[]);
        um.write_prefix(&CMD_GLOBAL_BRIGHTNESS, &[]);
        um.write_prefix(&CMD_SCROLL_CTRL, &[]);
        um.write_prefix(&CMD_SYSTEM_CTRL_OFF, &[]);
        um.write_prefix(&CMD_WRITE_DISPLAY, &um.data_buf.clone());//TODO without clone
        um.write_prefix(&CMD_COM_PIN_CTRL, &[]);
        um.write_prefix(&CMD_ROW_PIN_CTRL, &[]);
        um.write_prefix(&CMD_SYSTEM_CTRL_ON, &[]);

        um
    }

    fn buf_offset(buffer_idx: usize) -> Range<usize> {
        buffer_idx * BUF_SIZE .. (buffer_idx + 1) * BUF_SIZE
    }

    fn write(&mut self, data: &[u8]){
        self.write_prefix(&CMD_WRITE_DISPLAY, data)
    }
    fn write_prefix(&mut self, prefix: &[u8], data: &[u8]){
        fn concat(a: &[u8], b: &[u8]) -> Vec<u8> {
            let mut d = a.to_owned();
            d.extend(b);
            d
        }

        // Send data to both chips
        for i in 0..2 {
            let spi = &mut self.spi[i];
            if data.len() > 0 {
                let chunk = &data[Self::buf_offset(i)];
                spi.write(&concat(&prefix, &chunk)).expect("SPI write error");
            } else {
                spi.write(&prefix).expect("SPI write error");
            }
            // println!("Buffer {}, sent {}:\n{:?}", i, d.len()/3, d);
        }
    }

    pub fn set(&mut self, idx: usize, r: u8, g: u8, b: u8) {
        let [ir, ig, ib] = LUT[idx];
        self.data_buf[ir] = r;
        self.data_buf[ig] = g;
        self.data_buf[ib] = b;
    }
    pub fn flush(&mut self){
        self.write(&self.data_buf.clone());//TODO without clone
    }
}

#[tokio::main]
async fn main() {
    let mut um = UnicornMini::new();

    // for i in 0..1000usize {
    //     um.set(i%117, (i%3%250) as u8,(i%5%250) as u8,(i%7%250) as u8);
    //     um.flush();
    // }


    um.set(2, 100,0,0);
    um.set(4, 0,100,0);
    um.set(6, 0,0,100);
    um.set(8, 100,0,0);
    um.set(10, 0,100,0);
    um.set(12, 0,0,100);
    um.set(14, 100,0,0);

    um.set(100, 100,100,200);
    um.set(102, 100,200,100);
    um.set(104, 200,100,100);
    um.set(106, 100,100,200);
    um.set(108, 100,200,100);
    um.set(110, 200,100,100);

    um.flush();


    let mut events = um.buttons[0].async_events(
        LineRequestFlags::INPUT, 
        EventRequestFlags::BOTH_EDGES,
        "button-a-events").unwrap();

    // let events = t.await;

    use futures::stream::StreamExt;
    while let Some(event) = events.next().await {
        let event = event.unwrap();
        println!("GPIO Event: {:?}", event);
    }

    // for event in um.buttons[0].events(
    //     LineRequestFlags::INPUT,
    //     EventRequestFlags::BOTH_EDGES,
    //     "button-a-events",
    // ).unwrap(){
    //     println!("Event: {:?}", event)
    // }

    // for i in 0..4 {
    //     println!("Value: {:?}", um.buttons[i].get_value().unwrap());
    // }

    // println!("-- A");
    // um.buttons[0].set_async_interrupt(Trigger::RisingEdge, |a|println!(" A, {:?}",a)).unwrap();
    // um.buttons[2].set_async_interrupt(Trigger::RisingEdge, |a|println!("  X, {:?}",a)).unwrap();
    // um.buttons[3].set_async_interrupt(Trigger::RisingEdge, |a|println!("   Y, {:?}",a)).unwrap();
    // println!("-- B");   
    // um.buttons[1].set_interrupt(Trigger::RisingEdge).unwrap();
    // um.buttons[1].poll_interrupt(true, None).unwrap();
}