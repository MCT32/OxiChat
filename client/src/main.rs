use std::{io, time::Duration};
use irc::{
    self, config::IrcConfig, messages::{Message, Params}, users::{User, UserFlags}, IrcConnection
};
use tokio::{
    self,
    net::lookup_host,
    time::sleep,
};
use ncurses::*;

#[tokio::main]
async fn main() {

    initscr();
    raw();
    keypad(stdscr(), true);
    echo();

    let config = create_config().await;

    let prompt = "Input a channel to join: ";
    let mut channel = get_config_input(prompt);

    let mut connection = config.connect().await.unwrap();

    irc_client(&mut connection, channel.clone()).await;

    send_message(&mut connection, channel).await;

}

async fn create_config() -> IrcConfig {
    
    let nickname = get_config_input("Enter your NICK name:");

    let username = nickname.clone();
    let hostname = nickname.clone();
    let servername = nickname.clone();
    let realname = nickname.clone(); // TODO: all of this will come back after the tui...exists.

    let address = get_config_input("Enter the server address:");
    let port: u16 = get_config_input("Enter port:").parse().expect("Invalid port");

    IrcConfig::builder()
        .user(User {
            nickname,
            username,
            hostname,
            servername,
            realname,
            flags: UserFlags::default(),
        })
        //.password
        .set_receive_handler(|msg| { print_messages(msg) })
        .host(lookup_host(format!("{}:{}", address, port)).await.unwrap().next().unwrap()).unwrap()
}

async fn irc_client(connection: &mut IrcConnection, channel: String) {
    sleep(Duration::from_secs(5)).await;
    connection
        .send(Message {
            prefix: None,
            command: "JOIN".to_string(),
            params: Params(vec![
                format!("#{}", channel).to_string(),
            ]),
        })
        .await
        .unwrap();
    sleep(Duration::from_secs(2)).await;
    
}

async fn send_message(connection: &mut IrcConnection, channel: String) {
    println!("you're in the chat. | go ham. ");
    loop {
        let prompt = "";
        let input = get_message_input(prompt);
        if !input.is_empty() {
            connection
                .send(Message {
                prefix: None,
                command: "PRIVMSG".to_string(),
                params: Params(vec![
                    format!("#{}", channel).to_string(),
                    format!(":{}", input.to_string())
                ]),
            })
        .await
        .unwrap();
            if input == "QUIT" {
                connection.quit().await.unwrap();
                break
            } 
        }
    }
}

fn print_messages(msg: Message) {
    let prefix = msg.prefix.unwrap_or_else(|| String::new()); 
    let command = msg.command;
    let params = msg.params.0.join(" ");

    let output = format!("{:?}: {} {}\n", prefix, command, params);
    
    let (mut height, mut width) = (0, 0);
    getmaxyx(stdscr(), &mut height, &mut width);

    let box_height = height - 4; 
    let box_width = width - 1; 
    let box_y = 0;
    let box_x = 0; 

    let iwin = newwin(box_height, box_width, box_y, box_x);
    box_(iwin, 0, 0);
    wrefresh(iwin);

    mvwprintw(iwin, height - 2, 1, &output); // Print the message in the window
    wrefresh(iwin); // Refresh the window to display the message
}

fn get_config_input(prompt: &str) -> String {
    // Initialize ncurses
    initscr();
    raw();
    keypad(stdscr(), true);
    echo(); // Enable echoing user input

    let (mut height, mut width) = (0, 0);
    getmaxyx(stdscr(), &mut height, &mut width);

    let lbh = 3; 
    let lbw = width - 1; 
    let lby = height - lbh;
    let lbx = 0; 

    let iwin = newwin(lbh, lbw, lby, lbx);
    box_(iwin, 0, 0);
    mvwprintw(iwin, 1, 1, prompt);
    wrefresh(iwin);

    curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
    keypad(iwin, true);

    let mut config_input = String::new();
    mvwgetstr(iwin, 1, prompt.len() as i32 + 1, &mut config_input);

    // Cleanup ncurses
    delwin(iwin);
    endwin();

    config_input.trim().to_string()

}

fn get_message_input(prompt: &str) -> String {
    noecho(); 

    let (mut height, mut width) = (0, 0);
    getmaxyx(stdscr(), &mut height, &mut width);

    let lbh = 3; 
    let lbw = width - 1; 
    let lby = height - lbh;
    let lbx = 0; 

    let iwin = newwin(lbh, lbw, lby, lbx);
    box_(iwin, 0, 0);
    mvwprintw(iwin, 1, 1, prompt);
    wrefresh(iwin);

    curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
    keypad(iwin, true);

    let mut message_input = String::new();
    let mut ch: i32;
    loop {
        ch = wgetch(iwin);
        if ch == KEY_ENTER || ch == 10 || ch == 13 { 
            break;
        } else if ch == KEY_BACKSPACE || ch == 127 || ch == 8 { 
            if !message_input.is_empty() {
                message_input.pop(); 
                mvwdelch(iwin, 1, message_input.len() as i32 + 1); 
                wrefresh(iwin); 
            }
        } else if ch == ERR {
            continue; 
        } else {
            message_input.push(ch as u8 as char); 
            waddch(iwin, ch as u32); 
            wrefresh(iwin); 
        }
    }

    // Cleanup ncurses
    delwin(iwin);
    endwin();

    message_input.trim().to_string()
}
