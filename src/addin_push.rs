use crate::client;
use addin1c::{name, AddinResult, MethodInfo, Methods, PropInfo, SimpleAddin, Variant};
use std::error::Error;

pub struct AddinPush {
    client: client::Client,
    last_error: Option<Box<dyn Error>>,
}

impl AddinPush {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            client: client::Client::new(zmq::PUSH)?,
            last_error: None,
        })
    }

    fn last_error(&mut self, value: &mut Variant) -> AddinResult {
        match &self.last_error {
            Some(err) => value
                .set_str1c(err.to_string().as_str())
                .map_err(|e| e.into()),
            None => value.set_str1c("").map_err(|e| e.into()),
        }
    }

    fn bind(&mut self, endpoint: &mut Variant, _ret_value: &mut Variant) -> AddinResult {
        self.client.bind(endpoint)
    }

    fn unbind(&mut self, endpoint: &mut Variant, _ret_value: &mut Variant) -> AddinResult {
        self.client.unbind(endpoint)
    }

    fn connect(&mut self, endpoint: &mut Variant, _ret_value: &mut Variant) -> AddinResult {
        self.client.connect(endpoint)
    }

    fn disconnect(&mut self, endpoint: &mut Variant, _ret_value: &mut Variant) -> AddinResult {
        self.client.disconnect(endpoint)
    }

    fn send(&mut self, data: &mut Variant, _ret_value: &mut Variant) -> AddinResult {
        self.client.send(data)
    }

    fn send_part(&mut self, data: &mut Variant, _ret_value: &mut Variant) -> AddinResult {
        self.client.send_part(data)
    }
}

impl SimpleAddin for AddinPush {
    fn name() -> &'static [u16] {
        name!("ZeroMQ.Push")
    }

    fn save_error(&mut self, err: Option<Box<dyn Error>>) {
        self.last_error = err;
    }

    fn methods() -> &'static [addin1c::MethodInfo<Self>] {
        &[
            MethodInfo {
                name: name!("Bind"),
                method: Methods::Method1(Self::bind),
            },
            MethodInfo {
                name: name!("Unbind"),
                method: Methods::Method1(Self::unbind),
            },
            MethodInfo {
                name: name!("Connect"),
                method: Methods::Method1(Self::connect),
            },
            MethodInfo {
                name: name!("Disconnect"),
                method: Methods::Method1(Self::disconnect),
            },
            MethodInfo {
                name: name!("Send"),
                method: Methods::Method1(Self::send),
            },
            MethodInfo {
                name: name!("SendPart"),
                method: Methods::Method1(Self::send_part),
            },
        ]
    }

    fn properties() -> &'static [PropInfo<Self>] {
        &[PropInfo {
            name: name!("LastError"),
            getter: Some(Self::last_error),
            setter: None,
        }]
    }
}
