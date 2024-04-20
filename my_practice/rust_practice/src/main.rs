// fn add_one_v1 (x: u32) -> u32 {
//     x + 1
// }

// let add_one_v2 = |x: u32| -> u32 {
//     x + 1
// };

// let add_one_v3 = |x| {
//     x + 1
// };

// let add_one_v4 = |x|
//     x + 1;

// fn main() {
//     println!("Hello");
// }

struct Point(i32, i32);

struct Rectangle {
    top_left: Point,
    width: i32,
    height: i32,
}

trait Shape {
    fn area(&self) -> i32;
    fn contains(&self, p: Point) -> bool;
}

impl Shape for Rectangle {
	fn area(&self) -> i32 {
		self.width * self.height
	}

    fn contains(&self, p: Point) -> bool {
		if p.0 > self.top_left.0
			&& p.0 < (self.top_left.0 + self.width)
			&& p.1 > self.top_left.1
			&& p.1 < (self.top_left.1 + self.height)
		{
			return true;
		}

		false
	}
}

fn print_area<T: Shape> (some_shape: T) {
    println!("Shape's area: {}", some_shape.area());
}

fn main() {
    let mut x = 5;
    x = 6;

    println!("This is integer x: {}", x);

    let y: u8 = 123;

    let tup1: (i32, f64, u32) = (12, 6.9, 28);
    let tup2 = (234, 2325, 213);

    let (x, y, z) = tup2;
    println!("x: {}, y: {}, z: {}", x, y, z);

    let i = tup2.2;
    println!("i is {}", i);

    println!();

    let arr1: [i64; 5] = [1,2,3,4,5];

    for i in arr1 {
        println!("{}", i);
    }
    println!();

    // colon only used when you are specifying the data type i think
    let arr2 = [3; 5];

    for i in arr2 {
        println!("{}", i);
    }
    println!();

    println!("arr2[0]: {}, arr2[4]: {}", arr2[0], arr2[4]);

    let x = {
        let i = 5;
        i + 10
    };
    println!();

    println!("x = {}\n", x);

    let mut c = 5;
	while (c > 0) {
		println!("In while, c: {}", c);
		c -= 1;
	}
    println!();

    for i in 5..=10 {
        println!("i is {}", i)
    }

    println!();

    let mut c: u8 = 0;

    let m = loop {
        if c > 10 {
            break c
        }
        c += 1;
    };

    println!("m: {}\n", m);

    let c = 15;
	let d = 20;
	println!("{} + {} is: {}\n", c, d, adder(c, d));




    impl Rectangle {
        fn new(top_left: Point, width: i32, height: i32) -> Rectangle {
            Rectangle {
                top_left,
                width,
                height,
            }
        }

        fn identify(&self) {
            println!("i am a rectangel");
        }
    }

    let r = Rectangle::new(Point(3,3),5,10);
    r.identify();
}

fn adder(a: i32, b: i32) -> i32 {
    a + b
}