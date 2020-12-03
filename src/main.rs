#[macro_use]
extern crate lazy_static;

use dirs::home_dir;
use std::path::PathBuf;
use std::sync::RwLock;
use config::Config;
use std::fs;

mod login;

lazy_static! {
	pub static ref CONFIG: RwLock<Config> = {
	    let config = RwLock::new(Config::default());
	    let mut dir = app_home_dir();
	    dir.push("config.yml");
	    if !dir.exists() {
	        fs::write(&dir, "[]").unwrap();
	    }
	    config.write().unwrap().merge(config::File::with_name(dir.to_str().unwrap()));
	    config
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
    gtk::init();
    login::init();
    gtk::main();
}
