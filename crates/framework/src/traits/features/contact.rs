use crate::enums::chat_uid::ChatUId;
use crate::structs::contact::Contact as Send;
use crate::structs::contacts::options::Options as ContactOptions;
use crate::structs::message::Message;

#[async_trait::async_trait]
pub trait Contact {
    async fn send_contact(
        &self,
        chat_id: ChatUId,
        contact: Send,
        options: ContactOptions,
    ) -> Result<Message, Box<dyn std::error::Error>>;
}
