use std::io::{Read, Write};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::network::errors::*;
use tun::platform::Device;
use tun::Error;

extern crate tun;

pub struct NetworkHandler {
    device: Arc<Mutex<tun::platform::Device>>,
}

impl<'a> NetworkHandler {
    pub fn new() -> Result<NetworkHandler, Error> {
        let mut config = tun::Configuration::default();
        config
            .address((10, 0, 0, 1))
            .netmask((255, 255, 255, 0))
            .up();

        #[cfg(target_os = "linux")]
        config.platform(|config| {
            config.packet_information(true);
        });

        tun::create(&config).map(|dev| NetworkHandler {
            device: Arc::new(Mutex::new(dev)),
        })
    }

    pub fn start(&self, incoming: Sender<Vec<u8>>, outgoing: Receiver<Vec<u8>>) {
        let receiver_device = self.device.clone();
        let receiver = thread::spawn(move || {
            Self::run_receiver_job(receiver_device, incoming);
        });
        println!("receiver is spawned");

        let sender_device = self.device.clone();
        let sender = thread::spawn(move || {
            Self::run_sender_job(sender_device, outgoing);
        });
        println!("sender is spawned");

        let _ = receiver.join().unwrap();
        let _ = sender.join().unwrap();
    }

    fn run_receiver_job(device: Arc<Mutex<Device>>, channel: Sender<Vec<u8>>) {
        let mut buf = [0; 4096];

        loop {
            match device
                .lock()
                .map_err(|err| ReceivedError::from_locking(err.to_string()))
                .and_then(|mut device_guard| {
                    let res = device_guard
                        .read(&mut buf)
                        .map_err(|err| ReceivedError::from_reading(err.to_string()));
                    drop(device_guard);

                    res
                })
                .and_then(|amount| {
                    println!("received data, size={}", amount);

                    let mut received = vec![0u8; amount];
                    let mut i: usize = 0;

                    for byte in &buf[0..amount] {
                        received[i] = *byte;
                        i += 1;
                    }

                    Ok((received, amount))
                })
                .and_then(|(received, amount)| {
                    channel
                        .send(received.to_vec())
                        .map(|()| amount)
                        .map_err(|err| ReceivedError::from_sending(err.to_string()))
                }) {
                Ok(amount) => {
                    println!("received data, size={} [bytes]", amount);
                }
                Err(err) => {
                    println!("error happens on receiving, msg={}", err.to_string());
                }
            }

            thread::sleep(Duration::from_millis(100));
        }
    }

    fn run_sender_job(device: Arc<Mutex<Device>>, channel: Receiver<Vec<u8>>) {
        for packet in channel.iter() {
            match device
                .lock()
                .map_err(|err| SendingError::from_locking(err.to_string()))
                .and_then(|mut device_guard| {
                    let res = device_guard
                        .write(&packet)
                        .map_err(|err| SendingError::from_writing(err.to_string()))
                        .and_then(|_size: usize| {
                            device_guard
                                .flush()
                                .map_err(|err| SendingError::from_flushing(err.to_string()))
                        });

                    drop(device_guard);

                    res
                }) {
                Ok(_) => {
                    println!("sent data, size={} [bytes]", packet.len())
                }
                Err(err) => {
                    println!("error happens on sending data: {}", err.to_string());
                }
            }
        }
    }
}

impl Drop for NetworkHandler {
    fn drop(&mut self) {
        println!("Dropping NetworkHandler");
        // TODO: implement
    }
}
