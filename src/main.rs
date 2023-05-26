use std::io;
extern crate base85;
use base85::{decode,encode};
pub mod dpql;
pub mod compressor;

fn main() {

    //let my_string = "Maman died today. Or yesterday maybe, I don't know. I got a telegram from the home: Mother deceased . Funeral tomorrow. Faithfully yours. That doesn't mean anything. Maybe it was yesterday.".to_string();
    //println!("input string: {}", my_string);
        let mut my_string = String::new();
    io::stdin()
        .read_line(&mut my_string)
        .expect("Failed to read input");

    //let my_string = "Maman died today. Or yesterday maybe, I don't know. I got a telegram from the home: Mother deceased . Funeral tomorrow. Faithfully yours. That doesn't mean anything. Maybe it was yesterday.".to_string();
    println!("input string: {}", my_string);

    //println!("is input vector to write meta equal to output of readmeta? {}", my_vector==y);

    let z = dpql::zip::write(&my_string);
    println!("ZIP WRITE OUTPUT: {}",z);

    let zz = dpql::zip::read(&z);
    //println!("results of read meta: mlen: {}, moffset: {}, bwtidx: {}, huffmancodebook: {:?}", x.0, x.1, x.2, x.3);
    println!("output of dpql::zip::read is : {}", zz);
}
