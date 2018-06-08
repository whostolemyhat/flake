// extern crate snowflake;

// use snowflake::{ create_hash };
// use snowflake::draw::draw;


// fn main() {
//     let hash = match std::env::args().nth(1) {
//         Some(text) => text,
//         None => create_hash(64)
//     };

//     match draw(&hash) {
//         Ok(_) => println!("{:?}", hash),
//         Err(e) => println!("Error creating image: {:?}", e)
//     }
// }
