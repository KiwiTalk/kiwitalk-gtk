use gtk::prelude::WidgetExtManual;
use gtk::{WidgetExt, Inhibit, EntryExt, GtkWindowExt, GtkApplicationExt};
use loco::internal::{LoginData, TokenClient, StatusCode, DeviceRegisterData, LoginAccessData};
use uuid::Uuid;
use std::fs::File;
use crate::{app_home_dir, CONFIG};
use loco::types::Os;
use std::io::Write;
use std::rc::Rc;
use serde_yaml::Value;
use crate::gui::Login;

pub fn init(application: &gtk::Application) {
	application.add_window(&Login::get().window);

	Login::get().cancel_button.connect_button_release_event(move | _, _ | {
		Login::get().window.close();
		Inhibit(false)
	});

	Login::get().register_device_dialog
		.connect_delete_event(| dialog, _ | dialog.hide_on_delete());
	Login::get().id_not_found_message_dialog
		.connect_delete_event(| dialog, _ | dialog.hide_on_delete());

	Login::get().wrong_password_message_dialog
		.connect_delete_event(| dialog, _ | dialog.hide_on_delete());

	Login::get().too_many_confirm_request_message_dialog
		.connect_delete_event(| dialog, _ | dialog.hide_on_delete());

	Login::get().device_register_success_message_dialog
		.connect_delete_event(move | dialog, _ | {
			Login::get().register_device_dialog.close();
			dialog.hide_on_delete()
		});

	Login::get().wrong_confirm_code_message_dialog
		.connect_delete_event(| dialog, _ | dialog.hide_on_delete());


	CONFIG.write().unwrap().set_default("uuid", Value::String(Uuid::new_v4().to_string()));
	CONFIG.write().unwrap().save();
	let uuid = CONFIG.read().unwrap().get("uuid").unwrap().as_str().unwrap().to_owned();

	println!("{}", &uuid);

	let login_error_handle = Rc::new(move | login_access_data: &LoginAccessData | {
		match login_access_data.status {
			_ => {} //TODO
		}
	});

	let login = {
		let token_client = Rc::new(TokenClient::new(Os::Win32));


		Rc::new(move || {
			let login_data = LoginData::new(
				Login::get().email_entry.get_buffer().get_text(),
				Login::get().password_entry.get_buffer().get_text(),
				&uuid,
				whoami::hostname(),
				"10.0".to_owned(),
				Login::get().keep_login_check_button.get_hexpand(),
				false
			);
			let login_access_data = token_client.request_login(&login_data).unwrap();


			match &login_access_data.status.unwrap() {
				StatusCode::Success => {
					let mut output_dir = app_home_dir();
					output_dir.push("login_access_data.yml");
					let mut file = File::create(output_dir).unwrap();
					file.write(serde_yaml::to_string(&login_access_data).unwrap().as_bytes()).unwrap();
				},
				StatusCode::DeviceNotRegistered => {
					token_client.request_passcode(&login_data).unwrap();

					let token_client_c = token_client.clone();
					let login_data_c = login_data.clone();
					Login::get().register_device_code_send_button
						.connect_button_release_event(move | _, _ | {
							token_client_c.request_passcode(&login_data_c).unwrap();
							Inhibit(false)
						});

					let token_client_c = token_client.clone();
					let login_error_handle_c = login_error_handle.clone();
					let register_device = Rc::new(move || {
						let login_access_data = token_client_c.register_device(&DeviceRegisterData::new(
							login_data.clone(),
							Login::get().register_device_code_entry.get_buffer().get_text()
						)).unwrap();
						println!("{:?}", login_access_data);
						match &login_access_data.status.unwrap() {
							StatusCode::Success =>
								Login::get().device_register_success_message_dialog.show_all(),
							StatusCode::WrongConfirmCode =>
								Login::get().wrong_confirm_code_message_dialog.show_all(),
							_ => login_error_handle_c(&login_access_data)
						}
					});
					let register_device_c = register_device.clone();
					Login::get().register_device_apply_button
						.connect_button_release_event(move | _, _ | {
							register_device_c();
							Inhibit(false)
						});
					Login::get().register_device_code_entry.connect_activate(move | _ | register_device());
					Login::get().register_device_dialog.show_all();
				},
				StatusCode::WrongConfirmCode => {}, // cannot be happen
				StatusCode::CannotFindId => Login::get().id_not_found_message_dialog.show_all(),
				StatusCode::WrongPassword => Login::get().wrong_password_message_dialog.show_all(),
				StatusCode::TooManyConfirmRequest => Login::get().too_many_confirm_request_message_dialog.show_all(),
				_ => login_error_handle(&login_access_data)
			}
		})
	};

	let login_c = login.clone();
	Login::get().email_entry.connect_activate(move | _ | login_c());
	let login_c = login.clone();
	Login::get().password_entry.connect_activate(move | _ | login_c());
	Login::get().apply_button.connect_button_release_event(move | _, _ | {
		login();
		Inhibit(false)
	});
	Login::get().window.show_all();
}