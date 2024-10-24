use std::{error::Error, sync::Mutex};

use addin1c::{AddinResult, Variant};
use smallvec::SmallVec;
use zmq::{Context, Message, PollEvents, Socket, SocketType, SNDMORE};

#[allow(dead_code)]
struct State {
    count: u32,
    context: Option<Context>,
}

#[allow(dead_code)]
static STATE: Mutex<State> = Mutex::new(State {
    count: 0,
    context: None,
});

pub struct Client {
    socket_type: SocketType,
    socket: Option<Socket>,
    msg: Message,
    parts: SmallVec<[Message; 4]>,
}

fn get_socket(socket: Option<&Socket>) -> Result<&Socket, Box<dyn Error>> {
    match socket {
        Some(socket) => Ok(socket),
        None => Err("Not connected to a socket".into()),
    }
}

impl Client {
    pub fn new(socket_type: SocketType) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            socket_type,
            socket: None,
            msg: Message::new(),
            parts: SmallVec::new(),
        })
    }

    fn get_or_create_socket(&mut self) -> Result<&Socket, Box<dyn Error>> {
        if self.socket.is_none() {
            let mut state = STATE.lock()?;
            state.count += 1;
            let context = state.context.get_or_insert_with(|| Context::new());
            let socket = context.socket(self.socket_type)?;
            self.socket = Some(socket);
        }

        // SAFETY: always `Some`
        unsafe { Ok(self.socket.as_ref().unwrap_unchecked()) }
    }

    pub fn bind(&mut self, endpoint: &mut Variant) -> AddinResult {
        let endpoint = endpoint.get_string()?;
        let socket = self.get_or_create_socket()?;
        socket.bind(&endpoint)?;
        Ok(())
    }

    pub fn unbind(&mut self, endpoint: &mut Variant) -> AddinResult {
        let endpoint = endpoint.get_string()?;
        let socket = get_socket(self.socket.as_ref())?;
        socket.unbind(&endpoint)?;
        Ok(())
    }

    pub fn connect(&mut self, endpoint: &mut Variant) -> AddinResult {
        let endpoint = endpoint.get_string()?;
        let socket = self.get_or_create_socket()?;
        socket.connect(&endpoint)?;
        Ok(())
    }

    pub fn disconnect(&mut self, endpoint: &mut Variant) -> AddinResult {
        let endpoint = endpoint.get_string()?;
        let socket = get_socket(self.socket.as_ref())?;
        socket.disconnect(&endpoint)?;
        Ok(())
    }

    pub fn send(&mut self, data: &mut Variant) -> AddinResult {
        let data = data.get_blob()?;
        let socket = get_socket(self.socket.as_ref())?;
        socket.send(data, 0)?;
        Ok(())
    }

    pub fn send_part(&mut self, data: &mut Variant) -> AddinResult {
        let data = data.get_blob()?;
        let socket = get_socket(self.socket.as_ref())?;
        socket.send(data, SNDMORE)?;
        Ok(())
    }

    pub fn recv(&mut self, timeout: &mut Variant, ret_value: &mut Variant) -> AddinResult {
        let timeout = timeout.get_i32()? as i64;
        let socket = get_socket(self.socket.as_ref())?;
        if socket.poll(PollEvents::POLLIN, timeout)? != 1 {
            return Ok(());
        }

        socket.recv(&mut self.msg, zmq::DONTWAIT)?;
        ret_value.set_blob(&self.msg)?;

        Ok(())
    }

    pub fn recv_multipart(
        &mut self,
        timeout: &mut Variant,
        ret_value: &mut Variant,
    ) -> AddinResult {
        let timeout = timeout.get_i32()? as i64;
        let socket = get_socket(self.socket.as_ref())?;
        if socket.poll(PollEvents::POLLIN, timeout)? != 1 {
            return Ok(());
        }

        let mut count = 0;
        loop {
            if count + 1 > self.parts.len() {
                self.parts.push(Message::new());
            }
            socket.recv(&mut self.parts[count], zmq::DONTWAIT)?;
            count += 1;
            if !socket.get_rcvmore()? {
                break;
            }
        }

        self.parts.truncate(count);
        ret_value.set_i32(count as _);

        Ok(())
    }

    pub fn get_part(&mut self, part: &Variant<'_>, ret_value: &mut Variant<'_>) -> AddinResult {
        let part = part.get_i32()? as usize;
        if part >= self.parts.len() {
            return Err("Part does not exist".into());
        }
        ret_value.set_blob(&self.parts[part])?;
        Ok(())
    }

    pub fn subscribe(&mut self, data: &mut Variant) -> AddinResult {
        let data = data.get_blob()?;
        let socket = get_socket(self.socket.as_ref())?;
        socket.set_subscribe(data)?;
        Ok(())
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        if let Ok(mut state) = STATE.lock() {
            if state.context.is_some() {
                state.count -= 1;
                if state.count == 0 {
                    state.context = None;
                }
            }
        }
    }
}
