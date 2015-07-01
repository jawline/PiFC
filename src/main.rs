mod pi;

use pi::gpio::Pin;
use pi::light::{Light, LightState};
use pi::button::{Button, ButtonState};
use pi::polled_button::PolledButton;
use std::thread;

const STATUS_LIGHT_PIN : usize = 5;
const SWITCH_IN_PIN : usize = 6;
const ADAFR_SCL_PIN : usize = 10;
const ADAFR_SDA_PIN : usize = 11;

fn main() {
	let status_light = Light::new(Pin::new(STATUS_LIGHT_PIN));
	let switch_in = PolledButton::new(Pin::new(SWITCH_IN_PIN));
	
	loop {
		status_light.set_state(match switch_in.read_state() {
			ButtonState::Pressed => LightState::On,
			ButtonState::NotPressed => LightState::Off
		});
		thread::sleep_ms(50);
	}
}
