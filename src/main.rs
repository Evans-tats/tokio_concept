use::tokio::time::{Delay, delay_for};
//use std::sync::mpsc::Sender;
use::std::time::Duration;
use::tokio::sync::mpsc::{channel, Sender, Receiver};

fn sleep(ms: u64) -> Delay {
    delay_for(Duration::from_millis(ms))
}

#[derive(Debug)]
enum Message {
    Hello,
    World,
    
}

async fn message_generator(mut channel: Sender<Message> ) {
    loop{
        match channel.send(Message::Hello).await {
            Ok(()) => sleep(100).await,
            Err(_) => {
                eprintln!("error sending message");
                break;
            }
        } 

    }
}

async fn file_sink(mut channel: Receiver<Message>) {
    loop {
        while let Some(msg) = channel.recv().await {
            println!("{:?}", msg);
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx,rx) = channel::<Message>(10);

    tokio::spawn(message_generator(tx));
    tokio::spawn(file_sink(rx));
    sleep(2000).await
}
