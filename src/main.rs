mod commands;

use std::io::{Write, BufReader, BufRead};
use std::net::TcpStream;

const AUTH: &str = "oauth:someVeryLongTokenHere";
const USER: &str = "Botname";
const CHAN: &str = "channel"; // Lowercase (No "#" in front)

/* TO-DO: 
    - Add colors for messages, commands and responses
    - Add responses to a vector, then respond every X second (thread)
*/

fn main() {
	let mut socket = TcpStream::connect("irc.chat.twitch.tv:6667").unwrap();

	send_raw(&mut socket, &format!("PASS {}", AUTH));
    send_raw(&mut socket, &format!("NICK {}", USER));
	send_raw(&mut socket, &format!("JOIN #{}", CHAN));

    send_msg(&socket, "/me joined the chat!");
    println!("Connection to the channel #{} has been established!", CHAN);

    let buffered = BufReader::new(&socket);

    let mut lines = buffered.lines();
    while let Some(Ok(line)) = lines.next() {

    	if &line == "PING :tmi.twitch.tv\r\n" {
    		println!("Received PING message");
    	}

    	if line.contains("PRIVMSG") {
            let user = &line[1..line.find("!").unwrap()];
    		let msg = line[1..].splitn(2, ':').nth(1).unwrap();

    		if msg.chars().next() == Some('!') {
    			println!("[COMMAND] {}: {}", user, msg);

                let response = commands::handle_command(user, msg);
                if response.0 == true {
                    send_msg(&socket, &response.1);
                    println!("[RESPONSE] {}", response.1)
                };
    		} else {
    			println!("[MESSAGE] {}: {}", user, msg);
    		}
    	}
    }
}

fn send_raw(socket: &mut TcpStream, data: &str) {
	let msg = String::from(format!("{}\r\n", data));

	let result = socket.write(msg.as_bytes());
	println!("{:?}", result);
}

fn send_msg(mut socket: &TcpStream, data: &str) {
	let msg = String::from(format!("PRIVMSG #{} :{}\r\n", CHAN, data));

	socket.write(msg.as_bytes());
}
