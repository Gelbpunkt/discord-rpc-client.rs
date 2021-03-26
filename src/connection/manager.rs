use std::{
    io::ErrorKind,
    sync::mpsc::{channel, Receiver, Sender},
    thread, time,
};

use super::{Connection, SocketConnection};
use error::{Error, Result};
use models::Message;

type Tx = Sender<Message>;
type Rx = Receiver<Message>;

pub struct Manager {
    client_id: u64,
    outbound: (Option<Rx>, Tx),
    inbound: (Rx, Option<Tx>),
}

impl Manager {
    pub fn new(client_id: u64) -> Self {
        let (sender_o, receiver_o) = channel();
        let (sender_i, receiver_i) = channel();

        Self {
            client_id,
            inbound: (receiver_i, Some(sender_i)),
            outbound: (Some(receiver_o), sender_o),
        }
    }

    pub fn start(&mut self) {
        let inbound = self.inbound.1.take().unwrap();
        let outbound = self.outbound.0.take().unwrap();
        let client_id = self.client_id;
        thread::spawn(move || {
            send_and_receive_loop(inbound, outbound, client_id);
        });
    }

    pub fn send(&self, message: Message) -> Result<()> {
        self.outbound.1.send(message).unwrap();
        Ok(())
    }

    pub fn recv(&self) -> Result<Message> {
        let message = self.inbound.0.recv()?;
        Ok(message)
    }

    pub fn try_recv(&self) -> Result<Message> {
        let message = self.inbound.0.try_recv()?;
        Ok(message)
    }
}

fn send_and_receive_loop(
    mut inbound: Sender<Message>,
    outbound: Receiver<Message>,
    client_id: u64,
) {
    debug!("Starting sender loop");
    let mut connection: Option<SocketConnection> = None;

    loop {
        if let Some(ref mut conn) = connection {
            match send_and_receive(conn, &mut inbound, &outbound) {
                Err(Error::IoError(ref err)) if err.kind() == ErrorKind::WouldBlock => (),
                Err(Error::IoError(_)) | Err(Error::ConnectionClosed) => connection = None,
                Err(why) => error!("error: {:?}", why),
                _ => (),
            }
        } else {
            connection = SocketConnection::connect().ok();
            if let Some(ref mut conn) = connection {
                if conn.handshake(client_id).is_err() {
                    connection = None;
                } else {
                    debug!("Successfully connected to socket");
                }
            }
        }
        thread::sleep(time::Duration::from_millis(500));
    }
}

fn send_and_receive(
    connection: &mut SocketConnection,
    inbound: &mut Tx,
    outbound: &Rx,
) -> Result<()> {
    while let Ok(msg) = outbound.try_recv() {
        connection.send(msg).unwrap_or(());
    }

    let msg = connection.recv()?;
    inbound.send(msg).unwrap_or(());

    Ok(())
}
