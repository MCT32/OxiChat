use crate::utils::Canvas;
use irc::config::{IrcConfig, IrcConfigBuilder};

#[derive(Clone, PartialEq, Debug)]
pub struct OxiChat<'a> {
    pub canvas: Canvas,
    pub config: IrcConfig<'a>,
}
