use crate::enums::chat_action::ChatAction;
use crate::structs::options::send_options::SendOptions;
use crate::structs::update_kinds::message::Message;

#[async_trait::async_trait]
pub trait Sender {
    async fn send_chat_action(
        &self,
        chat_id: i64,
        action: ChatAction,
        options: Option<SendOptions>,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn send_contact(
        &self,
        chat_id: i64,
        phone_number: String,
        first_name: String,
        options: Option<SendOptions>,
    ) -> Result<Message, Box<dyn std::error::Error>>;

    async fn send_dice(
        &self,
        chat_id: i64,
        options: Option<SendOptions>,
    ) -> Result<Message, Box<dyn std::error::Error>>;

    async fn send_game(
        &self,
        chat_id: i64,
        game_short_name: String,
        options: Option<SendOptions>,
    ) -> Result<Message, Box<dyn std::error::Error>>;

    // fn send_invoice(&self) -> Message;
    // fn send_location(&self) -> Mesasge;
    // fn send_media_group(&self) -> Vec<Message>;

    async fn send_message(
        &self,
        chat_id: i64,
        text: String,
        options: Option<SendOptions>,
    ) -> Result<Message, Box<dyn std::error::Error>>;

    // fn send_poll(&self) -> Message;

    async fn send_venue(
        &self,
        chat_id: i64,
        latitude: f64,
        longitude: f64,
        title: String,
        address: String,
        options: Option<SendOptions>,
    ) -> Result<Message, Box<dyn std::error::Error>>;
}
