use std::{env, process, str::FromStr};
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

use reqwest::header::AUTHORIZATION;
fn main() {
    const OPEN_AI_BASE: &str = "https://api.openai.com/v1/chat/completions";
    const SYSTEM_PROMPT: &str = "You are a shell assistant to provide the user with the most appropriate command which matches their intention. You will be given a short description for the task a user is trying to accomplish in a unix like shell and you must respond with a single command you think best applies to them. If what the user has requested cannot be achived using a single command, you should chain together as many tools and utilities to achieve the user goal. You must output a functioning command that is syntactically correct and doesnt cause any errors. And if you are uncertain or dont understand the prompt just say \"sorry, I cant understand your request\" and nothing else. You dont need to explain why you cannot reply.";

    let mut api_key = String::from("Bearer ");
    match env::var("SHELLY_OPENAI_KEY") {
        Ok(k) => api_key.push_str(&k),
        Err(_) => panic!("SHELLY_OPENAI_KEY is not set")
    }

    let sys_msg = Message {
        role: String::from("system"),
        content: String::from_str(SYSTEM_PROMPT).expect("Failed to unwrap")
    };

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: shelly \"Your prompt\"\n\nNot enough args.");
        process::exit(1);
    }

    let prompt = String::from(args[1].clone());

    let user_msg = Message {
        role: String::from("user"),
        content: prompt,
    };

    let comp = CompleteMessage {
        model: String::from("gpt-4"),
        messages: vec![sys_msg, user_msg],
    };

    let client = reqwest::blocking::Client::new();

    let body = client.post(OPEN_AI_BASE)
               .json(&comp)
               .header(AUTHORIZATION, api_key);
    let resp: Response = body.send().unwrap().json().expect("failed");
    println!("{}", resp.choices[0].message.content);
}
