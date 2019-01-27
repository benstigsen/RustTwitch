pub fn handle_command(user: &str, msg: &str) {
	let data: Vec<&str> = msg[1..].split_whitespace().collect();

	match data[0] {
		// ADMIN / MODERATOR
		"dc"	=> disconnect(user),

		// REGULAR
		"ping" 	=> ping(),
		"hug" 	=> hug(user),

		_ 		=> ()
	};
}

// ADMIN \\
fn disconnect(user: &str) {
	let allowed = match user {
		crate::CHAN => true,
		_			=> false
	};

	if allowed {
		crate::send_msg("Disconnecting");
		crate::send_msg("/disconnect");
	}
}

// REGULAR \\
fn ping() {
	crate::send_msg("Pong!")
}

fn hug(user: &str) {
	crate::send_msg(&format!("/me hugs {}!", user))
}
