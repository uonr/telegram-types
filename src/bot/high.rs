//! High-level API
use bot::methods;
use bot::methods::UpdateList;
use bot::types;
use reqwest;
use reqwest::{Client, Response, Url};
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
    response: Option<Receiver<reqwest::Result<Response>>>,
    control: Option<Sender<Signal>>,
    updates: Vec<types::Update>,
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
            response: None,
            control: None,
            updates: Vec::new(),
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
}

impl Iterator for PollingUpdater {
    type Item = types::Update;

    fn next(&mut self) -> Option<types::Update> {
        if self.updates.is_empty() {
            if let None = self.response {
                let (tx_control, rx_control) = channel();
                let (tx_updates, rx_updates) = channel();
                self.response = Some(rx_updates);
                self.control = Some(tx_control);
                let url = self.base_url.join("getUpdates")
                    .expect("get updates url parse error");
                let param = self.param.clone();
                let client = self.client.clone();
                thread::spawn(move || {
                    let mut param = param;
                    let control_signal = rx_control;
                    while control_signal.try_recv() != Ok(Signal::Stop) {
                        let response = client
                            .post(url.clone())
                            .json(&param)
                            .send();
                        tx_updates.send(response);
                    }
                });
            }
            let rx = self.response.as_mut().unwrap();
            loop {
                let body = rx.recv().unwrap().unwrap().text().unwrap();
                let UpdateList(mut updates) = from_result::<UpdateList>(&*body).unwrap();
                if !updates.is_empty() {
                    updates.reverse();
                    self.updates = updates;
                    break;
                }
            }
        }
        if let Some(update) = self.updates.pop() {
            self.param.offset = Some(update.update_id.clone() + 1);
            return Some(update);
        } else { unreachable!(); }
    }
}


impl Drop for PollingUpdater {
    fn drop(&mut self) {
        if let Some(ref tx) = self.control {
            tx.send(Signal::Stop);
        }
    }
}