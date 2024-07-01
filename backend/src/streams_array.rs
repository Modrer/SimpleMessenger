use std::collections::HashMap;
use std::sync::Mutex;

use futures_channel::mpsc::Sender;
use rocket::futures::SinkExt;

use crate::messages_stream::Message;

pub struct StreamsArray {
    pub array: Mutex<HashMap<i32, Vec<Sender<Message>>>>,
}

impl StreamsArray {
    pub fn new() -> Self {
        StreamsArray {
            array: Default::default(),
        }
    }
    pub fn insert(&mut self, id: i32, sender: Sender<Message>) {
        let array = self.array.get_mut();

        let array = array.unwrap().get_mut(&id);

        match array {
            Some(vec) => vec.push(sender),

            None => {
                let vec = vec![sender];
                self.array.lock().unwrap().insert(id, vec);
            }
        }
        return;
    }

    pub fn remove(&mut self, id: i32, sender: &Sender<Message>) {
        let array = self.array.get_mut();

        let array = array.unwrap().get_mut(&id);

        match array {
            Some(vec) => {
                vec.retain(|x| !x.same_receiver(sender));
                if vec.is_empty() {
                    self.array.lock().unwrap().remove(&id);
                }
            }

            None => {
                return;
            }
        }
    }

    pub async fn send(&mut self, id: i32, message: Message) {
        let array = self.array.get_mut();
        let array = array.unwrap().get_mut(&id);

        match array {
            None => return,
            Some(vec) => {
                for sender in vec {
                    let _ = sender.send(message.clone()).await;
                }
            }
        }
        return;
    }
}
