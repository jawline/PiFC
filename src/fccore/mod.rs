pub mod fcconfig;
pub mod fccore;
pub mod fccore_logic;

pub use fccore::fccore::FCCore;
use std::thread::{JoinHandle};
use std::sync::{Arc, Mutex};

pub fn spawn_core(base_cfg_path : &str) -> (Arc<Mutex<FCCore>>, JoinHandle<()>) {
	let core = Arc::new(Mutex::new(FCCore::new(base_cfg_path)));
	let handle = fccore_logic::start_logic_thread(&core);
	return (core, handle);
}