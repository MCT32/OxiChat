use std::error::Error;

use crate::utils::Canvas;
use irc::config::{IrcConfig, IrcConfigBuilder};

#[derive(Clone, PartialEq, Debug)]
pub struct OxiChat<'a> {
    pub canvas: Canvas,
    pub config: IrcConfig<'a>,
}

impl<'a> OxiChat<'a> {
    pub fn construct() /* -> Result<Self, Box<dyn Error>> */
    {
        todo!()
        //Ok(Self {

        //})
    }
}
