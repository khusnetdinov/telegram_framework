use crate::bots_api::BotsApi;
use crate::enums::chat_uid::ChatUId;
use crate::enums::input_media::InputMedia;
use crate::enums::message_result::MessageResult;
use crate::enums::reaction_type::ReactionType;
use crate::structs::message::Message as Response;
use crate::structs::messages::message_id::MessageId;
use crate::structs::messages::options::Options as MessageOptions;
use crate::traits::features::message::Message;
use telegram_bots_api::api::params::copy_message::CopyMessage;
use telegram_bots_api::api::params::copy_messages::CopyMessages;
use telegram_bots_api::api::params::delete_message::DeleteMessage;
use telegram_bots_api::api::params::delete_messages::DeleteMessages;
use telegram_bots_api::api::params::edit_message_caption::EditMessageCaption;
use telegram_bots_api::api::params::edit_message_live_location::EditMessageLiveLocation;
use telegram_bots_api::api::params::edit_message_media::EditMessageMedia;
use telegram_bots_api::api::params::edit_message_reply_markup::EditMessageReplyMarkup;
use telegram_bots_api::api::params::edit_message_text::EditMessageText;
use telegram_bots_api::api::params::forward_message::ForwardMessage;
use telegram_bots_api::api::params::forward_messages::ForwardMessages;
use telegram_bots_api::api::params::send_message::SendMessage;
use telegram_bots_api::api::params::set_message_reaction::SetMessageReaction;
use telegram_bots_api::api::params::stop_message_live_location::StopMessageLiveLocation;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl Message for BotsApi {
    async fn copy_message(
        &self,
        chat_id: ChatUId,
        from_chat_id: ChatUId,
        message_id: MessageId,
        message_options: MessageOptions,
    ) -> Result<MessageId, Box<dyn std::error::Error>> {
        let MessageOptions {
            message_thread_id,
            disable_notification,
            protect_content,
            parse_mode,
            caption,
            caption_entities,
            show_caption_above_media,
            reply_markup,
            reply_parameters,
            ..
        } = message_options;

        let params = CopyMessage {
            chat_id: chat_id.into(),
            from_chat_id: from_chat_id.into(),
            message_id: message_id.into(),
            message_thread_id,
            disable_notification,
            protect_content,
            parse_mode,
            caption,
            caption_entities: caption_entities
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
            show_caption_above_media,
            reply_parameters: reply_parameters.map(|inner| inner.into()),
            reply_markup: reply_markup.map(|inner| inner.into()),
        };

        Ok(self.client.copy_message(&params).await?.into())
    }

    async fn copy_messages(
        &self,
        chat_id: ChatUId,
        from_chat_id: ChatUId,
        message_ids: Vec<MessageId>,
        message_options: MessageOptions,
    ) -> Result<Vec<MessageId>, Box<dyn std::error::Error>> {
        let MessageOptions {
            message_thread_id,
            disable_notification,
            protect_content,
            remove_caption,
            ..
        } = message_options;

        let params = CopyMessages {
            chat_id: chat_id.into(),
            from_chat_id: from_chat_id.into(),
            message_ids: message_ids
                .iter()
                .map(|inner| inner.to_owned().into())
                .collect(),
            message_thread_id,
            disable_notification,
            protect_content,
            remove_caption,
        };

        Ok(self
            .client
            .copy_messages(&params)
            .await?
            .iter()
            .map(|inner| inner.to_owned().into())
            .collect())
    }

    async fn delete_message(
        &self,
        chat_id: ChatUId,
        message_id: MessageId,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = DeleteMessage {
            chat_id: chat_id.into(),
            message_id: message_id.into(),
        };

        Ok(self.client.delete_message(&params).await?)
    }

    async fn delete_messages(
        &self,
        chat_id: ChatUId,
        message_ids: Vec<i64>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = DeleteMessages {
            chat_id: chat_id.into(),
            message_ids,
        };

        Ok(self.client.delete_messages(&params).await?)
    }

    async fn edit_message_caption(
        &self,
        chat_id: ChatUId,
        message_id: MessageId,
        message_options: MessageOptions,
    ) -> Result<MessageResult, Box<dyn std::error::Error>> {
        let MessageOptions {
            inline_keyboard_markup,
            business_connection_id,
            parse_mode,
            caption,
            caption_entities,
            show_caption_above_media,
            ..
        } = message_options;

        let params = EditMessageCaption {
            chat_id: Some(chat_id.into()),
            message_id: Some(message_id.into()),
            inline_message_id: None,
            reply_markup: inline_keyboard_markup.map(|inner| inner.into()),
            business_connection_id,
            parse_mode,
            caption,
            caption_entities: caption_entities
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
            show_caption_above_media,
        };

        Ok(self.client.edit_message_caption(&params).await?.into())
    }

    async fn edit_message_caption_inline(
        &self,
        inline_message_id: String,
        message_options: MessageOptions,
    ) -> Result<MessageResult, Box<dyn std::error::Error>> {
        let MessageOptions {
            inline_keyboard_markup,
            business_connection_id,
            parse_mode,
            caption,
            caption_entities,
            show_caption_above_media,
            ..
        } = message_options;

        let params = EditMessageCaption {
            chat_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id),
            reply_markup: inline_keyboard_markup.map(|inner| inner.into()),
            business_connection_id,
            parse_mode,
            caption,
            caption_entities: caption_entities
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
            show_caption_above_media,
        };

        Ok(self.client.edit_message_caption(&params).await?.into())
    }

    async fn edit_message_live_location(
        &self,
        chat_id: ChatUId,
        message_id: MessageId,
        latitude: f64,
        longitude: f64,
        live_period: Option<i64>,
        message_options: MessageOptions,
    ) -> Result<MessageResult, Box<dyn std::error::Error>> {
        let MessageOptions {
            inline_keyboard_markup,
            business_connection_id,
            horizontal_accuracy,
            heading,
            proximity_alert_radius,
            ..
        } = message_options;

        let params = EditMessageLiveLocation {
            chat_id: Some(chat_id.into()),
            message_id: Some(message_id.into()),
            inline_message_id: None,
            latitude,
            longitude,
            live_period,
            reply_markup: inline_keyboard_markup.map(|inner| inner.into()),
            business_connection_id,
            horizontal_accuracy,
            heading,
            proximity_alert_radius,
        };

        Ok(self
            .client
            .edit_message_live_location(&params)
            .await?
            .into())
    }

    async fn edit_message_live_location_inline(
        &self,
        inline_message_id: String,
        latitude: f64,
        longitude: f64,
        live_period: Option<i64>,
        message_options: MessageOptions,
    ) -> Result<MessageResult, Box<dyn std::error::Error>> {
        let MessageOptions {
            inline_keyboard_markup,
            business_connection_id,
            horizontal_accuracy,
            heading,
            proximity_alert_radius,
            ..
        } = message_options;

        let params = EditMessageLiveLocation {
            chat_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id),
            latitude,
            longitude,
            live_period,
            reply_markup: inline_keyboard_markup.map(|inner| inner.into()),
            business_connection_id,
            horizontal_accuracy,
            heading,
            proximity_alert_radius,
        };

        Ok(self
            .client
            .edit_message_live_location(&params)
            .await?
            .into())
    }

    async fn edit_message_media(
        &self,
        chat_id: ChatUId,
        message_id: MessageId,
        media: InputMedia,
        message_options: MessageOptions,
    ) -> Result<MessageResult, Box<dyn std::error::Error>> {
        let MessageOptions {
            inline_keyboard_markup,
            business_connection_id,
            ..
        } = message_options;

        let params = EditMessageMedia {
            chat_id: Some(chat_id.into()),
            message_id: Some(message_id.into()),
            inline_message_id: None,
            media: media.into(),
            reply_markup: inline_keyboard_markup.map(|inner| inner.into()),
            business_connection_id,
        };

        Ok(self.client.edit_message_media(&params).await?.into())
    }

    async fn edit_message_media_inline(
        &self,
        inline_message_id: String,
        media: InputMedia,
        message_options: MessageOptions,
    ) -> Result<MessageResult, Box<dyn std::error::Error>> {
        let MessageOptions {
            inline_keyboard_markup,
            business_connection_id,
            ..
        } = message_options;

        let params = EditMessageMedia {
            media: media.into(),
            chat_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id),
            reply_markup: inline_keyboard_markup.map(|inner| inner.into()),
            business_connection_id,
        };

        Ok(self.client.edit_message_media(&params).await?.into())
    }

    async fn edit_message_reply_markup(
        &self,
        chat_id: ChatUId,
        message_id: MessageId,
        message_options: MessageOptions,
    ) -> Result<MessageResult, Box<dyn std::error::Error>> {
        let MessageOptions {
            inline_keyboard_markup,
            business_connection_id,
            ..
        } = message_options;

        let params = EditMessageReplyMarkup {
            chat_id: Some(chat_id.into()),
            message_id: Some(message_id.into()),
            inline_message_id: None,
            reply_markup: inline_keyboard_markup.map(|inner| inner.into()),
            business_connection_id,
        };

        Ok(self.client.edit_message_reply_markup(&params).await?.into())
    }

    async fn edit_message_reply_markup_inline(
        &self,
        inline_message_id: String,
        message_options: MessageOptions,
    ) -> Result<MessageResult, Box<dyn std::error::Error>> {
        let MessageOptions {
            inline_keyboard_markup,
            business_connection_id,
            ..
        } = message_options;

        let params = EditMessageReplyMarkup {
            chat_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id),
            reply_markup: inline_keyboard_markup.map(|inner| inner.into()),
            business_connection_id,
        };

        Ok(self.client.edit_message_reply_markup(&params).await?.into())
    }

    async fn edit_message_text(
        &self,
        text: String,
        chat_id: ChatUId,
        message_id: MessageId,
        message_options: MessageOptions,
    ) -> Result<MessageResult, Box<dyn std::error::Error>> {
        let MessageOptions {
            parse_mode,
            entities,
            link_preview_options,
            inline_keyboard_markup,
            business_connection_id,
            ..
        } = message_options;

        let params = EditMessageText {
            text,
            chat_id: Some(chat_id.into()),
            message_id: Some(message_id.into()),
            inline_message_id: None,
            parse_mode,
            entities: entities
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
            link_preview_options: link_preview_options.map(|inner| inner.to_owned().into()),
            reply_markup: inline_keyboard_markup.map(|inner| inner.into()),
            business_connection_id,
        };

        Ok(self.client.edit_message_text(&params).await?.into())
    }

    async fn edit_message_text_inline(
        &self,
        text: String,
        inline_message_id: String,
        message_options: MessageOptions,
    ) -> Result<MessageResult, Box<dyn std::error::Error>> {
        let MessageOptions {
            parse_mode,
            entities,
            link_preview_options,
            inline_keyboard_markup,
            business_connection_id,
            ..
        } = message_options;

        let params = EditMessageText {
            text,
            chat_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id),
            parse_mode,
            entities: entities
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
            link_preview_options: link_preview_options.map(|inner| inner.to_owned().into()),
            reply_markup: inline_keyboard_markup.map(|inner| inner.into()),
            business_connection_id,
        };

        Ok(self.client.edit_message_text(&params).await?.into())
    }

    async fn forward_message(
        &self,
        chat_id: ChatUId,
        from_chat_id: ChatUId,
        message_id: MessageId,
        message_options: MessageOptions,
    ) -> Result<MessageId, Box<dyn std::error::Error>> {
        let MessageOptions {
            message_thread_id,
            disable_notification,
            protect_content,
            ..
        } = message_options;

        let params = ForwardMessage {
            chat_id: chat_id.into(),
            from_chat_id: from_chat_id.into(),
            message_id: message_id.into(),
            message_thread_id,
            disable_notification,
            protect_content,
        };

        Ok(self.client.forward_message(&params).await?.into())
    }

    async fn forward_messages(
        &self,
        chat_id: ChatUId,
        from_chat_id: ChatUId,
        message_ids: Vec<MessageId>,
        message_options: MessageOptions,
    ) -> Result<Vec<MessageId>, Box<dyn std::error::Error>> {
        let MessageOptions {
            message_thread_id,
            disable_notification,
            protect_content,
            ..
        } = message_options;

        let params = ForwardMessages {
            chat_id: chat_id.into(),
            from_chat_id: from_chat_id.into(),
            message_ids: message_ids
                .iter()
                .map(|inner| inner.to_owned().into())
                .collect(),
            message_thread_id,
            disable_notification,
            protect_content,
        };

        Ok(self
            .client
            .forward_messages(&params)
            .await?
            .iter()
            .map(|inner| inner.to_owned().into())
            .collect())
    }

    async fn set_message_reaction(
        &self,
        chat_id: ChatUId,
        message_id: MessageId,
        reaction: Option<Vec<ReactionType>>,
        is_big: Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = SetMessageReaction {
            message_id: message_id.into(),
            chat_id: chat_id.into(),
            reaction: reaction
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
            is_big,
        };

        Ok(self.client.set_message_reaction(&params).await?)
    }

    async fn stop_message_live_location(
        &self,
        message_options: MessageOptions,
    ) -> Result<MessageResult, Box<dyn std::error::Error>> {
        let MessageOptions {
            chat_id,
            message_id,
            inline_message_id,
            inline_keyboard_markup,
            ..
        } = message_options;

        let params = StopMessageLiveLocation {
            chat_id: chat_id.map(|inner| inner.into()),
            message_id: message_id.map(|inner| inner.into()),
            inline_message_id,
            reply_markup: inline_keyboard_markup.map(|inner| inner.into()),
        };

        Ok(self
            .client
            .stop_message_live_location(&params)
            .await?
            .into())
    }

    async fn send_message(
        &self,
        chat_id: ChatUId,
        text: String,
        message_options: MessageOptions,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let MessageOptions {
            message_thread_id,
            parse_mode,
            entities,
            link_preview_options,
            disable_notification,
            protect_content,
            reply_parameters,
            reply_markup,
            business_connection_id,
            message_effect_id,
            ..
        } = message_options;

        let params = SendMessage {
            chat_id: chat_id.into(),
            text,
            message_thread_id,
            parse_mode,
            entities: entities
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
            link_preview_options: link_preview_options.map(|inner| inner.to_owned().into()),
            disable_notification,
            protect_content,
            reply_parameters: reply_parameters.map(|inner| inner.into()),
            reply_markup: reply_markup.map(|inner| inner.into()),
            business_connection_id,
            message_effect_id,
        };

        Ok(self.client.send_message(&params).await?.into())
    }
}
