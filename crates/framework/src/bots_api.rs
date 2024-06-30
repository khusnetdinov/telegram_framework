use crate::config::Config;
use crate::enums::chat_action::ChatAction;
use crate::structs::bot_command::BotCommand;
use crate::structs::options::send_options::SendOptions;
use crate::structs::update::Update;
use crate::structs::update_kinds::message::Message;
use crate::structs::user::User;
use crate::structs::webhook::Webhook;
use crate::structs::webhook_info::WebhookInfo;
use crate::traits::bots_api::Pooler;
use crate::traits::commander::Commander;
use crate::traits::params::Params;
use crate::traits::sender::Sender;
use crate::traits::storage::Storage;
use crate::traits::webhooker::Webhooker;
use futures::Future;
use std::fmt::Debug;
use std::sync::Arc;
use telegram_bots_api::api::enums::chat_uid::ChatUId;
use telegram_bots_api::api::params::delete_my_commands::DeleteMyCommands;
use telegram_bots_api::api::params::get_my_commands::GetMyCommands;
use telegram_bots_api::api::params::get_update::GetUpdate;
use telegram_bots_api::api::params::send_chat_action::SendChatAction;
use telegram_bots_api::api::params::send_contact::SendContact;
use telegram_bots_api::api::params::send_dice::SendDice;
use telegram_bots_api::api::params::send_game::SendGame;
use telegram_bots_api::api::params::send_message::SendMessage;
use telegram_bots_api::api::params::send_venue::SendVenue;
use telegram_bots_api::api::params::set_my_commands::SetMyCommands;
use telegram_bots_api::api::requests::r#async::Requests;
use telegram_bots_api::clients::r#async::Async;
use tokio::time::sleep;
use tokio::time::Duration;

#[derive(Debug, Clone)]
pub struct BotsApi {
    config: Config,
    pub client: Async,
    pub user: User,
    pub webhook: Webhook,
}

impl BotsApi {
    pub async fn new(
        config: Config,
        client: Async,
        user: User,
        webhook: Webhook,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            config,
            client,
            user,
            webhook,
        })
    }

    pub async fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let config = Config::new();
        let client = Async::new(
            config.timeout,
            config.connect_timeout,
            config.url.as_str(),
            config.token.as_str(),
        );

        let user = User::from(client.get_me().await?);
        let webhook = Webhook::from(&config);

        let bots_api = Self::new(config, client, user, webhook).await?;

        Ok(bots_api)
    }
}

#[async_trait::async_trait]
impl Commander for BotsApi {
    async fn commands(
        &self,
        params: (DeleteMyCommands, GetMyCommands, SetMyCommands),
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (delete_params, _, set_params) = params;

        self.delete_commands(delete_params).await?;
        self.set_commands(set_params).await?;

        Ok(())
    }

    async fn delete_commands(
        &self,
        params: DeleteMyCommands,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(self.client.delete_my_commands(&params).await?)
    }

    async fn get_commands(
        &self,
        params: GetMyCommands,
    ) -> Result<Vec<BotCommand>, Box<dyn std::error::Error>> {
        Ok(self
            .client
            .get_my_commands(&params)
            .await?
            .iter()
            .map(|inner| BotCommand::from(inner.to_owned()))
            .collect::<Vec<BotCommand>>())
    }

    async fn set_commands(
        &self,
        params: SetMyCommands,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(self.client.set_my_commands(&params).await?)
    }
}

#[async_trait::async_trait]
impl<STO, STA> Pooler<STO, STA> for BotsApi {
    async fn pooling<Callback, Fut>(
        &self,
        storage: Arc<STO>,
        callback: Callback,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        Callback: Fn(BotsApi, Arc<STO>, Update) -> Fut + std::marker::Send,
        Fut: Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + 'static,
        STO: Storage<STA> + Debug + Send + Sync + 'async_trait,
        STA: Debug + Clone + 'async_trait,
    {
        let mut update_offset = self.config.updates_offset;

        self.delete_webhook().await?;

        loop {
            let updates = self
                .client
                .get_updates(&GetUpdate {
                    offset: update_offset,
                    limit: self.config.updates_limit,
                    timeout: self.config.updates_timeout,
                    allowed_updates: None,
                })
                .await
                .unwrap();

            for inner in updates.into_iter() {
                let offset = &inner.update_id + 1i64;

                callback(self.clone(), storage.clone(), Update::from(inner)).await?;

                update_offset = offset;
            }

            sleep(Duration::from_secs(self.config.pooling_timeout)).await;
        }
    }
}

#[async_trait::async_trait]
impl Sender for BotsApi {
    async fn send_chat_action(
        &self,
        chat_id: i64,
        action: ChatAction,
        options: Option<SendOptions>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = if let Some(options) = options {
            SendChatAction {
                action: action.into(),
                chat_id: ChatUId::from(chat_id),
                business_connection_id: options.business_connection_id,
                message_thread_id: options.message_thread_id,
            }
        } else {
            SendChatAction {
                action: action.into(),
                chat_id: ChatUId::from(chat_id),
                ..Default::default()
            }
        };

        Ok(self.client.send_chat_action(&params).await?)
    }

