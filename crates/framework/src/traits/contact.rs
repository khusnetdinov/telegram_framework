use crate::structs::contact::Contact as Send;
use crate::structs::options::Options;
use crate::structs::updates::message::Message;

#[async_trait::async_trait]
pub trait Contact {
    async fn send_contact(
        &self,
        chat_id: i64,
        contact: Send,
        options: Option<Options>,
    ) -> Result<Message, Box<dyn std::error::Error>>;
}
