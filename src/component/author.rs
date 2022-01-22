use std::borrow::Borrow;
use std::future::Future;
use std::ops::Deref;
use std::time::Instant;
use gio::glib::Object;
use gtk::{Align, Orientation};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use log::error;
use kiwitalk::Error;
use talk_loco_client::response::chat::Msg;
use talk_loco_client::structs::chat::Chatlog;
use talk_loco_client::structs::ids::{ChannelId, UserId};
use talk_loco_client::structs::user::UserVariant;
use crate::{gui, KIWITALK};

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

	pub struct Author {
		pub author: RefCell<UserId>,
	}

	#[glib::object_subclass]
	impl ObjectSubclass for Author {
		const NAME: &'static str = "Author";
		type Type = super::Author;
		type ParentType = gtk::Box;
	}

	impl Author {
	}

	impl ObjectImpl for Author {

	}

	impl WidgetImpl for Author {}

	impl ContainerImpl for Author {}

	impl BoxImpl for Author {}

	impl Default for Author {
		fn default() -> Self {
			Self {
				author: Default::default()
			}
		}
	}
}

gtk::glib::wrapper! {
    pub struct Author(ObjectSubclass<imp::Author>)
        @extends gtk::Box, gtk::Widget, gtk::Container,
        @implements gtk::Actionable, gtk::Buildable, gtk::Orientable;
}

impl Author {
	pub fn new(author: UserId, name: &str) -> Self {
		let obj: Self = Object::new(&[]).unwrap();
		obj.set_orientation(Orientation::Vertical);
		obj.set_expand(true);
		let inner = obj.inner();
		inner.author.replace(author);
		let author_label = gtk::Label::new(Some(name));
		author_label.set_margin(5);
		author_label.set_halign(Align::Start);
		obj.add(&author_label);
		obj.add(&gtk::Separator::new(Orientation::Horizontal));
		obj
	}

	pub fn author(&self) -> UserId {
		self.inner().author.borrow().deref().clone()
	}

	pub fn inner(&self) -> &imp::Author {
		imp::Author::from_instance(self)
	}
}