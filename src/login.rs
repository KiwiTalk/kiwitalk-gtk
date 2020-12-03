use gtk::prelude::{BuilderExtManual, WidgetExtManual};
use gtk::{WidgetExt, Inhibit, EntryExt, DialogFlags, MessageType, ButtonsType, DialogExt, Dialog, GtkWindowExt};
use loco::internal::{LoginData, TokenClient, StatusCode, DeviceRegisterData, LoginAccessData};
use uuid::Uuid;
use std::fs::File;
use crate::{app_home_dir, CONFIG};
use config::Config;
use loco::internal::agent::Os;
use std::io::Write;
use std::env::Args;
use std::rc::Rc;

pub fn init() {
	let builder = gtk::Builder::from_string(include_str!("gui/login.glade"));
	let window: gtk::Window = builder.get_object("window").unwrap();
	let icon_image: gtk::Image = builder.get_object("icon_image").unwrap();
	let email_entry: gtk::Entry = builder.get_object("email_entry").unwrap();
	let password_entry: gtk::Entry = builder.get_object("password_entry").unwrap();
	let keep_login_check_button: gtk::CheckButton = builder.get_object("keep_login_check_button").unwrap();
	let apply_button: gtk::Button = builder.get_object("apply_button").unwrap();

	let register_device_dialog: gtk::Dialog = builder.get_object("register_device_dialog").unwrap();
	let register_device_code_entry: gtk::Entry = builder.get_object("register_device_code_entry").unwrap();
	let register_device_code_send_button: gtk::Button = builder.get_object("register_device_code_send_button").unwrap();
	let register_device_apply_button: gtk::Button = builder.get_object("register_device_apply_button").unwrap();
	let register_device_cancel_button: gtk::Button = builder.get_object("register_device_cancel_button").unwrap();

	register_device_dialog.connect_delete_event(| dialog, _ | dialog.hide_on_delete());

	let id_not_found_message_dialog: gtk::MessageDialog = builder.get_object("id_not_found_message_dialog").unwrap();
	id_not_found_message_dialog.connect_delete_event(| dialog, _ | dialog.hide_on_delete());

	let wrong_password_message_dialog: gtk::MessageDialog = builder.get_object("wrong_password_message_dialog").unwrap();
	wrong_password_message_dialog.connect_delete_event(| dialog, _ | dialog.hide_on_delete());

	let too_many_confirm_request_message_dialog: gtk::MessageDialog = builder.get_object("too_many_confirm_request_message_dialog").unwrap();
	too_many_confirm_request_message_dialog.connect_delete_event(| dialog, _ | dialog.hide_on_delete());

	let device_register_success_message_dialog: gtk::MessageDialog = builder.get_object("device_register_success_message_dialog").unwrap();
	let register_device_dialog_c = register_device_dialog.clone();
	device_register_success_message_dialog.connect_delete_event(move | dialog, _ | {
		register_device_dialog_c.close();
		dialog.hide_on_delete()
	});

	let wrong_confirm_code_message_dialog: gtk::MessageDialog = builder.get_object("wrong_confirm_code_message_dialog").unwrap();


	wrong_confirm_code_message_dialog.connect_delete_event(| dialog, _ | dialog.hide_on_delete());


	CONFIG.write().unwrap().set_default("uuid", Uuid::new_v4().to_string()).unwrap();
	let uuid = CONFIG.read().unwrap().get_str("uuid").unwrap();

	println!("{}", &uuid);

	let login_error_handle = Rc::new(move | login_access_data: &LoginAccessData | {
		match login_access_data.status {
			_ => {} //TODO
		}
	});

	let mut login = {
		let window_c = window.clone();
		let email_entry_c = email_entry.clone();
		let password_entry_c = password_entry.clone();
		let keep_login_check_button_c = keep_login_check_button.clone();
		let token_client = Rc::new(TokenClient::new(Os::Win32));


		Rc::new(move || {
			let login_data = LoginData::new(
				email_entry_c.get_buffer().get_text(),
				password_entry_c.get_buffer().get_text(),
				&uuid,
				whoami::hostname(),
				"10.0".to_owned(),
				keep_login_check_button_c.get_hexpand(),
				false
			);
			let login_access_data = token_client.request_login(&login_data).unwrap();


			match &login_access_data.status.unwrap() {
				StatusCode::Success => {
					let mut output_dir = app_home_dir();
					output_dir.push("login_access_data.yml");
					let mut file = File::create(output_dir).unwrap();
					file.write(serde_yaml::to_string(&login_access_data).unwrap().as_bytes());
				},
				StatusCode::DeviceNotRegistered => {
					//token_client.request_passcode(&login_data);

					let token_client_c = token_client.clone();
					let login_data_c = login_data.clone();
					register_device_code_send_button.connect_button_release_event(move | _, _ | {
						token_client_c.request_passcode(&login_data_c);
						Inhibit(false)
					});

					let mut token_client_c = token_client.clone();
					let mut login_error_handle_c = login_error_handle.clone();
					let register_device_code_entry_c = register_device_code_entry.clone();
					let device_register_success_message_dialog_c = device_register_success_message_dialog.clone();
					let wrong_confirm_code_message_dialog_c = wrong_confirm_code_message_dialog.clone();
					let register_device = Rc::new(move || {
						let login_access_data = token_client_c.register_device(&DeviceRegisterData::new(
							login_data.clone(),
							register_device_code_entry_c.get_buffer().get_text()
						)).unwrap();
						println!("{:?}", login_access_data);
						match &login_access_data.status.unwrap() {
							StatusCode::Success =>
								device_register_success_message_dialog_c.show_all(),
							StatusCode::WrongConfirmCode =>
								wrong_confirm_code_message_dialog_c.show_all(),
							_ => login_error_handle_c(&login_access_data)
						}
					});
					let register_device_c = register_device.clone();
					register_device_apply_button.connect_button_release_event(move | _, _ | {
						register_device_c();
						Inhibit(false)
					});
					register_device_code_entry.connect_activate(move | _ | register_device());
					register_device_dialog.show_all();
				},
				StatusCode::WrongConfirmCode => {}, // cannot be happen
				StatusCode::CannotFindId => id_not_found_message_dialog.show_all(),
				StatusCode::WrongPassword => wrong_password_message_dialog.show_all(),
				StatusCode::TooManyConfirmRequest => too_many_confirm_request_message_dialog.show_all(),
				_ => login_error_handle(&login_access_data)
			}
		})
	};

	icon_image.connect_draw(| image, context | {
		//TODO
		Inhibit(false)
	});

	let login_c = login.clone();
	email_entry.connect_activate(move | entry | login_c());
	let login_c = login.clone();
	password_entry.connect_activate(move | entry | login_c());
	apply_button.connect_button_release_event(move |button, event_button | {
		login();
		Inhibit(false)
	});
	window.show_all();
}