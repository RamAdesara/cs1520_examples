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


fn main() {
    let mut x = 5;
    x = 6;

    println!("This is integer x: {}", x);

    let y: u8 = 123;

    let tup1: (i32, f64, u32) = (12, 6.9, 28);
    let tup2 = (234, 2325, 21);

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
}