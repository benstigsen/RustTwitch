pub fn color(color: &str, text: &str) -> String {

	// Color codes and styles can be found here: 
	// http://www.lihaoyi.com/post/BuildyourownCommandLinewithANSIescapecodes.html
	
	let output_color = match color {
		"black" 				=> "30;1m",
		"bg_black"				=> "40;1m",

		"red"					=> "31;1m",
		"bg_red"				=> "41;1m",

		"green"					=> "32;1m",
		"bg_green"				=> "42;1m",

		"yellow"				=> "33m",
		"bg_yellow"				=> "43;1m",

		"blue"					=> "34;1m",
		"bg_blue"				=> "44;1m",

		"magenta" | "purple"	=> "35;1m",
		"bg_magenta"|"bg_purple"=> "45;1m",

		"cyan"					=> "36;1m",
		"bg_cyan"				=> "46;1m",

		"white"					=> "37;1m",
		"bg_white"				=> "47;1m",

		_						=> "37;1m" // White
	};

	format!("\u{001b}[{}{}\u{001b}[0m", output_color, text)
}