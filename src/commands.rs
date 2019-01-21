pub fn handle_command(user: &str, msg: &str) -> (bool, String) {
	let data: Vec<&str> = msg[1..].split_whitespace().collect();

	let response = match data[0] {
		"ping" 	=> ping(),
		"hug" 	=> hug(user),

		// Admin commands
		"dc"	=> disconnect(user),
		_ 		=> "".to_string()
	};

	if response == "" {
		return (false, msg.to_string())
	} else {
		return (true, response)
	};

}

// ADMIN \\
fn disconnect(user: &str) -> String {
	
	let allowed = match user {
		"some_user" => true,
		_			=> false
	};

	let response = match allowed {
		true 	=> "dc",
		false 	=> "",
	};

	response.to_string()
}


// REGULAR \\
fn ping() -> String {
	"Pong!".to_string()
}

fn hug(user: &str) -> String {
	format!("/me hugs {}!", user)
}
