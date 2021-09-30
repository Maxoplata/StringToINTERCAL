/**
 * StringToINTERCAL.rs
 *
 * Converts a string to an INTERCAL script that will output said string.
 * usage: cargo build && ./StringToINTERCAL your string here
 *
 * @author Maxamilian Demian
 * @link https://www.maxodev.org
 * @link https://github.com/Maxoplata/StringToINTERCAL
 */
use std::{env, iter, process};

struct StringToINTERCAL {
	polite_count: i32,
}

impl StringToINTERCAL {
	fn new() -> Self {
		StringToINTERCAL {
			polite_count: 0,
		}
	}

	fn polite_line(&mut self, line: String) -> String {
		if self.polite_count == 3 {
			self.polite_count = 0;

			return "PLEASE ".to_owned() + &line + "\n";
		}

		self.polite_count += 1;

		return "DO ".to_owned() + &line + "\n";
	}

	fn leading_zeros(&self, dec: String) -> String {
		let chars: Vec<char> = dec.chars().collect();
		let count = chars.len();
		let mut ret_dec = dec.to_string();

		if count < 8 {
			ret_dec = iter::repeat("0").take(8 - count).collect::<String>() + &ret_dec;
		}

		return ret_dec;
	}

	fn convert_to_intercal(&mut self, string: String) -> String {
		// reset polite_count
		self.polite_count = 0;

		let chars: Vec<char> = string.chars().collect();

		let mut ret = self.polite_line(",1 <- #".to_string() + &chars.len().to_string());

		let mut last_char_loc = 256;

		for i in 0..chars.len() {
			let char_loc = isize::from_str_radix(&self.leading_zeros(format!("{:b}", (chars[i] as char) as i32)).chars().rev().collect::<String>(), 2).unwrap();
			let mut move_position = 0;

			if char_loc < last_char_loc {
				move_position = last_char_loc - char_loc;
			} else if char_loc > last_char_loc {
				move_position = (256 - char_loc) + last_char_loc;
			}

			last_char_loc = last_char_loc - move_position;
			if last_char_loc < 1 {
				last_char_loc = 256 + last_char_loc;
			}

			ret = ret + &self.polite_line(",1 SUB #".to_owned() + &(i + 1).to_string() + " <- #" + &move_position.to_string());
		}

		ret = ret + &self.polite_line("READ OUT ,1".to_string());
		ret = ret + &self.polite_line("GIVE UP".to_string());

		return ret;
	}
}

fn main() {
	let mut args: Vec<String> = env::args().collect();

	// make sure we have arguments passed to the script
	if args.len() < 2 {
		process::exit(1);
	}

	// remove file from args
	args.remove(0);

	// get all of the chars for the input string
	let input_string: String = args.join(" ");

	let mut my_intercal = StringToINTERCAL::new();

	println!("{}", my_intercal.convert_to_intercal(input_string));
}
