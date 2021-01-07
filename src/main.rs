#[macro_use]
extern crate lazy_static;

use dirs::home_dir;
use std::path::PathBuf;
use std::sync::RwLock;
use ezconfig::Config;
use std::fs;
use gio::{ApplicationFlags, ApplicationExt};
use gio::prelude::ApplicationExtManual;

mod login;
pub mod gui;

lazy_static! {
	pub static ref CONFIG: RwLock<Config> = {
	    let mut dir = app_home_dir();
	    dir.push("config.yml");
	    let mut config = Config::new(dir);
	    config.init();
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
    let application = gtk::Application::new(None, ApplicationFlags::default()).unwrap();
    application.connect_activate(move | application | {
        login::init(application);
    });
    application.run(&[]);
}
