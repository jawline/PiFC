extern crate rustc_serialize;

mod pi;
mod fccore;

use pi::gpio::Pin;
use pi::screen::{Screen, PixelScreen};
use pi::adafruit_oled::AdafruitOled;
use std::thread;
use fccore::fccore::FCCore;

const ADAFR_SCL_PIN : usize = 10;
const ADAFR_SDA_PIN : usize = 11;
const BASE_CFG_FILE : &'static str = "./assets/base.cfg";

fn red_screen(screen : &AdafruitOled) {
	let (width, height) = screen.resolution();
	for x in 0..width {
		for y in 0..height {
			screen.set(x, y, (255, 0, 0));
		}
	}
}

fn main() {
	let core = FCCore::new(BASE_CFG_FILE);
	let screen = AdafruitOled::new(Pin::new(ADAFR_SCL_PIN), Pin::new(ADAFR_SDA_PIN));
	
	loop {
		if core.lock().unwrap().armed {
			red_screen(&screen);
		}
		thread::sleep_ms(50);
	}
}
