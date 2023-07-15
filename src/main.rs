use std::sync::mpsc::channel;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

mod network;

fn main() {
    println!("Hello, world!");

    match network::network::NetworkHandler::new() {
        Ok(network) => {
            let (incoming_tx, incoming_rx) = channel::<Vec<u8>>();
            let (outgoing_tx, outgoing_rx) = channel::<Vec<u8>>();

            let sub_thread = thread::spawn(move || {
                network.start(incoming_tx.clone(), outgoing_rx);
            });

            for packet in incoming_rx {
                println!("incoming packet: {:?}", packet);

                outgoing_tx.send(packet).unwrap();

                sleep(Duration::from_secs(1));
            }

            sub_thread.join().unwrap();
        }
        Err(err) => println!("error happen on initialization, msg={}", err.to_string()),
    }
}
