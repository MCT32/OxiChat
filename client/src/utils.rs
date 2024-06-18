use crossterm::{
    execute, queue,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, Clear, EnterAlternateScreen, LeaveAlternateScreen,
    },
    QueueableCommand,
};
use irc::config::IrcConfig;
use std::{
    borrow::Borrow,
    error::Error,
    fmt::{self, Display},
    io::stdout,
    net::{IpAddr, SocketAddr, ToSocketAddrs},
};

use rand::seq::SliceRandom;
use rand::thread_rng;

pub const FIRST_WORDS: [&str; 10] = [
    // for random name generation lol
    "Happy", "Funny", "Silly", "Clever", "Brave", "Giggly", "Cheeky", "Witty", "Daring", "Charming",
];

pub const SECOND_WORDS: [&str; 10] = [
    "Pirate", "Ninja", "Wizard", "Jedi", "Samurai", "Rockstar", "Guru", "Magician", "Sorcerer",
    "Master",
];

#[derive(Clone, PartialEq, Debug)]
pub struct Canvas {
    pub w: u16,
    pub h: u16,
    pub chat: Chat,
    pub domain: Domain,
    pub newline_index: u16,

    pub input: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Chat {
    pub messages: Vec<Message>,
    pub context: (u32, u32), // context (# - #) according to height of screen, also cannot display default message
}

#[derive(Clone, PartialEq, Debug)]
pub struct Message {
    pub timestamp: String,
    pub content: String,
    pub author: Author,
    pub pos: i32,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Author {
    pub text: String,
    pub color: Color,
    pub styled: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Domain {
    // chat domain
    pub a: (u32, u32), // top left point            a --------- b
    pub b: (u32, u32), // top right point             | area  |
    pub c: (u32, u32), // bottom right point          |       |
    pub d: (u32, u32), // bottom left point         d --------- c
}

#[derive(Clone, PartialEq, Debug)]
pub enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Arguments {
    nickname: Option<String>,
    server: Option<String>,
    port: Option<u16>,
}

pub enum ParseResult {
    Config(IrcConfig),
    Args(Arguments),
}

impl fmt::Debug for ParseResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseResult::Config(config) => write!(f, "Config({:?})", config),
            ParseResult::Args(args) => write!(f, "Args({:?})", args),
        }
    }
}

impl Canvas {
    pub fn init_canvas() -> Result<(std::io::Stdout, Self), Box<dyn std::error::Error>> {
        let mut stdout: std::io::Stdout = stdout();
        enable_raw_mode()?;
        execute!(
            stdout,
            EnterAlternateScreen,
            Clear(crossterm::terminal::ClearType::All)
        )?;
        let (w, h) = terminal::size().expect("Failed to get terminal size...");
        Ok((
            stdout,
            Self {
                w,
                h,
                chat: Chat::new(),
                domain: Domain::new(),
                newline_index: 0,
            },
        ))
    }
    pub fn leave_canvas(
        &mut self,
        mut stdout: std::io::Stdout,
    ) -> Result<(), Box<dyn std::error::Error>> {
        disable_raw_mode()?;
        execute!(
            stdout,
            Clear(crossterm::terminal::ClearType::All),
            LeaveAlternateScreen
        )?;
        Ok(())
    }
}

impl Chat {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            context: (0, 0),
        }
    }
}

impl Message {
    pub fn new() -> Self {
        // this returns a very default-looking message. it will be ignored by the renderer
        Self {
            timestamp: "%H/%M".to_owned(),
            content: "[ CONTENT ]".to_owned(),
            author: Author::new(),
            pos: -1, // this will be ignored
        }
    }
}

impl Author {
    pub fn new() -> Self {
        // very default looking author.
        Self {
            text: "[ AUTHOR ]".to_owned(),
            color: Color::Red,
            styled: "[TD]".to_owned(),
        }
    }
}

impl Domain {
    pub fn new() -> Self {
        // empty Domain
        Self {
            a: (0, 0),
            b: (10, 0),
            c: (10, 10),
            d: (0, 10),
            // 10x10 square
        }
    }
}

impl Arguments {
    pub fn new() -> Self {
        Self {
            nickname: None,
            server: None,
            port: None,
        }
    }
    pub fn parse_arguments(
        args: Vec<String>,
    ) -> Result<ParseResult, crate::error::ArgumentParseError> {
        match args.len() {
            2 => {
                let nickname = args.get(1).unwrap().clone();
                Ok(ParseResult::Args(Self {
                    nickname: Some(nickname),
                    server: None,
                    port: None,
                }))
            }
            4 => {
                let nickname = args.get(1).unwrap();
                let server = args.get(2).unwrap();
                let port = args.get(3).unwrap().parse::<u16>()?;

                /*
                let mut config = IrcConfigBuilder::new();

                config.server_address(format!("{}:{}", server, port))?;
                config.username(String::from(nickname.clone()));
                config.nickname(String::from(nickname.clone()));
                config.password(None);

                let config = config.build()?;
                */

                let config = IrcConfig {
                    // TODO: should prob find a better way to do this.
                    // Error handling is required too. was previously done by builder.
                    server_address: format!("{}:{}", server, port)
                        .to_socket_addrs()?
                        .next()
                        .unwrap(),
                    username: nickname.clone(), // confusing var names lol
                    nickname: None,             // No nickname, username is used
                    password: None,
                };

                println!("Config created: {:?}", config);

                Ok(ParseResult::Config(config))
            }
            _ => {
                let nickname = generate_random_name();
                let server = "irc.libera.chat:6667".to_owned();

                let config = IrcConfig {
                    // TODO: should prob find a better way to do this.
                    // Error handling is required too. was previously done by builder.
                    server_address: server.to_socket_addrs().unwrap().next().unwrap(),
                    username: nickname.clone(), // confusing var names lol
                    nickname: None,             // No nickname, username is used
                    password: None,
                };

                Ok(ParseResult::Config(config))
            }
        }
    }
}

pub fn generate_random_name() -> String {
    let mut rng = thread_rng();
    let first = FIRST_WORDS.choose(&mut rng).unwrap_or(&"");
    let second = SECOND_WORDS.choose(&mut rng).unwrap_or(&"");
    format!("{}{}", first, second)
}
