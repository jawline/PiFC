pub mod fcconfig;
pub mod fccore;
pub mod fccore_logic;
pub mod log;
pub mod sensors;
pub mod motors;
pub mod configbutton;

pub use fccore::fccore::Core;
use std::thread::{JoinHandle};
use std::sync::{Arc, Mutex};

const TAG : &'static str = "fccore";

/**
 * create a core instance and a core logic thread and return a mutex and handle to them
 */
pub fn spawn_fc(base_cfg_path : &str) -> (Arc<Mutex<FCCore>>, JoinHandle<()>) {
    let core = Arc::new(Mutex::new(FCCore::new(base_cfg_path)));
    let handle = fccore_logic::start_logic_thread(&core);
    core.lock().unwrap().log_mut().add(TAG, "done spawning core");
    return (core, handle);
}
