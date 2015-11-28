use piphysical::gpio::Pin;
use piphysical::button::{Button, ButtonState};
use piphysical::polled_button::PolledButton;
use fccore::config::Switch;

pub struct ConfigButton {
	enabled: bool,
	disabled_return_value: bool,
	button: PolledButton
}

impl ConfigButton {
	pub fn new(switch: &Switch) -> ConfigButton {
		ConfigButton {
			enabled: switch.use_switch,
			disabled_return_value: switch.disabled_return_value,
			button: PolledButton::new(Pin::new(switch.pin))
		}
	}

	pub fn read_state(&self) -> bool {
		match self.enabled {
			true => match self.button.read_state() {
				ButtonState::Pressed => true,
				ButtonState::NotPressed => false
			},
			false => self.disabled_return_value
		}
	}
}
