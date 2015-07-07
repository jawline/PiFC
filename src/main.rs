extern crate rustc_serialize;
extern crate time;
extern crate iron;

mod physical;
mod fccore;
mod fcwebserve;

const BASE_CFG_FILE : &'static str = "./configs/base.cfg";
const TAG : &'static str = "main";

fn main() {
	let (core, handle) = fccore::spawn_fc(BASE_CFG_FILE);

	if core.lock().unwrap().config().fc_webserve_enabled {
		fcwebserve::spawn(&core);
	} else {
		core.lock().unwrap().log_mut().add(TAG, "webserve disabled by config");
	}

	if handle.join().is_err() {
		panic!("Error in FCCore thread");
	}
}
