#[macro_use]
extern crate lazy_static;

use dirs::home_dir;
use std::path::PathBuf;
use std::sync::RwLock;
use ezconfig::Config;
use std::fs;

mod login;
pub mod gui;

lazy_static! {
	pub static ref CONFIG: RwLock<Config> = {
	    let mut dir = app_home_dir();
	    dir.push("config.yml");
	    let mut config = Config::new(dir);
	    config.load();
	    RwLock::new(config)
	};
}

fn app_home_dir() -> PathBuf {
    let mut dir = home_dir().unwrap();
    dir.push(".kiwitalk");
    if !dir.exists() {
        fs::create_dir(&dir).unwrap();
    }
    dir
}

fn main() {
    gtk::init().unwrap();
    login::init();
    gtk::main();
}
