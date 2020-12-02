use gtk::prelude::BuilderExtManual;
use gtk::{WidgetExt, Inhibit, EntryExt, DialogFlags};
use loco::internal::{LoginData, TokenClient, StatusCode};
use uuid::Uuid;
use std::fs::File;
use crate::{app_home_dir, CONFIG};
use config::Config;
use loco::internal::agent::Os;
use std::io::Write;

pub fn init() {
	let builder = gtk::Builder::from_string(include_str!("gui/login.glade"));
	let window: gtk::Window = builder.get_object("window").unwrap();
	let icon_image: gtk::Image = builder.get_object("icon_image").unwrap();
	let email_entry: gtk::Entry = builder.get_object("email_entry").unwrap();
	let password_entry: gtk::Entry = builder.get_object("password_entry").unwrap();
	let keep_login_check_button: gtk::CheckButton = builder.get_object("keep_login_check_button").unwrap();

	CONFIG.write().unwrap().set_default("uuid", Uuid::new_v4().to_string());
	let uuid = CONFIG.read().unwrap().get_str("uuid").unwrap();

	let login = {
		let window_c = window.clone();
		let email_entry_c = email_entry.clone();
		let password_entry_c = password_entry.clone();
		let keep_login_check_button_c = keep_login_check_button.clone();
		let token_client = TokenClient::new(Os::Win32);

		let id_not_found_message_dialog: gtk::MessageDialog = builder.get_object("id_not_found_message_dialog").unwrap();
		move || {
			let login_data = LoginData::new(
				email_entry_c.get_buffer().get_text(),
				password_entry_c.get_buffer().get_text(),
				&uuid,
				whoami::hostname(),
				whoami::os(),
				keep_login_check_button_c.get_hexpand(),
				false
			);
			let request = token_client.request_login(&login_data).unwrap();


			match &request.status.unwrap() {
				StatusCode::Success => {
					let mut output_dir = app_home_dir();
					output_dir.push("login_access_data.yml");
					let mut file = File::create(output_dir).unwrap();
					file.write(serde_yaml::to_string(&request).unwrap().as_bytes());
				},
				StatusCode::CannotFindId => {
					id_not_found_message_dialog.show_all();
				},
				StatusCode::WrongPassword => {
					
				}
			}
		}
	};

	icon_image.connect_draw(| image, context | {
		//TODO
		Inhibit(false)
	});

	email_entry.connect_activate(| a | {
		println!("act");
	});
	window.show_all();
}