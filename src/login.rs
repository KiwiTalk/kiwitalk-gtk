use std::lazy::Lazy;
use std::time::Duration;
use email_address::EmailAddress;
use gtk::gdk::Event;
use gtk::prelude::*;
use log::{debug, error, info};
use talk_api_client::agent::TalkApiAgent;
use talk_api_client::ApiRequestError;
use talk_api_client::auth::{AccountLoginForm, AuthClientConfig, LoginMethod, Status, TalkAuthClient};
use talk_api_client::auth::resources::LoginData;
use talk_api_client::auth::xvc::default::Win32XVCHasher;
use talk_api_client::response::TalkStatusResponse;
use crate::gui::Login;
use crate::{auth_device_config, LOGIN_DATA_PATH, try_login};

fn auth_client_config() -> AuthClientConfig<'static> {
	AuthClientConfig::new_const(
		auth_device_config(),
		"ko",
		"3.2.8",
		TalkApiAgent::Win32("10.0".into())
	)
}

const AUTH_CLIENT: Lazy<TalkAuthClient<Win32XVCHasher>> = Lazy::new(|| {
	TalkAuthClient::new(auth_client_config(), XVC_HASHER)
});

const XVC_HASHER: Win32XVCHasher = Win32XVCHasher::new_const("JAYDEN", "JAYMOND");

pub fn init() {

	let gui = Login::get();
	fn hide_on_delete(w: &impl WidgetExtManual, _: &Event) -> Inhibit {
		w.hide_on_delete()
	}


	gui.wrong_password_message_dialog.set_parent(&gui.window);


	gui.register_device_dialog.connect_delete_event(hide_on_delete);
	gui.register_device_dialog.set_parent(&gui.window);
	gui.register_device_code_entry.connect_activate(|_| register_device());
	gui.register_device_code_send_button.connect_clicked(|_| request_passcode());
	gui.register_device_apply_button.connect_clicked(|_| register_device());
	gui.register_device_cancel_button.connect_clicked(|_| {
		Login::get().register_device_dialog.close()
	});


	gui.wrong_confirm_code_message_dialog.connect_delete_event(hide_on_delete);
	gui.wrong_confirm_code_message_dialog.set_parent(&gui.window);


	gui.too_many_confirm_request_message_dialog.connect_delete_event(hide_on_delete);
	gui.too_many_confirm_request_message_dialog.set_parent(&gui.register_device_dialog);


	gui.device_register_success_message_dialog.connect_delete_event(hide_on_delete);
	gui.device_register_success_message_dialog.set_parent(&gui.register_device_dialog);


	gui.id_not_found_message_dialog.connect_delete_event(hide_on_delete);
	gui.id_not_found_message_dialog.set_parent(&gui.window);

	gui.apply_button.connect_clicked(|_| on_apply());
	gui.email_entry.connect_activate(|_| on_apply());
	gui.email_entry.connect_activate(|_| on_apply());
	gui.password_entry.connect_activate(|_| on_apply());
	gui.cancel_button.connect_clicked(|_| {
		Login::get().window.close()
	});

	gui.window.show_all();
}

fn login_form() -> AccountLoginForm<'static> {
	let email = Login::get().email_entry.text().as_str().to_owned();
	let password = Login::get().password_entry.text().as_str().to_owned();
	AccountLoginForm {
		email: email.into(),
		password: password.into()
	}
}

fn on_apply() {
	let login_form = login_form();
	let force_login = Login::get().force_check_button.is_active();

	if !EmailAddress::is_valid(&login_form.email) {
		Login::get().email_entry.grab_focus();
		return;
	}

	if login_form.password.is_empty() {
		Login::get().password_entry.grab_focus();
		return;
	}

	tokio::task::spawn(async move { if let Result::<(), ApiRequestError>::Err(e) = try {
		info!("sending login request...");

		let response = AUTH_CLIENT.login(&LoginMethod::Account(login_form.clone()), force_login).await?;
		info!("login request sent");
		handle_response(response).await;
	} {
		error!("error while logging in: {}", e);
		debug!("{:#?}", e);
	}});
}

async fn handle_response(response: TalkStatusResponse<LoginData>) {
	debug!("response: {:#?}", response);
	match response.status {
		Status::Success => {
			if let Some(data) = response.data {
				std::fs::write(
					LOGIN_DATA_PATH.as_path(),
					serde_yaml::to_string(&data).unwrap()
				).map_err(|e| error!("error while saving login data: {}", e)).ok();
				//todo start kiwitalk
				//gtk::glib::idle_add_once(|| Login::get().window.close());
				let result = try_login().await;
				if let Err(e) = result {
					println!("err: {:#?}", e);
				}
			}
		},
		Status::DeviceNotRegistered => {
			gtk::glib::idle_add_once(|| {
				request_passcode();
				Login::get().register_device_dialog.show_all()
			});
		}
		_ => debug!("unhandled response while login: {:#?}", response)
	}
}

fn request_passcode() {
	let login_form = login_form();
	Login::get().register_device_code_send_button.set_sensitive(false);
	let text_original = Login::get().register_device_code_send_button.label().map(|x| x.as_str().to_owned());
	info!("requesting passcode...");
	tokio::spawn(async move {
		if let Err(e) = AUTH_CLIENT.request_passcode(&login_form).await {
			error!("error while requesting passcode: {}", e);
			debug!("{:#?}", e);
		}
		for i in (1..=60).rev() {
			gtk::glib::idle_add_once(move || {
				Login::get().register_device_code_send_button.set_label(&format!("{} secs left", i))
			});
			tokio::time::sleep(Duration::from_secs(1)).await;
		}
		if let Some(text_original) = text_original {
			gtk::glib::idle_add_once(move || {
				Login::get().register_device_code_send_button.set_label(&text_original);
				Login::get().register_device_code_send_button.set_sensitive(true);
			});
		}
	});
}

fn register_device() {
	let login_form = login_form();
	let passcode = Login::get().register_device_code_entry.text().as_str().to_string();
	if passcode.len() != 4 || passcode.parse::<u16>().is_err() {
		Login::get().register_device_code_entry.grab_focus();
		return;
	}
	let permanent = Login::get().keep_login_check_button.is_active();
	tokio::spawn(async move {
		match AUTH_CLIENT.register_device(&passcode, &login_form, permanent).await {
			Ok(response) => {
				gtk::glib::idle_add_once(move || match response.status {
					Status::Success => {
						Login::get().register_device_dialog.close();
						Login::get().device_register_success_message_dialog.show_all();
					},
					Status::DeviceRegisterFailed => Login::get().wrong_password_message_dialog.show_all(),
					_ => debug!("unhandled response while device register: {:#?}", response)
				});
			}
			Err(e) => {
				error!("error while registering device: {}", e);
				debug!("{:#?}", e);
			}
		}
	});
}