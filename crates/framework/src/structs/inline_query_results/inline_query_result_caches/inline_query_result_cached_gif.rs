use crate::enums::input_message_content::InputMessageContent;
use crate::structs::messages::message_entity::MessageEntity;
use crate::structs::reply_markups::inline_keyboards::inline_keyboard_markup::InlineKeyboardMarkup;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::inline_query_result_cached_gif::InlineQueryResultCachedGif as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct InlineQueryResultCachedGif {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub id: String,
    pub gif_file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
}