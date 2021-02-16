use std::sync::mpsc;
use crossterm::{event::{poll, read, Event, KeyCode}, Result};
use std::time::Duration;
use std::thread;

pub enum KeyType {
    Quit,
    Pause,
    None,
}

pub fn run() -> mpsc::Receiver<KeyType> {
    let (sender, receiver) = mpsc::channel::<KeyType>();
    
    thread::spawn(move || -> Result<()> {
        loop {
            if poll(Duration::from_millis(1_000))? {
                let event = read()?;
                
                if event == Event::Key(KeyCode::Char('q').into()) {
                    sender.send(KeyType::Quit).unwrap();
                }
                
                if event == Event::Key(KeyCode::Char(' ').into()) {
                    sender.send(KeyType::Pause).unwrap();
                }
            }
        }

    });

    receiver
}

pub fn handle_input(receiver: &mpsc::Receiver<KeyType>) -> KeyType {
    match receiver.try_recv() {
        Ok(KeyType::Pause) => KeyType::Pause,
        Ok(KeyType::Quit) => KeyType::Quit,
        _ => KeyType::None,
    }
}
