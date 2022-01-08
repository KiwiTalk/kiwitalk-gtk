#![feature(once_cell, try_blocks)]

use std::path::PathBuf;
use std::lazy::Lazy;
use std::sync::Arc;
use directories::ProjectDirs;
use gio::ApplicationFlags;
use gio::prelude::{ApplicationExt, ApplicationExtManual};
use gtk::prelude::GtkWindowExt;
use log::{error, info};
use uuid::Uuid;
use kiwitalk::Kiwitalk;
use talk_api_client::auth::AuthDeviceConfig;
use talk_api_client::auth::resources::LoginData;
use crate::gui::Login;

mod login;
mod talk;
pub mod gui;

const PROJECT_DIRS: Lazy<ProjectDirs> = Lazy::new(|| {
    let dirs = ProjectDirs::from("org", "Kiwitalk", "Kiwitalk GTK").unwrap();
    let res: std::io::Result<()> = try {
        std::fs::create_dir_all(dirs.data_dir())?;
        std::fs::create_dir_all(dirs.data_local_dir())?;
    };
    res.map_err(|e| error!("error while creating directory: {}", e)).ok();
    dirs
});

const UUID: Lazy<Uuid> = Lazy::new(|| {
    let mut uuid_path = PROJECT_DIRS.data_local_dir().to_path_buf();
    uuid_path.push("uuid.bin");

    let uuid: Option<Uuid> = try {
        Uuid::from_slice(&std::fs::read(&uuid_path).ok()?).ok()?
    };
    match uuid {
        Some(uuid) => uuid,
        None => {
            info!("failed to load uuid from file, creating new one...");
            let uuid = Uuid::new_v4();
            if let Err(e) = std::fs::write(uuid_path, uuid.as_bytes()) {
                error!("error while writing uuid to file: {}", e);
            }
            uuid
        }
    }
});

const LOGIN_DATA_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = PROJECT_DIRS.data_dir().to_path_buf();
    path.push("login_data.yml");
    path
});

fn auth_device_config() -> AuthDeviceConfig<'static> {
    AuthDeviceConfig::new_pc(whoami::devicename().into(), base64::encode(UUID.to_hyphenated().to_string()).into())
}

thread_local! {
    static APPLICATION: gtk::Application = gtk::Application::new(Some("org.kiwitalk.Kiwitalk-GTK"), ApplicationFlags::default());
}

fn application() -> gtk::Application {
    APPLICATION.with(|application| application.clone())
}

#[tokio::main(worker_threads = 1)]
async fn main() {
    simple_logger::SimpleLogger::new().init().unwrap();
    gtk::init().expect("error while initializing gtk4");
    let application = application();
    application.connect_activate(move |app| {
        Login::get().window.set_application(Some(app));
        login::init();
    });
    application.run();
}

async fn try_login() -> Result<Arc<Kiwitalk>, Error> {
    info!("trying to login using stored token");
    let login_data: LoginData = serde_yaml::from_slice(
        &std::fs::read(&*LOGIN_DATA_PATH)?
    )?;
    Ok(Kiwitalk::new(login_data, auth_device_config()).await?)
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("{0}")]
    Kiwitalk(#[from] kiwitalk::Error),
    #[error("{0}")]
    StdIo(#[from] std::io::Error),
    #[error("{0}")]
    Yaml(#[from] serde_yaml::Error)
}