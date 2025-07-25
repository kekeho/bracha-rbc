use std::{collections::{HashMap, HashSet}, env::args, io, sync::Arc};

use tokio::{self, net::UdpSocket};

use crate::network::{broadcast, Message};

mod network;


fn calc_f(nodes_count: u64) -> u64 {
    (nodes_count-1) / 3
}


async fn receiving_thread(socket: Arc<UdpSocket>, dests: Vec<String>, byzantine: bool) {
    // ASSUMPTION: source addresses are authenticated by link
    let mut buffer: [u8; 1024] = [0; 1024];
    let mut received_message: HashMap<Message, HashSet<String>> = HashMap::new();  // message, from
    let n = dests.len() as u64 + 1;
    let f = calc_f(n);
    loop {
        match socket.recv_from(&mut buffer).await {
            Ok((length, addr)) => {
                match Message::from_bytes(&buffer[..length]) {
                    Ok(message) => {
                        let count: u64;
                        match received_message.get_mut(&message) {
                            Some(from) => {
                                if from.contains(&addr.to_string()) {
                                    continue;
                                }
                                from.insert(addr.to_string());
                                count = from.len() as u64;
                            }
                            None => {
                                let mut from = HashSet::new();
                                from.insert(addr.to_string());
                                _ = received_message.insert(message.clone(), from);
                                count = 1;
                            }
                        }

                        if byzantine {
                            broadcast(dests.clone(), Message::Vote("FAKE FAKE FAKE".to_string().into_bytes()), socket.clone()).await;
                            continue;
                        }
                        
                        match &message {
                            Message::Echo(value) => {
                                if count == 1 {
                                    broadcast(dests.clone(), message, socket.clone()).await;
                                } else if count == (n+f+1)/2 {
                                    // 2f+1 messages are collected
                                    broadcast(dests.clone(), Message::Vote(value.clone()), socket.clone()).await;
                                }
                            }

                            Message::Vote(value) => {
                                if count == f+1 {
                                    broadcast(dests.clone(), message.clone(), socket.clone()).await;
                                } else if count == 2*f+1 {
                                    // 2f + 1 messages are collected
                                    // deliver
                                    println!("DELIVER {:?}", String::from_utf8(value.clone()).unwrap_or("undecoded message".to_string()));
                                }
                            }
                        }
                    }
                    Err(_) => { continue; }
                }
            }
            _ => {}
        }
    }
    
}




#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let args: Vec<String> = args().collect();
    let socket: Arc<UdpSocket> = Arc::new(UdpSocket::bind(args[2].to_string()).await?);
    let byzantine: bool = args[1] == "byzantine";
    let destinations = args[2..].to_vec();  // includes myself

    // Receive messages
    tokio::spawn(receiving_thread(socket.clone(), destinations.clone(), byzantine));

    // Send proposal
    loop {
        let mut user_input = String::new();
        _ = std::io::stdin().read_line(&mut user_input)?;
        user_input = user_input.trim().to_string();
        tokio::spawn(broadcast(destinations.clone(), Message::Echo(user_input.into_bytes()), socket.clone()));
    }
}
