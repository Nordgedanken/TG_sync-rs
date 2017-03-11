extern crate telebot;
extern crate slack;
extern crate tokio_core;
extern crate futures;

use telebot::bot;
use tokio_core::reactor::Core;
use futures::stream::Stream;

// import all available functions
use telebot::functions::*;

struct MyHandler;

#[allow(unused_variables)]
impl slack::EventHandler for MyHandler {
    fn on_event(&mut self, cli: &mut slack::RtmClient, event: Result<slack::Event, slack::Error>, raw_json: &str) {
        println!("on_event(event: {:?}, raw_json: {:?})", event, raw_json);
    }

    fn on_ping(&mut self, cli: &mut slack::RtmClient) {
        println!("on_ping");
    }

    fn on_close(&mut self, cli: &mut slack::RtmClient) {
        println!("on_close");
    }

    fn on_connect(&mut self, cli: &mut slack::RtmClient) {
        println!("on_connect");
        // Do a few things using the api:
        // send a message over the real time api websocket
        let _ = cli.send_message("#testing_syncotronic", "Hello world! (rtm)");
        // post a message as a user to the web api
        let _ = cli.post_message("#testing_syncotronic", "hello world! (postMessage)", None);
        // set a channel topic via the web api
        // let _ = cli.set_topic("#general", "bots rule!");
    }
}

fn main() {
    slack();
    tg();
}

fn slack() {
    let args: Vec<String> = std::env::args().collect();
    let api_key = match args.len() {
        0 | 1 => panic!("No api-key in args! Usage: cargo run -- <api-key>"),
        x => {
            args[x - 1].clone()
        }
    };
    let mut handler = MyHandler;
    let mut cli = slack::RtmClient::new(&api_key);
    let r = cli.login_and_run::<MyHandler>(&mut handler);
    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    }
    println!("{}", cli.get_name().unwrap());
    println!("{}", cli.get_team().unwrap().name);
}

fn tg() {
    let mut lp = Core::new().unwrap();
    let bot = bot::RcBot::new(lp.handle(), "315066220:AAFGOUmSvq0T725a6Ke4C-zmfx80m_PcsxM")
        .update_interval(200);

    let handle = bot.new_cmd("/reply")
        .and_then(|(bot, msg)| {
            let mut text = msg.text.unwrap().clone();
            if text.is_empty() {
                text = "<empty>".into();
            }

            bot.message(msg.chat.id, text).send()
        });

    bot.register(handle);

    bot.run(&mut lp).unwrap();
}
