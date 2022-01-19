use std::ops::Deref;
use std::time::Instant;
use gio::glib::Object;
use gtk::prelude::*;
use talk_loco_client::structs::channel_info::ChannelListData;
use talk_loco_client::structs::chat::{ChatId, Chatlog};
use gtk::subclass::prelude::*;

mod imp {
	use std::cell::RefCell;
	use std::rc::Rc;
	use std::time::Instant;
	use gtk::subclass::prelude::*;
	use gtk::glib;
	use talk_loco_client::structs::channel_info::ChannelListData;
	use talk_loco_client::structs::chat::ChatId;
	use crate::gui;

	pub struct ChannelEntry {
		pub builder: gui::ChannelEntry,
		pub chat_id: RefCell<ChatId>,
		pub last_update: RefCell<Instant>
	}

	#[glib::object_subclass]
	impl ObjectSubclass for ChannelEntry {
		const NAME: &'static str = "ChatroomEntry";
		type Type = super::ChannelEntry;
		type ParentType = gtk::Bin;
	}

	impl ChannelEntry {
	}

	impl ObjectImpl for ChannelEntry {

	}

	impl WidgetImpl for ChannelEntry {}

	impl ContainerImpl for ChannelEntry {}

	impl BinImpl for ChannelEntry {}

	impl Default for ChannelEntry {
		fn default() -> Self {
			Self {
				builder: Default::default(),
				chat_id: RefCell::new(Default::default()),
				last_update: RefCell::new(Instant::now())
			}
		}
	}
}

gtk::glib::wrapper! {
    pub struct ChannelEntry(ObjectSubclass<imp::ChannelEntry>)
        @extends gtk::Box, gtk::Widget, gtk::Container,
        @implements gtk::Actionable, gtk::Buildable;
}

impl ChannelEntry {
	pub fn new(channel_list: ChannelListData) -> Self {
		let obj: Self = Object::new(&[]).unwrap();
		let inner = obj.inner();
		obj.set_child(Some(&inner.builder.body));
		obj.update(channel_list);
		obj
	}

	pub fn update(&self, channel_list: ChannelListData) {
		let inner = self.inner();
		inner.chat_id.replace(channel_list.id);
		if let Some(Chatlog { message: Some(message), .. }) = channel_list.chatlog {
			inner.builder.last_message_label.set_text(&message)
		}
		inner.builder.unread_label.set_text(&format!("{}", channel_list.unread_count));
		inner.last_update.replace(Instant::now());
	}

	pub fn inner(&self) -> &imp::ChannelEntry {
		imp::ChannelEntry::from_instance(self)
	}

	pub fn id(&self) -> ChatId {
		self.inner().chat_id.borrow().deref().clone()
	}

	pub fn last_update(&self) -> Instant {
		self.inner().last_update.borrow().deref().clone()
	}
}