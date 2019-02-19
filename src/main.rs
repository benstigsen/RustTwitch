#[macro_use]
extern crate lazy_static;

mod commands;
mod credentials;

use std::net::TcpStream;
use std::io::{Write, BufReader, BufRead};

/* TO-DO: 
	- Add colors for messages, commands and responses
	- Add responses to a vector, then respond every X second (thread)
	- Load credentials from .txt file [DONE]
		- Clarify where the line AUTH, USER and CHAN variable should be
*/

lazy_static! {
    static ref SOCKET: TcpStream 		= TcpStream::connect("irc.chat.twitch.tv:6667").unwrap();
    static ref CREDENTIALS: Vec<String> = credentials::get_credentials("src/credentials.txt").unwrap();

    static ref AUTH: &'static str = &CREDENTIALS[0];
	static ref USER: &'static str = &CREDENTIALS[1];
	static ref CHAN: &'static str = &CREDENTIALS[2];
}


fn main() {
	// AUTHENTICATION
	send_raw(&format!("PASS {}", *AUTH));
	send_raw(&format!("NICK {}", *USER));
	send_raw(&format!("JOIN #{}", *CHAN));

	// CONNECTION ESTABLISHED
	send_msg("/me joined the chat!");
	println!("Connection to the channel #{} has been established!\n", *CHAN);

	let buffered = BufReader::new(&*SOCKET);

	let mut lines = buffered.lines();
	while let Some(Ok(line)) = lines.next() {

		dbg!(&line);

		if line.contains("PRIVMSG") {
			let user = &line[1..line.find("!").unwrap()];		// GET USERNAME
			let msg  =  line[1..].splitn(2, ':').nth(1).unwrap();	// GET MESSAGE

			match msg.chars().next() == Some('!') {
				true  => {recv_cmd(msg, user); commands::handle_command(user, msg)}, // COMMAND
				false => {recv_msg(msg, user);}	// MESSAGE
			};
		} else {
			match &line[..] {
				"PING :tmi.twitch.tv" => { recv_irc(&line); send_raw("PONG :tmi.twitch.tv") },
				_					  =>   recv_irc(&line)
			};
		};
	};
}

// MESSAGE
fn recv_msg(msg: &str, user: &str) {println!("[MESSAGE] {}: {}", user, msg)}

// COMMAND
fn recv_cmd(cmd: &str, user: &str) {println!("[COMMAND] {}: {}", user, cmd)}

// DATA FROM IRC
fn recv_irc(data: &str) {
	println!("[IRC DATA] {}", data);
}

// SEND CHAT MESSAGES
fn send_msg(data: &str) {
	let msg 	= String::from(format!("PRIVMSG #{} :{}\r\n", *CHAN, data));
	let result  = (&*SOCKET).write(msg.as_bytes()).expect("send_msg failed!");

	println!("[RESPONSE] [{:?}]: {}", result, data)
}

// SEND RAW DATA
fn send_raw(data: &str) {
	let msg 	= String::from(format!("{}\r\n", data));
	let result  = (&*SOCKET).write(msg.as_bytes()).expect("send_raw failed!");

	match data.contains("PASS oauth") {
		true  => println!("[RAW DATA] [{:?}]: {}{:*<30}", result, data[0..11].to_string(), ""),
		false => println!("[RAW DATA] [{:?}]: {}", result, data)
	};
}
