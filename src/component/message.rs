use std::future::Future;
use std::ops::Deref;
use std::time::Instant;
use gio::glib::Object;
use gtk::{Align, Orientation, STYLE_PROPERTY_BACKGROUND_COLOR};
use gtk::gdk::RGBA;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use log::error;
use kiwitalk::Error;
use talk_loco_client::response::chat::Msg;
use talk_loco_client::structs::chat::Chatlog;
use talk_loco_client::structs::ids::{ChannelId, UserId};
use talk_loco_client::structs::user::UserVariant;
use crate::gui;

mod imp {
	use std::cell::RefCell;
	use std::rc::Rc;
	use std::time::Instant;
	use gtk::subclass::prelude::*;
	use gtk::glib;
	use talk_loco_client::structs::chat::Chatlog;
	use talk_loco_client::structs::ids::{ChannelId, UserId};
	use talk_loco_client::structs::user::UserVariant;
	use crate::gui;

	pub struct Message {
		pub chat_log: RefCell<Chatlog>,
	}

	#[glib::object_subclass]
	impl ObjectSubclass for Message {
		const NAME: &'static str = "Message";
		type Type = super::Message;
		type ParentType = gtk::Box;
	}

	impl Message {
	}

	impl ObjectImpl for Message {

	}

	impl WidgetImpl for Message {}

	impl ContainerImpl for Message {}

	impl BoxImpl for Message {}

	impl Default for Message {
		fn default() -> Self {
			Self {
				chat_log: Default::default()
			}
		}
	}
}

gtk::glib::wrapper! {
    pub struct Message(ObjectSubclass<imp::Message>)
        @extends gtk::Box, gtk::Widget, gtk::Container,
        @implements gtk::Actionable, gtk::Buildable, gtk::Orientable;
}

impl Message {
	pub fn new(chat_log: Chatlog) -> Self {
		let obj: Self = Object::new(&[]).unwrap();
		obj.set_orientation(Orientation::Horizontal);
		let inner = obj.inner();
		let message_label = gtk::Label::new(chat_log.message.as_ref().map(|x| x as _));
		message_label.set_hexpand(true);
		message_label.set_halign(Align::Start);
		let time_label = gtk::Label::new(Some(&chat_log.send_at.to_string()));
		time_label.set_halign(Align::End);
		obj.add(&message_label);
		obj.add(&time_label);
		inner.chat_log.replace(chat_log);
		obj
	}

	pub fn chat_log(&self) -> Chatlog {
		self.inner().chat_log.borrow().deref().clone()
	}

	pub fn inner(&self) -> &imp::Message {
		imp::Message::from_instance(self)
	}
}