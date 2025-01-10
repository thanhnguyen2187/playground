use crate::{Command, CommandResponse};
// use std::ops::DerefMut;
use crate::err::Result;

pub trait KvsEngine: Send + Sync {
    fn set(&mut self, key: String, value: String) -> Result<()>;
    fn get(&self, key: String) -> Result<Option<String>>;
    fn remove(&mut self, key: String) -> Result<Option<String>>;

    fn name(&self) -> &'static str;
}

pub fn evaluate_command(command: &Command, store: &mut dyn KvsEngine) -> Result<CommandResponse> {
    match command {
        Command::Get { key } => Ok(CommandResponse::Get {
            value: store.get(key.clone())?,
        }),
        Command::Set { key, value } => {
            store.set(key.clone(), value.clone())?;
            Ok(CommandResponse::Set {})
        }
        Command::Rm { key } => {
            store.remove(key.clone())?;
            Ok(CommandResponse::Rm {
                value: Some(key.clone()),
            })
        }
    }
}
