mod commands;
use crate::commands::Value;


use commands::{Command, connect, send, receive, close, wr, sc};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

pub struct ExecutionContext {
    pub stream: Option<std::net::TcpStream>,
    pub sets: HashMap<String, HashSet<Value>>,
}


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Kullanım: bsl <script.bsl>");
        return Ok(());
    }

    let filename = &args[1];
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut context = ExecutionContext {
        stream: None,
        sets: HashMap::new(),
    };

    for line_res in reader.lines() {
        let line = line_res?;
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        let command_name = parts[0];
        let args = if parts.len() > 1 { parts[1] } else { "" };

        let result = match command_name {
            "connect" => connect::ConnectCommand.execute(args, &mut context),
            "send" => send::SendCommand.execute(args, &mut context),
            "receive" => receive::ReceiveCommand.execute(args, &mut context),
            "close" => close::CloseCommand.execute(args, &mut context),
            "wr" => wr::PrintCommand::new().execute(args, &mut context),
            "sc" => sc::SetCommand::new().execute(args, &mut context),
            _ => Err(format!("Bilinmeyen komut: {}", command_name)),
        };

        if let Err(e) = result {
            println!("error: {}", e);
        }
    }

    Ok(())
}
