use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::forum_topic_edited::ForumTopicEdited as Remote;
use telegram_bots_api::api::structs::message::Message;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct ForumTopicEdited {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

impl From<Message> for ForumTopicEdited {
    fn from(remote: Message) -> Self {
        let Message {
            forum_topic_edited, ..
        } = remote;

        Self::from(forum_topic_edited.unwrap())
    }
}