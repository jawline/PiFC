pub mod config;
pub mod core;
pub mod core_logic;
pub mod sensors;
pub mod motors;
pub mod configled;
pub mod configbutton;

pub use fccore::core::Core;
use std::thread::{JoinHandle};
use std::sync::{Arc, Mutex};

const TAG: &'static str = "fccore";

/**
 * create a core instance and a core logic thread and return a mutex and handle to them
 */
pub fn spawn_fc(base_cfg_path : &str) -> (Arc<Mutex<Core>>, JoinHandle<()>) {
    let core = Arc::new(Mutex::new(Core::new(base_cfg_path)));
    let handle = core_logic::start_logic_thread(&core);
    core.lock().unwrap().log_mut().add(TAG, "done spawning core");
    return (core, handle);
}