    async fn send_contact(
        &self,
        chat_id: i64,
        phone_number: String,
        first_name: String,
        options: Option<SendOptions>,
    ) -> Result<Message, Box<dyn std::error::Error>> {
        let params = if let Some(options) = options {
            SendContact {
                phone_number,
                first_name,
                chat_id: ChatUId::from(chat_id),
                business_connection_id: options.business_connection_id,
                disable_notification: options.disable_notification,
                protect_content: options.protect_content,
                message_effect_id: options.message_effect_id,
                message_thread_id: options.message_thread_id,
                reply_parameters: options.reply_parameters,
                reply_markup: options.reply_markup,
                last_name: options.last_name,
                vcard: options.vcard,
            }
        } else {
            SendContact {
                phone_number,
                first_name,
                chat_id: ChatUId::from(chat_id),
                ..Default::default()
            }
        };

        Ok(self.client.send_contact(&params).await?.into())
    }

    async fn send_dice(
        &self,
        chat_id: i64,
        options: Option<SendOptions>,
    ) -> Result<Message, Box<dyn std::error::Error>> {
        let params = if let Some(options) = options {
            SendDice {
                chat_id: ChatUId::from(chat_id),
                business_connection_id: options.business_connection_id,
                emoji: options.emoji.map(|emoji| emoji.into()),
                disable_notification: options.disable_notification,
                protect_content: options.protect_content,
                message_effect_id: options.message_effect_id,
                message_thread_id: options.message_thread_id,
                reply_parameters: options.reply_parameters,
                reply_markup: options.reply_markup,
            }
        } else {
            SendDice {
                chat_id: ChatUId::from(chat_id),
                ..Default::default()
            }
        };

        Ok(self.client.send_dice(&params).await?.into())
    }

    async fn send_game(
        &self,
        chat_id: i64,
        game_short_name: String,
        options: Option<SendOptions>,
    ) -> Result<Message, Box<dyn std::error::Error>> {
        let params = if let Some(options) = options {
            SendGame {
                game_short_name,
                chat_id: ChatUId::from(chat_id),
                business_connection_id: options.business_connection_id,
                disable_notification: options.disable_notification,
                protect_content: options.protect_content,
                message_effect_id: options.message_effect_id,
                message_thread_id: options.message_thread_id,
                reply_parameters: options.reply_parameters,
                reply_markup: options.reply_markup,
            }
        } else {
            SendGame {
                game_short_name,
                chat_id: ChatUId::from(chat_id),
                ..Default::default()
            }
        };

        Ok(self.client.send_game(&params).await?.into())
    }

    async fn send_message(
        &self,
        chat_id: i64,
        text: String,
        options: Option<SendOptions>,
    ) -> Result<Message, Box<dyn std::error::Error>> {
        let params = if let Some(options) = options {
            SendMessage {
                text,
                chat_id: ChatUId::from(chat_id),
                business_connection_id: options.business_connection_id,
                disable_notification: options.disable_notification,
                protect_content: options.protect_content,
                message_effect_id: options.message_effect_id,
                message_thread_id: options.message_thread_id,
                reply_parameters: options.reply_parameters,
                reply_markup: options.reply_markup,
                parse_mode: options.parse_mode,
                entities: options.entities,
                link_preview_options: options.link_preview_options,
            }
        } else {
            SendMessage {
                text,
                chat_id: ChatUId::from(chat_id),
                ..Default::default()
            }
        };

        Ok(self.client.send_message(&params).await?.into())
    }

    async fn send_venue(
        &self,
        chat_id: i64,
        latitude: f64,
        longitude: f64,
        title: String,
        address: String,
        options: Option<SendOptions>,
    ) -> Result<Message, Box<dyn std::error::Error>> {
        let params = if let Some(options) = options {
            SendVenue {
                latitude,
                longitude,
                title,
                address,
                chat_id: ChatUId::from(chat_id),
                business_connection_id: options.business_connection_id,
                disable_notification: options.disable_notification,
                protect_content: options.protect_content,
                message_effect_id: options.message_effect_id,
                message_thread_id: options.message_thread_id,
                reply_parameters: options.reply_parameters,
                reply_markup: options.reply_markup,
                foursquare_id: options.foursquare_id,
                foursquare_type: options.foursquare_type,
                google_place_id: options.google_place_id,
                google_place_type: options.google_place_type,
            }
        } else {
            SendVenue {
                latitude,
                longitude,
                title,
                address,
                chat_id: ChatUId::from(chat_id),
                ..Default::default()
            }
        };

        Ok(self.client.send_venue(&params).await?.into())
    }
}

#[async_trait::async_trait]
impl Webhooker for BotsApi {
    async fn delete_webhook(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let params = self.webhook.delete_params();

        Ok(self.client.delete_webhook(&params).await?)
    }

    async fn get_webhook(&self) -> Result<WebhookInfo, Box<dyn std::error::Error>> {
        Ok(WebhookInfo::from(self.client.get_webhook_info().await?))
    }

    async fn set_webhook(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let params = self.webhook.set_params();

        Ok(self.client.set_webhook(&params).await?)
    }
}
