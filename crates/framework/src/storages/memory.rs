use crate::traits::storage::Storage;
use futures::future::BoxFuture;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct MemoryStorage<ST> {
    pub states: Mutex<HashMap<i64, ST>>,
}

impl<ST> MemoryStorage<ST> {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            states: Mutex::new(HashMap::new()),
        })
    }
}

impl<ST> Storage<ST> for MemoryStorage<ST>
where
    ST: Debug + Clone + Send + 'static,
{
    type Error = Box<dyn std::error::Error>;

    fn get(self: Arc<Self>, chat_id: i64) -> BoxFuture<'static, Result<Option<ST>, Self::Error>> {
        Box::pin(async move {
            Ok(self
                .states
                .lock()
                .await
                .get(&chat_id)
                .map(ToOwned::to_owned))
        })
    }

    fn set(
        self: Arc<Self>,
        chat_id: i64,
        state: ST,
    ) -> BoxFuture<'static, Result<(), Self::Error>> {
        Box::pin(async move {
            self.states.lock().await.insert(chat_id, state);
            Ok(())
        })
    }
}
