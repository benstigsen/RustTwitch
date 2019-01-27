#[macro_use]
extern crate lazy_static;

mod commands;
mod credentials;

use credentials::*;
use std::net::TcpStream;
use std::io::{Write, BufReader, BufRead};

/* TO-DO: 
	- Add colors for messages, commands and responses
	- Add responses to a vector, then respond every X second (thread)
*/

lazy_static! {
    static ref SOCKET: TcpStream = TcpStream::connect("irc.chat.twitch.tv:6667").unwrap();
}

fn main() {

	// AUTHENTICATION
	send_raw(&format!("PASS {}", AUTH));
	send_raw(&format!("NICK {}", USER));
	send_raw(&format!("JOIN #{}", CHAN));

	// CONNECTION ESTABLISHED
	send_msg("/me joined the chat!");
	println!("Connection to the channel #{} has been established!\n", CHAN);

	let buffered = BufReader::new(&*SOCKET);

	let mut lines = buffered.lines();
	while let Some(Ok(line)) = lines.next() {

		dbg!(&line);

		if line.contains("PRIVMSG") {
			let user = &line[1..line.find("!").unwrap()];		// GET USERNAME
			let msg = line[1..].splitn(2, ':').nth(1).unwrap();	// GET MESSAGE

			match msg.chars().next() == Some('!') {
				true  => {println!("[COMMAND] {}: {}", user, msg); 	// COMMAND
						  commands::handle_command(user, msg)},
				false => println!("[MESSAGE] {}: {}", user, msg)	// MESSAGE
			};
		};

		if &line == "PING :tmi.twitch.tv" {
			println!("[IRC DATA] PING :tmi.twitch.tv");
			send_raw("PONG :tmi.twitch.tv");
		};
	}
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

// SEND CHAT MESSAGES
fn send_msg(data: &str) {
	let msg 	= String::from(format!("PRIVMSG #{} :{}\r\n", CHAN, data));
	let result  = (&*SOCKET).write(msg.as_bytes()).expect("send_msg failed!");

	println!("[RESPONSE] [{:?}]: {}", result, data)
}
