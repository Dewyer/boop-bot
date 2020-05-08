use std::time::Duration;
use ircclient::message::IrcMessage;
use crate::command::Command;

mod ircclient;
mod command;

fn main() {
    println!("Hello, world!");
    let config = ircclient::Config{
        nickname:String::from("boop-bot"),
        server:String::from("chat.freenode.net"),
        port:None
    };
    let mut irc = ircclient::IrcClient::new(config);
    irc.connect().unwrap();
    irc.join_channel("#test-boop").unwrap();

    let mut counter = 0;
    loop
    {
        let msg = irc.read_next_message();
        if msg.is_ok()
        {
            let msg = msg.unwrap();

            match msg
            {
                IrcMessage::Message(priv_msg)=>
                {
                    println!("got priv msg from : ({}) -> ({})",priv_msg.author_nick,priv_msg.content);

                    if let Some(chat_cmd) = Command::from(&priv_msg.content,"!")
                    {
                        println!("got cmd {:?}",chat_cmd);
                    }
                }
                IrcMessage::Other(ss) =>
                {
                    println!("got msg : |||{}",ss);
                }
                _ => ()
            }
        }
        else
        {
            panic!();
        }

        std::thread::sleep(Duration::from_millis(300));
        counter+=1;
        if counter % 3 == 0
        {
            irc.send_ping().unwrap();
        }
    }
}
