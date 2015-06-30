mod pi;

use pi::gpio::Pin;
use pi::light::{Light, LightState};
use pi::polled_button::PolledButton;

const status_light_pin : usize = 5;
const switch_in_pin : usize = 6;

fn main() {
	let status_light = Light::new(Pin::new(status_light_pin));
	let switch_in = PolledButton::new(Pin::new(switch_in_pin));
	
	loop {
		status_light.set_state(match switch_in.read_state() {
			Pressed => On,
			NotPressed => Off
		});
	}
}
