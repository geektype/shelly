mod config;

use config::Config;
use std::str::FromStr;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String
}

#[derive(Debug, Serialize, Deserialize)]
struct CompleteMessage {
    model: String,
    messages: Vec<Message>
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message
}

#[derive(Debug, Deserialize)]
struct Response {
    choices: Vec<Choice>
}

use reqwest::{header::AUTHORIZATION, StatusCode};

use crate::config::ConfigError;

const OPEN_AI_BASE: &str = "https://api.openai.com/v1/chat/completions";
const SYSTEM_PROMPT: &str = "You are a shell assistant to provide the user with the most appropriate command which matches their intention. You will be given a short description for the task a user is trying to accomplish in a unix like shell and you must respond with a single command you think best applies to them. If what the user has requested cannot be achived using a single command, you should chain together as many tools and utilities to achieve the user goal. You must output a functioning command that is syntactically correct and doesnt cause any errors. And if you are uncertain or dont understand the prompt just say \"sorry, I cant understand your request\" and nothing else. You dont need to explain why you cannot reply.";

const VERSION:&str = "0.0.1-dev";
fn main() {
    let mut pargs = pico_args::Arguments::from_env();
    if pargs.contains(["-h", "--help"]) {
        println!("Usage: shelly \"Your prompt\"");
        std::process::exit(0);
    }

    if pargs.contains(["-v", "--version"]) {
        println!("shelly {}", VERSION);
        std::process::exit(0);
    }
    
    let cfg = match Config::load() {
        Ok(a) => a,
        Err(ConfigError::MissingApiKey) => {
            eprintln!("API Key not found. Make sure OPEN_AI_KEY is set.");
            std::process::exit(1);
        }
        Err(ConfigError::MissingPrompt) => {
            eprint!("No prompt given");
            std::process::exit(1); 
        }
    };


    let sys_msg = Message {
        role: String::from("system"),
        content: String::from_str(SYSTEM_PROMPT).expect("Failed to unwrap")
    };

    let user_msg = Message {
        role: String::from("user"),
        content: cfg.prompt.clone()
    };

    let comp = CompleteMessage {
        model: String::from("gpt-4"),
        messages: vec![sys_msg, user_msg],
    };

    let client = reqwest::blocking::Client::new();

    let body = client.post(OPEN_AI_BASE)
               .json(&comp)
               .header(AUTHORIZATION, format!("Bearer {}", cfg.api_key));
    let resp = body.send().unwrap();
    match resp.status() {
        StatusCode::OK => {}

        StatusCode::UNAUTHORIZED => {
            eprintln!("Invalid API Key provided");
            std::process::exit(1);
        }
        _ => {
            eprintln!("Unhandled error from API");
            std::process::exit(1);
        }
    } 

    let resp_decoded = resp.json::<Response>().unwrap();
    
    println!("{}", resp_decoded.choices[0].message.content);
}
