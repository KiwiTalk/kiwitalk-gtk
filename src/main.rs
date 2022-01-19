#![feature(once_cell, try_blocks)]

use std::path::PathBuf;
use std::lazy::{Lazy, SyncLazy};
use std::ops::Deref;
use std::sync::Arc;
use directories::ProjectDirs;
use gio::ApplicationFlags;
use gio::prelude::{ApplicationExt, ApplicationExtManual};
use gtk::Application;
use gtk::prelude::*;
use log::{debug, error, info};
use rand::{RngCore, SeedableRng};
use rand::rngs::StdRng;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::{Mutex, RwLock};
use tokio::task::JoinHandle;
use uuid::Uuid;
use kiwitalk::{Kiwitalk, KiwitalkEvent, KiwitalkHandle};
use talk_api_client::auth::AuthDeviceConfig;
use talk_api_client::auth::resources::LoginData;
use talk_loco_client::structs::openlink::OpenProfileType::Main;
use crate::component::ChannelEntry;
use crate::gui::Login;

mod login;
mod talk;
mod component;
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

const UUID: Lazy<Vec<u8>> = Lazy::new(|| {
    let mut uuid_path = PROJECT_DIRS.data_local_dir().to_path_buf();
    uuid_path.push("uuid.bin");

    let uuid: Option<Vec<u8>> = std::fs::read(&uuid_path).ok();

    match uuid {
        Some(uuid) => uuid,
        None => {
            info!("failed to load uuid from file, creating new one...");
            let mut uuid: Vec<u8> = vec![0u8; 64];
            StdRng::from_entropy().fill_bytes(&mut uuid);
            if let Err(e) = std::fs::write(uuid_path, &uuid) {
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
    AuthDeviceConfig::new_pc(whoami::devicename().into(), base64::encode(UUID.as_slice()).into())
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
        login::init(app);
        init(app);
        gui::Main::get().main_application_window.set_application(Some(app));
        tokio::spawn(async {
            if let Err(e) = try_login().await {
                error!("{}", e);
                gtk::glib::idle_add_once(|| Login::get().window.show_all());
            } else {
                gtk::glib::idle_add_once(|| gui::Main::get().main_application_window.show_all());
            }
        });
    });
    application.run();
}


pub fn init(app: &Application) {
    gui::Main::get().main_application_window.set_application(Some(app));


    gui::Main::get().chats_list_box.set_sort_func(Some(Box::new(|a, b| {
        let a: ChannelEntry = a.child().unwrap().downcast().unwrap();
        let b: ChannelEntry = b.child().unwrap().downcast().unwrap();
        a.last_update().duration_since(b.last_update()).as_secs() as _
    })));
}

static KIWITALK: SyncLazy<RwLock<Option<KiwitalkHandle>>> = SyncLazy::new(Default::default);

static EVENT_HANDLER: SyncLazy<Mutex<Option<JoinHandle<()>>>> = SyncLazy::new(Default::default);


async fn try_login() -> Result<(), Error> {
    info!("trying to login using stored token");
    let login_data: LoginData = serde_yaml::from_slice(
        &std::fs::read(&*LOGIN_DATA_PATH)?
    )?;
    let (kiwitalk_handle, channel) = Kiwitalk::new(login_data, auth_device_config()).await?;
    let kiwitalk_clone = kiwitalk_handle.inner.clone();
    if let Some(v) = KIWITALK.write().await.replace(kiwitalk_handle) {
        tokio::spawn(async move {
            //for x in v.inner.stop().await {
            //    error!("error after stopping loco instance: {}", x);
            //}
        });
    }
    std::mem::replace(&mut *EVENT_HANDLER.lock().await, Some(tokio::spawn(async move {
        tokio::select! {
            _ = kiwitalk_clone.wait_stop() => (),
            result = event_handler(channel, &kiwitalk_clone) => {
                if let Err(e) = result {
                    log::error!("error on event handler: {}", e);
                }
            }
        }
    }))).map(|x| x.abort());
    Ok(())
}

async fn event_handler(mut event_channel: UnboundedReceiver<KiwitalkEvent>, kiwitalk: &Arc<Kiwitalk>) -> Result<(), Error>{
    while kiwitalk.is_running().await {
        let event = event_channel.recv().await.unwrap();
        debug!("recv event: {:#?}", event);
        match event {
            KiwitalkEvent::ChatListUpdate(chat_list) => {
                let entries = gui::Main::get().chats_list_box.children();
                for x in entries {
                    gui::Main::get().chats_list_box.remove(&x);
                }
                'a: for channel_list in chat_list.chat_datas {
                    let entry = ChannelEntry::new(channel_list);
                    gui::Main::get().chats_list_box.add(&entry);
                }
                gui::Main::get().chats_list_box.invalidate_sort();
            }
        }
    }
    Ok(())
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