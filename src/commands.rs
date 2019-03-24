use std::io::prelude::*;
use std::fs::File;

pub fn handle_command(user: &str, msg: &str) {
	let data: Vec<&str> = msg[1..].split_whitespace().collect();

	match data[0] {
		// ADMIN / MODERATOR
		"disconnect" | "dc"		=> disconnect(user),

		// REGULAR
		"example" | "from_file"	=> response_from_file(),
		"ping" 					=> ping(),
		"hug" 					=> hug(user),

		_ 			=> ()
	};
}

// ADMIN \\
fn disconnect(user: &str) {
	let allowed_users = [*crate::CHAN, "some_other_user"];	// Add users here that are allowed to use the "disconnect" command
	let allowed = allowed_users.contains(&user);

	if allowed {
		crate::send_msg("Disconnecting");
		crate::send_msg("/disconnect");
	}
}

// REGULAR \\
fn response_from_file() {
	let mut file = File::open("src/responses/example.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    crate::send_msg(&contents.replace("\n", " ").replace("\r", ""))
}

fn ping() {
	crate::send_msg("Pong!")
}

fn hug(user: &str) {
	crate::send_msg(&format!("/me hugs {}!", user))
}
