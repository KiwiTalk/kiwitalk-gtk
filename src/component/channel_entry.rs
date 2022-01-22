use std::ops::Deref;
use std::time::Instant;
use gio::glib::Object;
use gtk::gdk::keys::constants::ch;
use gtk::prelude::*;
use talk_loco_client::structs::channel_info::ChannelListData;
use talk_loco_client::structs::chat::Chatlog;
use gtk::subclass::prelude::*;
use talk_loco_client::response::chat::Msg;
use talk_loco_client::structs::ids::ChannelId;
use crate::component::Channel;
use crate::gui;

mod imp {
	use std::cell::RefCell;
	use std::rc::Rc;
	use std::time::Instant;
	use gtk::subclass::prelude::*;
	use gtk::glib;
	use talk_loco_client::structs::channel_info::ChannelListData;
	use talk_loco_client::structs::ids::ChannelId;
	use crate::component::Channel;
	use crate::gui;

	pub struct ChannelEntry {
		pub builder: gui::ChannelEntry,
		pub last_update: RefCell<Instant>,
		pub channel: RefCell<Channel>
	}

	#[glib::object_subclass]
	impl ObjectSubclass for ChannelEntry {
		const NAME: &'static str = "ChannelEntry";
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
				last_update: RefCell::new(Instant::now()),
				channel: Default::default()
			}
		}
	}
}

gtk::glib::wrapper! {
    pub struct ChannelEntry(ObjectSubclass<imp::ChannelEntry>)
        @extends gtk::Bin, gtk::Widget, gtk::Container,
        @implements gtk::Actionable, gtk::Buildable;
}

impl ChannelEntry {
	pub fn new(channel_list: &ChannelListData, channel: Channel) -> Self {
		let obj: Self = Object::new(&[]).unwrap();
		let inner = obj.inner();
		inner.channel.replace(channel);
		obj.set_child(Some(&inner.builder.body));
		obj.update(channel_list);
		obj
	}

	pub fn update(&self, channel_list: &ChannelListData) {
		let inner = self.inner();
		inner.channel.borrow().inner().channel_id.replace(channel_list.id);
		if let Some(Chatlog { message: Some(message), .. }) = &channel_list.chatlog {
			inner.builder.last_message_label.set_text(message)
		}
		inner.builder.unread_label.set_text(&format!("{}", channel_list.unread_count));
		if let Some(v) = &channel_list.icon_user_nicknames {
			inner.builder.channel_name_label.set_text(&v.join(","))
		}
		inner.last_update.replace(Instant::now());
	}

	pub fn on_msg(&self, msg: Msg) {
		if let Some(chat_log) = &msg.chatlog {
			let unread: usize = self.inner().builder.unread_label.text().to_string().parse().unwrap();
			self.inner().builder.unread_label.set_text(&format!("{}", unread + 1));
			if let Some(message) = &chat_log.message {
				self.inner().builder.last_message_label.set_text(message);
			}
		}
		self.inner().channel.borrow().on_msg(msg)
	}

	pub fn inner(&self) -> &imp::ChannelEntry {
		imp::ChannelEntry::from_instance(self)
	}

	pub fn id(&self) -> ChannelId {
		self.inner().channel.borrow().inner().channel_id.borrow().clone()
	}

	pub fn last_update(&self) -> Instant {
		self.inner().last_update.borrow().deref().clone()
	}

	pub fn on_selected(&self) {
		if let Some(child) = gui::Main::get().main_viewport.child() {
			gui::Main::get().main_viewport.remove(&child);
		}
		gui::Main::get().main_viewport.set_child(Some(self.inner().channel.borrow().deref()));
		self.inner().channel.borrow().show();
	}
}