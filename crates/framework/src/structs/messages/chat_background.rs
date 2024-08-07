use crate::enums::background_type::BackgroundType;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_background::ChatBackground as Remote;
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatBackground {
    pub kind: BackgroundType,
}
impl From<Remote> for ChatBackground {
    fn from(remote: Remote) -> Self {
        Self {
            // TODO: #[remote(into)]
            kind: remote.kind.into(),
        }
    }
}
impl From<Message> for ChatBackground {
    fn from(remote: Message) -> Self {
        let Message {
            chat_background_set,
            ..
        } = remote;

        Self::from(chat_background_set.unwrap())
    }
}
