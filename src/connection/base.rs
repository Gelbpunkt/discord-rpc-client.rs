use std::{
    io::{ErrorKind, Read, Write},
    marker::Sized,
    path::PathBuf,
    thread, time,
};

use bytes::BytesMut;

use error::{Error, Result};
use models::{
    message::{Message, OpCode},
    payload::Payload,
    ReadyEvent,
};
use utils;

pub trait Connection: Sized {
    type Socket: Write + Read;

    fn socket(&mut self) -> &mut Self::Socket;

    fn ipc_path() -> PathBuf;

    fn connect() -> Result<Self>;

    fn socket_path(n: u8) -> PathBuf {
        Self::ipc_path().join(format!("discord-ipc-{}", n))
    }

    fn handshake(&mut self, client_id: u64) -> Result<Payload<ReadyEvent>> {
        let hs = json![{
            "client_id": client_id.to_string(),
            "v": 1,
            "nonce": utils::nonce()
        }];

        loop {
            match self.send(Message::new(OpCode::Handshake, hs.clone())) {
                Ok(v) => break,
                Err(Error::IoError(ref err)) if err.kind() == ErrorKind::WouldBlock => (),
                Err(why) => return Err(why),
            }

            thread::sleep(time::Duration::from_millis(500));
        }

        loop {
            match self.recv() {
                Ok(v) => return Ok(serde_json::from_str(&v.payload).unwrap()),
                Err(Error::IoError(ref err)) if err.kind() == ErrorKind::WouldBlock => (),
                Err(why) => return Err(why),
            }

            thread::sleep(time::Duration::from_millis(500));
        }
    }

    fn ping(&mut self) -> Result<OpCode> {
        let message = Message::new(OpCode::Ping, json![{}]);
        self.send(message)?;
        let response = self.recv()?;
        Ok(response.opcode)
    }

    fn send(&mut self, message: Message) -> Result<()> {
        match message.encode() {
            Err(why) => error!("{:?}", why),
            Ok(bytes) => {
                self.socket().write_all(bytes.as_ref())?;
            }
        };
        debug!("-> {:?}", message);
        Ok(())
    }

    fn recv(&mut self) -> Result<Message> {
        let mut buf = BytesMut::new();
        buf.resize(1024, 0);
        let n = self.socket().read(&mut buf)?;
        debug!("Received {} bytes", n);

        if n == 0 {
            return Err(Error::ConnectionClosed);
        }

        buf = buf.split_to(n);
        let message = Message::decode(&buf)?;
        debug!("<- {:?}", message);

        Ok(message)
    }
}
