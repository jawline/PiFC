extern crate rustc_serialize;
extern crate time;
extern crate iron;

mod physical;
mod fccore;
mod fcwebserve;

const BASE_CFG_FILE : &'static str = "./assets/base.cfg";

fn main() {
	let (core, handle) = fccore::spawn_fc(BASE_CFG_FILE);

	if core.lock().unwrap().config().fc_webserve_enabled {
		fcwebserve::spawn(&core);
	} else {
		println!("WebServe disabled");
	}

	if handle.join().is_err() {
		panic!("Error in FCCore thread");
	}
}
