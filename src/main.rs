use linux_embedded_hal::I2cdev;
use ht16k33::{HT16K33, Dimming, Display, DisplayData, DisplayDataAddress, LedLocation};
use geodate::geodate::*;
use std::time::{SystemTime, Duration};
use std::thread;

pub const CHARS: [u8; 10] = [
    0x3F, // 0
    0x06, // 1
    0x5B, // 2
    0x4F, // 3
    0x66, // 4
    0x6D, // 5
    0x7D, // 6
    0x07, // 7
    0x7F, // 8
    0x6F, // 9
];

pub const INDEX: [u8; 4] = [0, 2, 6, 8];

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => {
            println!("Usage: geoclock <longitude> [<brightness>]");
            return
        }
        2 => {
            args.push("3".to_string())
        }
        _ => {
        }
    }

    let longitude = match args[1].parse() {
        Ok(longitude) => longitude,
        Err(_) => 0.0,
    };

    let brightness = match args[2].parse() {
        Ok(brightness_level) => brightness_level,
        Err(_) => 15,
    };

    let path = "/dev/i2c-1";
    let addr = 0x70u8;
    let continuous = true;

    let mut i2c = I2cdev::new(path).unwrap();
    i2c.set_slave_address(addr as u16).unwrap();

    let mut ht16k33 = HT16K33::new(i2c, addr);
    ht16k33.initialize().unwrap();
    ht16k33.set_display(Display::ON).unwrap();
    ht16k33.set_dimming(Dimming::from_u8(brightness).unwrap()).unwrap();

    // Turn colon on
    let row = DisplayDataAddress::from_bits_truncate(4);
    let common = DisplayData::from_bits_truncate(2);
    ht16k33.set_led(LedLocation { row, common }, true).unwrap();

    if continuous {
        println!("Refreshing geotime on display every 8.64 seconds");
    }

    loop {
        ht16k33.clear_display_buffer();
        let timestamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(time) => time.as_secs() as i64,
            Err(_) => 0
        };
        let date = get_formatted_date("%c%b", timestamp, longitude);
        for (i, d) in date.chars().enumerate() {
            let c = CHARS[d.to_digit(10).unwrap() as usize];
            let row = DisplayDataAddress::from_bits_truncate(INDEX[i]);
            let common = DisplayData::from_bits_truncate(c);
            let led_location = LedLocation { row, common };
            //let led_location = LedLocation::new(addr, data).unwrap();
            ht16k33.set_led(led_location, true).unwrap();
        }

        if !continuous {
            break
        }

        thread::sleep(Duration::from_millis(8640));
    }
}
