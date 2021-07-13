mod words;
use text_io::read;
use std::convert::TryInto;

fn main() {
    let input: u64 = read!();
    let mut encoded_number = words::encode(input);
    
    while encoded_number.chars().count() != 4 >{
        let length = encoded_number.chars().count();
        print!("{} is {}, ", encoded_number, length);
        encoded_number = words::encode(length.try_into().unwrap());
    }

    println!("{} is four, four is magic", encoded_number)

}
