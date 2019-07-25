use super::decode::Decoder;
use crate::connection::{ConnContext, Connection};
use bytes::Bytes;
use failure::Error;

pub struct DeContext<'a> {
    pub conn: &'a mut ConnContext,
    pub decoder: Decoder<'a>,
}

impl<'a> DeContext<'a> {
    pub fn new(conn: &'a mut ConnContext, buf: &'a Bytes) -> Self {
        DeContext { conn, decoder: Decoder::new(&buf) }
    }
}

pub trait Deserialize: Sized {
    fn deserialize(ctx: &mut DeContext) -> Result<Self, Error>;
}
