use crate::structs::media::photo_size::PhotoSize;
use crate::structs::messages::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as IncomingMessage;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhotoSizesReceived {
    pub photo: Vec<PhotoSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_media_spoiler: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
}

impl From<IncomingMessage> for PhotoSizesReceived {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage {
            photo,
            media_group_id,
            has_media_spoiler,
            caption,
            caption_entities,
            show_caption_above_media,
            ..
        } = remote;

        Self {
            photo: photo
                .unwrap()
                .iter()
                .map(|inner| inner.to_owned().into())
                .collect(),
            media_group_id,
            has_media_spoiler,
            caption,
            caption_entities: caption_entities
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
            show_caption_above_media,
        }
    }
}
