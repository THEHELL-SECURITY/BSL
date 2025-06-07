use crate::commands::{Command, Value};

pub struct PrintCommand;

impl PrintCommand {
    pub fn new() -> Self {
        PrintCommand
    }
}

impl Command for PrintCommand {
    fn execute(&self, args: &str, context: &mut crate::ExecutionContext) -> Result<(), String> {
        let arg = args.trim();

        if arg.starts_with('(') && arg.ends_with(')') {
            let cluster_name = &arg[1..arg.len() - 1];
            

            if let Some(set) = context.sets.get(cluster_name) {
                
                if set.is_empty() {
                    println!("<boş>");
                } else if set.len() == 1 {
                    let val = set.iter().next().unwrap();
                    match val {
                        Value::Int(i) => println!("{}", i),
                        Value::Str(s) => println!("{}", s),
                        Value::Bool(b) => println!("{}", b),
                    }
                } else {
                    let mut first = true;
                    for val in set {
                        if !first {
                            print!(", ");
                        }
                        first = false;
                        match val {
                            Value::Int(i) => print!("{}", i),
                            Value::Str(s) => print!("{}", s),
                            Value::Bool(b) => print!("{}", b),
                        }
                    }
                    println!();
                }
                Ok(())
            } else {
                Err(format!("not found: {}", cluster_name))
            }
        } else {
            println!("{}", arg);
            Ok(())
        }
    }
}
