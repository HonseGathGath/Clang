use std::sync::mpsc::{self, Receiver, Sender};
use std::task::Context;
use std::thread::{self, JoinHandle};

#[derive(Debug)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug)]
pub struct Message {
    pub content: String,
    pub sender_id: u32,
    pub priority: Priority,
}

pub fn create_message_channel() -> (Sender<Message>, Receiver<Message>) {
    // 1. Implement this function to create and return a message channel
    mpsc::channel()
}

pub fn create_producer_thread(messages: Vec<Message>, tx: Sender<Message>) -> JoinHandle<()> {
    // TODO: Create a thread that:
    // - Updates the priority based on content
    // - Sends the updated message through the channel
    thread::spawn(move || {
        messages.into_iter().for_each(|message| {
            let priority: Priority = if message.content.contains("ERROR") {
                Priority::Critical
            } else if message.content.contains("WARNING") {
                Priority::High
            } else if message.content.contains("DEBUG") {
                Priority::Medium
            } else {
                Priority::Low
            };

            let msg: Message = Message {
                content: message.content,
                sender_id: message.sender_id,
                priority: priority,
            };

            tx.send(msg).unwrap();
        });
    })
}

pub fn create_consumer_thread(rx: Receiver<Message>) -> JoinHandle<Vec<String>> {
    // TODO: Create a thread that:
    // - Receives messages from the channel
    // - Formats them as "[PRIORITY|SENDER_ID] CONTENT"
    // - Returns a vector of formatted messages
    //
    thread::spawn(move || {
        rx.iter()
            .map(|msg| {
                let prio: &str = match msg.priority {
                    Priority::Critical => "CRIT",
                    Priority::High => "HIGH",
                    Priority::Low => "LOW",
                    Priority::Medium => "MED",
                };
                format!("[{}|{}] {}", prio, msg.sender_id, msg.content)
            })
            .collect::<Vec<String>>()
    })
}

// Example Usage
pub fn main() {
    let (tx, rx) = create_message_channel();

    let mut producer_handles = vec![];
    for id in 0..3 {
        let tx_clone = tx.clone();
        let messages = vec![
            Message {
                content: format!("Normal message from producer {}", id),
                sender_id: id,
                priority: Priority::Low,
            },
            Message {
                content: format!("WARNING: System running hot on producer {}", id),
                sender_id: id,
                priority: Priority::Low,
            },
            Message {
                content: format!("ERROR: Connection lost on producer {}", id),
                sender_id: id,
                priority: Priority::Low,
            },
        ];
        let handle = create_producer_thread(messages, tx_clone);
        producer_handles.push(handle);
    }

    drop(tx);
    let consumer_handle = create_consumer_thread(rx);

    for handle in producer_handles {
        handle.join().unwrap();
    }

    let results = consumer_handle.join().unwrap();
    println!("Processed messages:");
    for msg in results {
        println!("{}", msg);
    }
}
