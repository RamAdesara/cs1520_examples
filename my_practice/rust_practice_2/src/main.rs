// Not in the standard library, note that I'm pulling this package from the 
// web via crates.io, it needed to be listed in Cargo.toml dependecies
use rand::Rng;

// Can be used like classic enum:
enum Color {
	Red,
	Green,
	Blue,
}

// Can be used to solve the lack of null
fn maybe_get_color() -> Option<Color> {
	let chance = rand::thread_rng().gen_range(0..100);

	println!("Generated: {}", chance);

	// match is an expression
	match chance {
		0..=24 => Some(Color::Red), // Option::Some, but included by default
		25..=49 => Some(Color::Green),
		50..=74 => Some(Color::Blue),
		_ => None,    // Option::None, but included by default
	}
}

fn main() {
	let l = "this is a string literal";	
	let m = &l[10..13];
	println!("l: {}", l);
	println!("m: {}", m);

	let h10 = Box::new(10);
	*h10;

	super::nested_a::hi();
	crate::nester::nested_b::hi();
}
