extern crate fccore;
extern crate fcwebserve;

const BASE_CFG_FILE: &'static str = "./configs/base.cfg";
const WEBSERVE_CFG_FILE: &'static str = "./configs/webserve.cfg";

fn main() {
	let (core, handle) = fccore::spawn_fc(BASE_CFG_FILE);
	fcwebserve::spawn(&core, WEBSERVE_CFG_FILE);

	if handle.join().is_err() {
		panic!("Error in FCCore thread");
	}
}
