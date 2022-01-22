use std::future::Future;
use std::ops::Deref;
use std::time::Instant;
use gio::glib::Object;
use gtk::ListBoxRow;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use log::error;
use kiwitalk::Error;
use talk_loco_client::response::chat::Msg;
use talk_loco_client::structs::channel_info::ChannelInfo;
use talk_loco_client::structs::chat::Chatlog;
use talk_loco_client::structs::ids::{ChannelId, UserId};
use talk_loco_client::structs::user::UserVariant;
use crate::{gui, KIWITALK};
use crate::component::{Author, Message};

mod imp {
	use std::cell::RefCell;
	use std::rc::Rc;
	use std::time::Instant;
	use gtk::subclass::prelude::*;
	use gtk::glib;
	use talk_loco_client::structs::channel_info::ChannelInfo;
	use talk_loco_client::structs::ids::ChannelId;
	use talk_loco_client::structs::user::UserVariant;
	use crate::gui;
	use std::default::Default;

	pub struct Channel {
		pub builder: gui::Channel,
		pub channel_id: RefCell<ChannelId>,
		pub members: RefCell<Vec<UserVariant>>,
		pub channel_info: RefCell<ChannelInfo>
	}

	#[glib::object_subclass]
	impl ObjectSubclass for Channel {
		const NAME: &'static str = "Channel";
		type Type = super::Channel;
		type ParentType = gtk::Bin;
	}

	impl Channel {
	}

	impl ObjectImpl for Channel {

	}

	impl WidgetImpl for Channel {}

	impl ContainerImpl for Channel {}

	impl BinImpl for Channel {}

	impl Default for Channel {
		fn default() -> Self {
			Self {
				builder: Default::default(),
				channel_id: RefCell::new(Default::default()),
				members: Default::default(),
				channel_info: RefCell::new(Default::default())
			}
		}
	}
}

gtk::glib::wrapper! {
    pub struct Channel(ObjectSubclass<imp::Channel>)
        @extends gtk::Bin, gtk::Widget, gtk::Container,
        @implements gtk::Actionable, gtk::Buildable;
}

impl Channel {
	pub fn new(channel_id: ChannelId, members: Vec<UserVariant>, channel_info: ChannelInfo) -> Self {
		let obj: Self = Object::new(&[]).unwrap();
		let inner = obj.inner();
		inner.channel_id.replace(channel_id);
		inner.members.replace(members);
		inner.builder.info_label.set_text(&format!("{} members", channel_info.display_members.len()));
		inner.channel_info.replace(channel_info);
		obj.set_child(Some(&inner.builder.body));
		obj
	}

	pub fn on_msg(&self, msg: Msg) {
		if let Some(chat_log) = msg.chatlog {
			let last = self.inner().builder.message_list_box.children().into_iter().last().map(|x| {
				let message: Message = x.downcast::<ListBoxRow>().unwrap().child().unwrap().downcast().unwrap();
				message.chat_log().author_id
			}).unwrap_or(UserId::default());
			if last != chat_log.author_id {
				let channel_info = self.inner().channel_info.borrow();
				let name = channel_info.display_members.iter().find(|x| x.user_id == chat_log.author_id).unwrap();
				let author = Author::new(chat_log.author_id, &name.nickname);
				self.inner().builder.message_list_box.add(&author);
				author.show_all();
			}
			let message = Message::new(chat_log);
			self.inner().builder.message_list_box.add(&message);
			message.show_all();
		}
	}

	pub fn id(&self) -> ChannelId {
		self.inner().channel_id.borrow().deref().clone()
	}

	pub fn inner(&self) -> &imp::Channel {
		imp::Channel::from_instance(self)
	}
}

impl Default for Channel {
	fn default() -> Self {
		Object::new(&[]).unwrap()
	}
}