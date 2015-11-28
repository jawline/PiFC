extern crate rustc_serialize;
extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate hyper;
extern crate simplelog;
extern crate fccore;

pub mod fcwebserve;
pub use fcwebserve::*;