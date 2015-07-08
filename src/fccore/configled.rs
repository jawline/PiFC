use physical::gpio::Pin;
use physical::light::{Light, LightState};
use fccore::config::Light;

pub struct ConfigLed {
    enabled: bool,
    light: Light
}

impl ConfigLed {
    pub fn new(led: &Light) -> ConfigLed {
        ConfigLed{
            enabled: led.use_switch,
            light: Light::new(Pin::new(led.pin))
        }
    }

    pub fn set(&self, state: bool) {
        if self.enabled {
            self.light.set_state(match state {
                true => LightState::On,
                false => LightState::Off
            });
        };
    }
}
