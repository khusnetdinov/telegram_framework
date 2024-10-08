use crate::enums::background_type::BackgroundType;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_background::ChatBackground as Remote;
use telegram_bots_api::api::structs::message::Message as IncomingMessage;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct ChatBackground {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: BackgroundType,
}

impl From<IncomingMessage> for ChatBackground {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage {
            chat_background_set,
            ..
        } = remote;

        Self::from(chat_background_set.unwrap())
    }
}
