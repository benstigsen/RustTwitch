#[macro_use]
extern crate lazy_static;

mod coloro;
mod commands;
mod credentials;
mod configuration;

use std::io::{Write, BufReader, BufRead};
use std::{sync::mpsc, thread, time};
use std::net::TcpStream;

use coloro::color;
use configuration::*;

// SOCKET AND AUTHENTICATION CREDENTIALS
lazy_static! {
    static ref SOCKET: TcpStream 		= TcpStream::connect("irc.chat.twitch.tv:6667").unwrap();
    static ref CREDENTIALS: Vec<String> = credentials::get_credentials("src/credentials.txt").unwrap();

    static ref AUTH: &'static str = &CREDENTIALS[0];
	static ref USER: &'static str = &CREDENTIALS[1];
	static ref CHAN: &'static str = &CREDENTIALS[2];
}

fn main() {

	// Authentication
	send_raw(&format!("PASS {}", *AUTH));
	send_raw(&format!("NICK {}", *USER));
	send_raw(&format!("JOIN #{}", *CHAN));

	// Connection Established
	send_msg("/me joined the chat!".to_string());
	println!("Connection to the channel #{} has been established!\n", *CHAN);
	
	// Setup Thread Message Queueing
	let (sender, receiver) = mpsc::sync_channel(MAX_RESPONSE_QUEUE);

    thread::spawn(move || {

        while let Ok(message) = receiver.recv() {

	        thread::sleep(time::Duration::from_millis(DELAY_BETWEEN_REQUESTS));
	        send_msg(message);
        }
    });

    // Setup Buffer
	let buffered = BufReader::new(&*SOCKET); // Buffer which contains every line received from the IRC
	
	let mut lines = buffered.lines();
	while let Some(Ok(line)) = lines.next() {

		// Chat Message
		if line.contains("PRIVMSG") {
			let user = &line[1..line.find("!").unwrap()];			// GET USERNAME
			let msg  =  line[1..].splitn(2, ':').nth(1).unwrap();	// GET MESSAGE


			let response = match msg.chars().next() == Some(PREFIX)
			{
				// Command
				true => {recv_cmd(msg, user); commands::handle_command(user, msg)},
				
				// Message
				false => {recv_msg(msg, user); "".to_string()}
			};
			
			// Send response if there is any
			if response != "".to_string()
			{
				sender.send(response.to_string()).unwrap();
			}

		// IRC Data
		} else {

			// Respond to Twitch IRC "PING" message
			match &line[..] {
				"PING :tmi.twitch.tv" => { recv_irc(&line); send_raw("PONG :tmi.twitch.tv") }, // TWITCH IRC PING MESSAGE
				_					  =>   recv_irc(&line)
			};
		};
	};
}

// MESSAGE
fn recv_msg(user: &str, msg: &str) 
{
	println!("{} {}: {}", color("green", "[MESSAGE]"), user, msg);
}

// COMMAND
fn recv_cmd(user: &str, cmd: &str) 
{
	println!("{} {}: {}", color("blue", "[COMMAND]"), user, cmd);
}

// DATA FROM IRC
fn recv_irc(data: &str) {

	// Exclude user messages and bot messages from showing up as if it was received from the IRC 
	if !data.contains("PRIVMSG") && !data.contains(*USER)
	{
		println!("{} {}", color("red", "[IRCDATA]"), data);
	}
}

// SEND CHAT MESSAGES
fn send_msg(data: String) {
	let msg 	= String::from(format!("PRIVMSG #{} :{}\r\n", *CHAN, data));
	let _result  = (&*SOCKET).write(msg.as_bytes()).expect("send_msg failed!");

	println!("{} {}", color("yellow", "[SENDING]"), data);
}

// SEND RAW DATA
fn send_raw(data: &str) {
	let msg = String::from(format!("{}\r\n", data));
	let _result = (&*SOCKET).write(msg.as_bytes()).expect("send_raw failed!"); // Send data
}
