//! High-level API
use bot::methods;
use bot::methods::UpdateList;
use bot::types;
use reqwest::{Client, Url};
use serde;
use serde_json;
use std::iter::Iterator;
use std::sync::Arc;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

#[derive(PartialEq)]
enum Signal {
    Stop,
}

pub fn from_result<T>(raw: &str) -> Result<T, serde_json::Error>
    where T: for<'de> serde::Deserialize<'de> {
    let result: serde_json::Value = serde_json::from_str(raw)?;
    let value = result.get("result").unwrap().clone();
    Ok(serde_json::from_value::<T>(value)?)
}


pub struct PollingUpdater {
    token: String,
    base_url: Url,
    param: methods::GetUpdates,
    client: Arc<Client>,
}


impl PollingUpdater {
    pub fn with_client(client: Client, token: String) -> PollingUpdater {
        let url = "https://api.telegram.org/bot".to_string() + &token + "/";
        let base_url = Url::parse(&*url).expect("base url parse failure.");
        PollingUpdater {
            param: methods::GetUpdates::new(),
            token,
            base_url,
            client: Arc::new(client),
        }
    }
    pub fn new(token: String) -> PollingUpdater {
        PollingUpdater::with_client(Client::new(), token)
    }

    pub fn timeout(self, x: i32) -> PollingUpdater {
        PollingUpdater {
            param: methods::GetUpdates {
                timeout: Some(x),
                ..self.param
            },
            ..self
        }
    }

    pub fn iter(&self) -> PollingStream {
        let (tx_control, rx_control) = channel();
        let (tx_updates, rx_updates) = channel();
        let url = self.base_url.join("getUpdates")
            .expect("get updates url parse error");
        let param = self.param.clone();
        let client = self.client.clone();
        thread::spawn(move || {
            let mut param = param;
            let control_signal = rx_control;
            while control_signal.try_recv() != Ok(Signal::Stop) {
                let body = client
                    .post(url.clone())
                    .json(&param)
                    .send()
                    .unwrap()
                    .text()
                    .unwrap();
                let UpdateList(updates) = from_result::<UpdateList>(&*body).unwrap();
                for update in updates {
                    param.offset = Some(update.update_id.clone() + 1);
                    tx_updates.send(update).unwrap();
                }
            }
        });
        PollingStream {
            updates: rx_updates,
            control: tx_control,
        }
    }
}


pub struct PollingStream {
    updates: Receiver<types::Update>,
    control: Sender<Signal>,
}


impl Iterator for PollingStream {
    type Item = types::Update;

    fn next(&mut self) -> Option<types::Update> {
        self.updates.recv().ok()
    }
}

impl Drop for PollingStream {
    fn drop(&mut self) {
        let _ = self.control.send(Signal::Stop);
    }
}
