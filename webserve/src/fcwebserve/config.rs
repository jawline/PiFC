use rustc_serialize::json;
use std::fs::File;
use std::io::Read;
use std::string::String;

#[derive(RustcEncodable, RustcDecodable)]
pub struct Config {
    pub enabled: bool,
    pub static_dir: String,
    pub api_address: String,
    pub static_address: String
}

impl Config {
    fn read_config_file(base_file: &str) -> String {
        let mut result = String::new();
        
        if let Err(_) = File::open(base_file).unwrap().read_to_string(&mut result) {
            panic!("Could not read from FCConfig file {}", base_file);
        }
    
        return result;
    }
    
    pub fn load(base_file: &str) -> Config {
        let text = Config::read_config_file(base_file);
        return json::decode(&text).unwrap();
    }
}