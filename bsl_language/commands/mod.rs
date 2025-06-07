use std::hash::{Hash, Hasher};


pub mod connect;
pub mod send;
pub mod receive;
pub mod close;
pub mod wr;
pub mod sc;

// Komutların ortak trait'i
pub trait Command {
    fn execute(&self, args: &str, context: &mut crate::ExecutionContext) -> Result<(), String>;
}

pub enum Value {
    Str(String),
    Int(i64),
    Bool(bool),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Str(a), Value::Str(b)) => a == b,
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for Value {}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::Str(s) => {
                0u8.hash(state); // Tür ayrımı için
                s.hash(state);
            }
            Value::Int(i) => {
                1u8.hash(state);
                i.hash(state);
            }
            Value::Bool(b) => {
                2u8.hash(state);
                b.hash(state);
            }
        }
    }
}