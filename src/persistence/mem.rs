use lazy_static::lazy_static;
use serenity::model::id::MessageId;
use std::sync::Mutex;

lazy_static! {
    pub static ref STICKY_MESSAGE: Mutex<Option<String>> = Mutex::new(None);
    pub static ref STICKY_MESSAGE_ID: Mutex<Option<MessageId>> = Mutex::new(None);
}

pub fn set_sticky(message: String) {
    *STICKY_MESSAGE.lock().unwrap() = Some(message);
}

pub fn clear_sticky() {
    *STICKY_MESSAGE.lock().unwrap() = None;
    *STICKY_MESSAGE_ID.lock().unwrap() = None;
}

pub fn is_sticky_set() -> bool {
    STICKY_MESSAGE.lock().unwrap().is_some()
}

pub fn update_message_id(message_id: MessageId) {
    *STICKY_MESSAGE_ID.lock().unwrap() = Some(message_id);
}

pub fn get_sticky() -> String {
    STICKY_MESSAGE.lock().unwrap().as_ref().unwrap().clone()
}

pub fn get_sticky_id() -> MessageId {
    STICKY_MESSAGE_ID.lock().unwrap().as_ref().unwrap().clone()
}
