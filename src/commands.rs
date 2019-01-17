pub fn handle_command(user: &str, msg: &str) -> (bool, String) {
	let data: Vec<&str> = msg[1..].split_whitespace().collect();

	let response = match data[0] {
		"ping" 	=> ping(),
		"hug" 	=> hug(user),
		_ 		=> "".to_string()
	};

	// Add checks to see if it's an admin command (disconnect, etc)
	if response == "" {
		return (false, msg.to_string())
	} else {
		return (true, response)
	};

}

fn ping() -> String {
	"Pong!".to_string()
}

fn hug(user: &str) -> String {
	format!("/me hugs {}!", user)
}
