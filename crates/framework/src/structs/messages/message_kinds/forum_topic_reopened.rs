use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::forum_topic_reopened::ForumTopicReopened as Remote;
use telegram_bots_api::api::structs::message::Message as IncomingMessage;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct ForumTopicReopened {}

impl From<IncomingMessage> for ForumTopicReopened {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage {
            forum_topic_reopened,
            ..
        } = remote;

        Self::from(forum_topic_reopened.unwrap())
    }
}