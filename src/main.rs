use std::env;

use serenity::async_trait;
use serenity::http::CacheHttp;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
// use serenity::model::interactions::message_component;
use serenity::prelude::*;

struct Handler;


pub struct MessageTubmler {
    messages: Vec<Message>,
    maxlen: usize
}

impl MessageTubmler {
    pub fn get_matching_message(&self, substr: &String) -> Option<&Message>{
        for m in self.messages.iter().rev() {
            if m.content.contains(substr){
                return Some(m)
            } else {
                // println!("wrong {}",m.content)
            }
        }
        None
    }

    pub fn push_msg(&mut self, msg: &Message){
        self.messages.push(msg.clone());
        if self.messages.len() > self.maxlen {
            self.messages.pop();
        }
    }
}

static mut tumbler: MessageTubmler = MessageTubmler {
    maxlen: 100,
    messages: Vec::new()
};

fn ironize_string(s: &String) -> String {
    let s_lettered = s.split("");
    let s_ironized = s_lettered.fold(
        String::new(),
        |acc, x| {
            if rand::random() {
                acc + &x.to_uppercase()
            } else {
                acc + &x.to_lowercase()
            }
            
        }
    );
    return s_ironized;
}

async fn handle_mold_command(args: &String, ctx: &Context) -> Option<String> {
    if args.len() < 3 {
        Some(String::from("cmon bruv, I'm not matching a random shit from allover the server"))
    } else {
        println!("molding <{}>",args);
        unsafe {
            let message_to_be_molded = tumbler.get_matching_message(&args);

            match message_to_be_molded {
                Some(x) => {
                    if let Err(why) = x.reply(&ctx.http, ironize_string(&x.content)).await {
                        println!("Error sending message: {:?}", why);
                    }
                    None
                }
                None => {
                    Some(String::from("This kind of message was not found"))
                }
            }
        }
    }
}

async fn handle_bot_shittalk(msg: &Message, ctx: &Context){
    
    let rand_u8 = rand::random::<u8>();
    if rand_u8 > 7 {        
        return;
    }
    println!("Molding a bot: {}", msg.author.name);

    if let Err(why) = msg.reply(
        &ctx.http,
        ironize_string(&msg.content)
    ).await {
        println!("Error sending message: {:?}", why);
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {

        if msg.is_own(&ctx.cache) {
            // println!("is own! {}",msg.content);
            return;
        }

        if msg.author.bot {
            handle_bot_shittalk(&msg, &ctx).await;
            return;
        }
        let first_word = msg.content
            .split_whitespace()
            .next()
            .unwrap_or("");
        
        let rest: String = msg.content
            .replace(&first_word, "")
            .trim_start_matches(char::is_whitespace)
            .to_string();
        
        let err_message :Option<String> = match first_word {
            "!mold" => handle_mold_command(&rest, &ctx).await,
            _ => {
                println!("{}",msg.content);
                unsafe{
                    tumbler.push_msg(&msg);
                }
                None
            }
        };
        
        match err_message {
            Some(x) => {
                if let Err(why) = msg.reply(&ctx.http, x).await {
                    println!("Error sending message: {:?}", why);
                }    
            },
            None => return
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
