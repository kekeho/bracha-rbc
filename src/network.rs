use std::io;
use tokio;
use tokio::net::UdpSocket;
use std::sync::Arc;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Message {
    Echo(Vec<u8>),
    Vote(Vec<u8>),
}


impl Message {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        match self {
            Self::Echo(value) => {
                result.push(0);
                result.extend_from_slice(value);
            }

            Self::Vote(value) => {
                result.push(1);
                result.extend_from_slice(value);
            }
        }
        result
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, io::Error> {
        if bytes.len() < 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid message: length of bytes < 2"))
        }

        match bytes[0] {
            0 => {
                Ok(Self::Echo(bytes[1..].to_vec()))
            }
            1 => {
                Ok(Self::Vote(bytes[1..].to_vec()))
            }
            _ => {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid message: unknown message type"))
            }
        }
    }
}


pub async fn broadcast(dests: Vec<String>, message: Message, socket: Arc<UdpSocket>) {
    for dest in dests {
        _ = socket.send_to(&message.to_bytes(), dest).await;
    }
}
