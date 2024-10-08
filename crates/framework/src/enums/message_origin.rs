use crate::structs::messages::message_origins::message_origin_channel::MessageOriginChannel;
use crate::structs::messages::message_origins::message_origin_chat::MessageOriginChat;
use crate::structs::messages::message_origins::message_origin_hidden_user::MessageOriginHiddenUser;
use crate::structs::messages::message_origins::message_origin_user::MessageOriginUser;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::message_origin::MessageOrigin as Remote;
use telegram_macros::{FromRemoteEnum, IntoRemoteEnum};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteEnum, IntoRemoteEnum)]
#[serde(untagged)]
pub enum MessageOrigin {
    User(MessageOriginUser),
    HiddenUser(MessageOriginHiddenUser),
    Chat(MessageOriginChat),
    Channel(MessageOriginChannel),
}
