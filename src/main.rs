mod pi;

use pi::gpio::Pin;
use pi::light::{Light, LightState};
use pi::button::{Button, ButtonState};
use pi::polled_button::PolledButton;
use pi::screen::{Screen, PixelScreen};
use pi::adafruit_oled::AdafruitOled;
use std::thread;

const STATUS_LIGHT_PIN : usize = 5;
const SWITCH_IN_PIN : usize = 6;
const ADAFR_SCL_PIN : usize = 10;
const ADAFR_SDA_PIN : usize = 11;

fn red_screen(screen : &AdafruitOled) {
	let (width, height) = screen.resolution();
	for x in 0..width {
		for y in 0..height {
			screen.set(x, y, (255, 0, 0));
		}
	}
}

fn main() {
	let status_light = Light::new(Pin::new(STATUS_LIGHT_PIN));
	let switch_in = PolledButton::new(Pin::new(SWITCH_IN_PIN));
	let screen = AdafruitOled::new(Pin::new(ADAFR_SCL_PIN), Pin::new(ADAFR_SDA_PIN));
	
	loop {
		status_light.set_state(match switch_in.read_state() {
			ButtonState::Pressed => LightState::On,
			ButtonState::NotPressed => LightState::Off
		});
		red_screen(&screen);
		thread::sleep_ms(50);
	}
}
