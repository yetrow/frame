use std::sync::mpsc::Receiver;

use super::data::ConfigData;

#[derive(Debug)]
pub struct Inner {
    rx: Receiver<ConfigData>,
    config: ConfigData,
}

impl Inner {
    pub fn new(config: ConfigData, rx: Receiver<ConfigData>) -> Self {
        Self { config, rx }
    }

    pub fn config(&mut self) -> &mut ConfigData {
        if let Some(config) = self.rx.try_iter().last() {
            self.config = config;
        }

        &mut self.config
    }
}