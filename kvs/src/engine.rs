// use std::ops::DerefMut;
use crate::err::Result;

pub trait KvsEngine: Send + Sync {
    fn set(&mut self, key: String, value: String) -> Result<()>;
    fn get(&self, key: String) -> Result<Option<String>>;
    fn remove(&mut self, key: String) -> Result<Option<String>>;

    fn name(&self) -> &'static str;
}
