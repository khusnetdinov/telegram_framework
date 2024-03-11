use std::ops::Deref;
use telegram_bots_api::config::Config as Inner;

#[derive(Debug)]
pub struct Config {
    pub inner: Inner,
}

impl Config {
    pub fn new() -> Self {
        Self {
            inner: Inner::new(),
        }
    }
}

impl Deref for Config {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<Inner> for Config {
    fn from(inner: Inner) -> Self {
        Self { inner }
    }
}
