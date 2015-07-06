use rustc_serialize::json;
use std::fs::File;
use std::io::{Read};
use std::string::ToString;

#[derive(RustcEncodable, RustcDecodable)]
pub struct FCConfig {
    pub fc_webserve_enabled : bool,
    pub fc_webserve_port : usize,
    pub status_pin : usize,
    pub arm_switch_pin : usize
}

impl FCConfig {
    fn read_config_file(base_file : &str) -> String {
        let mut result = String::new();
        
        if let Err(_) = File::open(base_file).unwrap().read_to_string(&mut result) {
            panic!("Could not read from FCConfig file {}", base_file);
        }
    
        return result;
    }
  
    pub fn load(base_file : &str) -> FCConfig {
        let text = FCConfig::read_config_file(base_file);
        return json::decode(&text).unwrap();
    }
}

impl ToString for FCConfig {
    fn to_string(&self) -> String {
        json::encode(self).unwrap()
    }
}
