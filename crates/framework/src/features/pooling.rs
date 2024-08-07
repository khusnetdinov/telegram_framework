use crate::bots_api::BotsApi;
use crate::structs::update::Update;
use crate::traits::bots_apis::pooling::Pooling;
use crate::traits::storage::Storage;
use futures::Future;
use std::fmt::Debug;
use std::sync::Arc;
use telegram_bots_api::api::params::get_update::GetUpdate;
use telegram_bots_api::api::requests::r#async::Requests;
use tokio::time::sleep;
use tokio::time::Duration;

use crate::feature::webhook::*;

#[async_trait::async_trait]
impl<STO, STA> Pooling<STO, STA> for BotsApi {
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
