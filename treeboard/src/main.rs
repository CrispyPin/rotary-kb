use std::io::Read;
use enigo::*;


fn main() {
	let port = "/dev/ttyACM1";
	println!("Connecting to arduino on port {}...", port);
	let connection = serial::open(port);
	if connection.is_err() {
		println!("Connection failed {}", connection.err().unwrap());
		return;
	}

	let mut connection = connection.unwrap();
	let mut board = Treeboard::new();
	board.print();
	loop {
		let mut recieved = [0; 1];
		if let Ok(_count) = connection.read(&mut recieved) {
			let data = String::from_utf8(recieved.into()).unwrap();
			match data.chars().next().unwrap() {
				'r' => board.right(),
				'l' => board.left(),
				'd' => board.press(),
				'u' => board.release(),
				_ => ()
			}
		}
	}
}

struct Treeboard {
	input: Enigo,
	pressed: bool,
	keys: Vec<KeyNode>,
	pos: usize,
}

struct KeyNode {
	name: String,
	key: Key,
	right: usize,
	left: usize,
}

impl KeyNode {
	fn new(name: &str, key: Key, right: usize, left: usize) -> Self {
		Self {
			name: name.to_owned(),
			key, right, left,
		}
	}
}


impl Treeboard {
	fn new() -> Self {
		let keys = vec![
			KeyNode::new("_", Key::Space, 1, 2),

			KeyNode::new("E", Key::Layout('e'), 3, 4),
			KeyNode::new("A", Key::Layout('a'), 5, 6),

			KeyNode::new("R", Key::Layout('r'), 7, 8),
			KeyNode::new("I", Key::Layout('i'), 9, 10),
			KeyNode::new("O", Key::Layout('o'), 11, 12),
			KeyNode::new("T", Key::Layout('t'), 13, 14),

			KeyNode::new("N", Key::Layout('n'), 15, 16),
			KeyNode::new("S", Key::Layout('s'), 17, 18),
			KeyNode::new("L", Key::Layout('l'), 19, 20),
			KeyNode::new("C", Key::Layout('c'), 21, 22),
			KeyNode::new("U", Key::Layout('u'), 23, 24),
			KeyNode::new("D", Key::Layout('d'), 25, 26),
			KeyNode::new("P", Key::Layout('p'), 27, 28),
			KeyNode::new("M", Key::Layout('m'), 29, 30),

			KeyNode::new("H", Key::Layout('h'), 0, 0),
			KeyNode::new("G", Key::Layout('g'), 0, 0),
			KeyNode::new("B", Key::Layout('b'), 0, 0),
			KeyNode::new("F", Key::Layout('f'), 0, 0),
			KeyNode::new("Y", Key::Layout('y'), 0, 0),
			KeyNode::new("W", Key::Layout('w'), 0, 0),
			KeyNode::new("K", Key::Layout('k'), 0, 0),
			KeyNode::new("V", Key::Layout('v'), 0, 0),
			KeyNode::new("X", Key::Layout('x'), 0, 0),
			KeyNode::new("Z", Key::Layout('z'), 0, 0),
			KeyNode::new("J", Key::Layout('j'), 0, 0),
			KeyNode::new("Q", Key::Layout('q'), 0, 0),
			KeyNode::new(",", Key::Layout(','), 0, 0),
			KeyNode::new(".", Key::Layout('.'), 0, 0),
			KeyNode::new("⏎", Key::Return, 0, 0),
			KeyNode::new("<", Key::Backspace, 0, 0),
		];

		Self {
			input: Enigo::new(),
			pressed: false,
			keys,
			pos: 0,
		}
	}

	fn print(&self) {
		println!("\n\n\n\n");
		let mut depth = 4;
		let mut row = Vec::new();
		row.push(0);
		loop {
			if depth != 0 {
				print!(" ");
			}
			if depth > 1 {
				for _ in 0..(2usize.pow(depth-1)-1) {
					print!("  ");
				}
			}
			let mut next_row = Vec::new();
			for k in row {
				next_row.push(self.keys[k].left);
				next_row.push(self.keys[k].right);
				let name = &self.keys[k].name;
				if k == self.pos {
					print!("{})", name);
				}
				else {
					print!("{} ", name);
				}
				for _ in 0..(2usize.pow(depth)-1) {
					print!("  ");
				}
			}
			println!("");
			row = next_row;
			if depth == 0 {
				break;
			}
			depth -= 1;
		}

	}

	fn right(&mut self) {
		self.move_to(self.keys[self.pos].right);
	}

	fn left(&mut self) {
		self.move_to(self.keys[self.pos].left);
	}
	
	fn move_to(&mut self, new_pos: usize) {
		if new_pos < self.keys.len() && !self.pressed {
			self.pos = new_pos;
			self.print()
		}
	}

	fn press(&mut self) {
		self.input.key_down(self.keys[self.pos].key);
	}

	fn release(&mut self) {
		self.input.key_up(self.keys[self.pos].key);
		self.move_to(0);
	}
}
