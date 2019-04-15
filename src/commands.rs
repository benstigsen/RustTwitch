use std::io::prelude::*;
use std::fs::File;
extern crate reqwest;

// Handle Commands
pub fn handle_command(user: &str, msg: &str) -> String {
	let data: Vec<&str> = msg[1..].split_whitespace().collect();

	let response = match data[0] {
		// ADMIN / MODERATOR
		"disconnect" | "dc"	=> {disconnect(user); "".to_string()},

		// REGULAR
		"example" | "file"	=> response_from_file(),

		"ping" 				=> "Pong!".to_string(),
		"hug" 				=> format!("/me hugs {}!", user),

		// API
		"uptime"			=> uptime(),

		// DEFAULT
		_ 					=> format!("{} is not a recognized command!", data[0])
	};

	response
}

// ADMIN \\
fn disconnect(user: &str) {
	let allowed_users = [*crate::CHAN, "some_other_user"];	// Add users here that are allowed to use the "disconnect" command
	let allowed = allowed_users.contains(&user);

	if allowed {
		crate::send_msg("Disconnecting".to_string());
		crate::send_msg("/disconnect".to_string());
	}
}

// REGULAR \\
fn response_from_file() -> String {
	let mut file = File::open("src/responses/example.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    contents.replace("\n", " ").replace("\r", "")
}

// API \\
fn uptime() -> String {
	let url = format!("https://decapi.me/twitch/uptime/{}", *crate::CHAN);

 	let response = reqwest::get(&url).expect("Error").text().unwrap();

    response
}