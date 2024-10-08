use crate::structs::media::paid_media::input_paid_media::input_paid_media_photo::InputPaidMediaPhoto;
use crate::structs::media::paid_media::input_paid_media::input_paid_media_video::InputPaidMediaVideo;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::input_paid_media::InputPaidMedia as Remote;
use telegram_macros::{FromRemoteEnum, IntoRemoteEnum};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteEnum, IntoRemoteEnum)]
pub enum InputPaidMedia {
    Photo(InputPaidMediaPhoto),
    Video(InputPaidMediaVideo),
}
