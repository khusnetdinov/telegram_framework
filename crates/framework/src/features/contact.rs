use crate::bots_api::BotsApi;
use crate::structs::contact::Contact as Send;
use crate::structs::options::Options;
use crate::structs::updates::message::Message;
use crate::traits::contact::Contact;
use telegram_bots_api::api::enums::chat_uid::ChatUId;
use telegram_bots_api::api::params::send_contact::SendContact;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl Contact for BotsApi {
    async fn send_contact(
        &self,
        chat_id: i64,
        contact: Send,
        options: Option<Options>,
    ) -> Result<Message, Box<dyn std::error::Error>> {
        let params = if let Some(options) = options {
            SendContact {
                chat_id: ChatUId::from(chat_id),
                phone_number: contact.phone_number,
                first_name: contact.first_name,
                last_name: contact.last_name,
                vcard: contact.vcard,
                business_connection_id: options.business_connection_id,
                disable_notification: options.disable_notification,
                protect_content: options.protect_content,
                message_effect_id: options.message_effect_id,
                message_thread_id: options.message_thread_id,
                reply_parameters: options.reply_parameters,
                reply_markup: options.reply_markup,
            }
        } else {
            SendContact {
                chat_id: ChatUId::from(chat_id),
                phone_number: contact.phone_number,
                first_name: contact.first_name,
                last_name: contact.last_name,
                vcard: contact.vcard,
                ..Default::default()
            }
        };

        Ok(self.client.send_contact(&params).await?.into())
    }
}
