use super::*;
use std::collections::VecDeque;


pub fn write(text: &String) -> String{
    let diropql_program = super::write(text.to_string()); //convert the input text to a diropql program
    //println!("diropql_program: {}",diropql_program);

    let (bwt_encoded,bwt_idx) = crate::compressor::bwt::encode(&diropql_program); // output of bwt encode
    //println!("bwt_encoded: {:?} bwt_idx: {:?}",bwt_encoded,bwt_idx);
    let mtf_dict = crate::compressor::mtf::alphabet_sort(&bwt_encoded); // alphabet
    //println!("mtf_dict: {:?}",mtf_dict);  
    let mtf_encoded = crate::compressor::mtf::encode(&bwt_encoded, &mtf_dict); // output of mtf encode
    //println!("mtf_encoded: {:?}",mtf_encoded);  
    let rle_encoded = crate::compressor::rle::encode(&mtf_encoded); // output of rle encode
    //println!("rle_encoded: {:?}",rle_encoded); 
    let (huffman_encoded,huffman_codebook) = crate::compressor::huffman::encode(&rle_encoded); //output of huffman encode
    //println!("huffman_encoded: {:?}",huffman_encoded); 

    //extracting of metadata 

    let mlen_raw:u64 = huffman_encoded.len() as u64;
    let moffset:u8 = 8 - (mlen_raw % 8) as u8;
    let mlen_fin:u64 = mlen_raw + (moffset as u64);
    let bwt_idx_u64 = bwt_idx as u64;
    
    //putting meta data in struct
    let metadata = DpqlzMeta(mlen_fin, moffset, bwt_idx_u64, huffman_codebook);
    //write the metadata
    let diropqlz_fin = write_meta(&metadata, &huffman_encoded);
    //println!("{:?}",diropqlz_fin);
    diropqlz_fin  // from input text, convert to diropqlz file!
}
pub fn read(prog: &String) -> String{
    let (metadata, compressed_prog) = read_meta(&prog);

    let huffman_decoded = crate::compressor::huffman::decode(&compressed_prog, &metadata.3); //output of huffman decode
    //println!("huffman_decoded: {:?}",huffman_decoded);
    
    let rle_decoded = crate::compressor::rle::decode(&huffman_decoded); //output of rle decode
    //println!("rle_decoded: {:?}",rle_decoded);

    let mtf_alphabet = "dior\0".to_string(); //only makes use of dior
    let mtf_decoded = crate::compressor::mtf::decode(&rle_decoded, &mtf_alphabet); //output of mtf decode
    //println!("mtf_decoded: {:?}",mtf_decoded);
    
    let bwt_idx_usize = metadata.2 as usize;
    //println!("test");
    let bwt_decoded = crate::compressor::bwt::decode(&mtf_decoded, &bwt_idx_usize); // output of bwt decode, medyo matagal to
    // the bwt decoded string above is the diropql program! time to read
    //println!("bwt_decoded: {:?}",bwt_decoded);
    let final_decoded = super::read(bwt_decoded.to_string()); // this is the final deobfuscated message 

    final_decoded
}
pub fn write_meta(meta: &DpqlzMeta, prog: &Vec <u8>) -> String{
    let mut diropqlz = String::new();
    let mlen = meta.0; // this is in bytes
    let moffset = meta.1; //in bits
    let bwt_idx = meta.2; //not in bytes
    let huffman_tree = &meta.3;

    let mlen_as_64bits = format!("{:064b}", mlen*8);  //mlen is in bytes, convert it to # of bits - format into 64 bit binary number
    let mlen_64bit_string = format!("{}", mlen_as_64bits);
    diropqlz.push_str(&mlen_64bit_string);    //appending of mlen to diropqlz file

    let moffset_as_8bits = format!("{:08b}", moffset);  //moffset is in bits, convert into 8 bit binary number
    let moffset_8bit_string = format!("{}", moffset_as_8bits);
    diropqlz.push_str(&moffset_8bit_string); //appending of moffset to diropqlz file

    let bwt_idx_as_64bits = format!("{:064b}", bwt_idx);  //bwt idx expressed as 64 bit binary number
    let bwt_idx_64bit_string = format!("{}", bwt_idx_as_64bits);
    diropqlz.push_str(&bwt_idx_64bit_string); //appending of bwt_idx to the file

    let mut ctr = 0;
    for item in huffman_tree.iter(){
        let item_as_8bits =  format!("{:08b}", item);
        let item_as_8bit_string = format!("{}", item_as_8bits);
        diropqlz.push_str(&item_as_8bit_string);            
        ctr+=1;}  //appending of elements of huffman tree
    while ctr < 16{
        let item = 0;
        let item_as_8bits =  format!("{:08b}", item);
        let item_as_8bit_string = format!("{}", item_as_8bits);       
        diropqlz.push_str(&item_as_8bit_string);   // fill the rest w zeroes
        ctr+=1;
    }
    //meta data is appended ! total length is 264 bits ! 

    for element in prog.iter(){
        let string_element = element.to_string(); 
        diropqlz.push_str(&string_element); //push obfuscated message into diropqlz
    }

    let mut ctr_zeroes = 0;
    while ctr_zeroes < moffset{
        diropqlz.push('0');
        ctr_zeroes+=1;} // append any zeroes if needed since obfuscated message wont always be divisible by 8

// b85 encoding needs a vector input of bytes

    let mut diropqlz_bytes: Vec<u8> = Vec::new();
    let mut working_byte = String::new();
    for character in diropqlz.chars(){
        let char_string = character.to_string();
        working_byte.push_str(&char_string);
        if working_byte.len() == 8 {
            let decimal = u8::from_str_radix(&working_byte, 2).unwrap();
            diropqlz_bytes.push(decimal);
            working_byte.clear();}
    }

    let base85_diropqlz = base85::encode(&diropqlz_bytes); //encoded!

    let diropqlz_final = "DIROPQLZ".to_owned() + &base85_diropqlz; //prepend the magic string

    diropqlz_final //diropqlz file
    
}


pub fn read_meta(prog: &String) -> (DpqlzMeta, Vec <u8>){
    //assuming that prog is a diropqlz file, the first 7 chars in the string should be DIROPQLZ
    let  b85_encoded_prog = &prog[8..]; //remove magic string to get base 85 encoded string
    
    let decoded_prog = base85::decode(&b85_encoded_prog); // decoded program, the output will be a vector of bytes
    let decoded_vec = decoded_prog.unwrap(); 

    let mut b85_decoded = String::new();
    
    for item in decoded_vec.iter(){
        let item_as_8bits =  format!("{:08b}", item);
        let item_as_8bit_string = format!("{}", item_as_8bits);
        b85_decoded.push_str(&item_as_8bit_string);
    } // the vector of bytes should be converted into its 8 bit representations
    
    // slice the stuff
    //[inc..] [..not]
    let mlen_string = &b85_decoded[..64];
    let moffset_string = &b85_decoded[64..72]; 
    let bwt_idx_string = &b85_decoded[72..136]; 
    let huffman_tree_string = &b85_decoded[136..264];
    let obfuscated_string = &b85_decoded[264..];

    //convert binary strings to correct data types
    let mlen = u64::from_str_radix(mlen_string, 2).unwrap();
    let mlen_final = mlen/8;
    let moffset = u8::from_str_radix(moffset_string, 2).unwrap();
    let bwt_idx = u64::from_str_radix(bwt_idx_string, 2).unwrap();
    let mut huffman_tree: Vec<u8> = Vec::new();
    let mut working_string = String::new();
    for bin_digit in huffman_tree_string.chars(){
        if working_string.len() != 8 {
            let bin_digit_str = bin_digit.to_string();   //form bytes from the characters
            working_string.push_str(&bin_digit_str);}
        if working_string.len() == 8{
            let huffman_entry = u8::from_str_radix(&working_string, 2).unwrap();
            huffman_tree.push(huffman_entry);  // push the bytes
            working_string.clear();}
    }
    
    let huffman_tree_final = &huffman_tree[..10]; //only 10 elements accdg to don

    let mut output_vec: Vec<u8> = Vec::new();

    for bin_digit in obfuscated_string.chars(){
        output_vec.push(bin_digit.to_digit(10).unwrap() as u8);}  // put each element in the obfuscated string into a vector
    let final_index:usize = (mlen_final - (moffset as u64)) as usize; //remove offset zeroes

    let output_vec_final = &output_vec[..final_index];  //final output vector!

    return (DpqlzMeta(mlen_final, moffset, bwt_idx, huffman_tree_final.to_vec()), output_vec_final.to_vec());
} 

#[derive(PartialEq)]
#[derive(Debug)]
pub struct DpqlzMeta (pub u64, pub u8, pub u64, pub Vec <u8>);





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write1() {
        let text = String::from(" ");
        let write_text_ref = String::from("DIROPQLZ000000001h0RR91000000RaF20|Np800000000000Jj7");
        let write_text = write(&text);

        assert_eq!(write_text, write_text_ref);
    }

    #[test]
    fn read1() {
        let text = String::from("DIROPQLZ000000001h0RR91000000RaF20|Np800000000000Jj7");
        let read_text_ref = String::from(" ");
        let read_text = read(&text);

        assert_eq!(read_text, read_text_ref);
    }

    #[test]
    fn write_meta1() {
        let data = super::DpqlzMeta(16, 1, 1, vec![1, 0, 0, 3, 3, 2, 0, 0, 0, 0]);
        let prog = vec![1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0];
        let write_text_ref = String::from("DIROPQLZ000000001h0RR91000000RaF20|Np800000000000Jj7");
        let write_text = write_meta(&data, &prog);
        assert_eq!(write_text, write_text_ref);
    }

    #[test]
    fn read_meta1() {
        let text = String::from("DIROPQLZ000000001h0RR91000000RaF20|Np800000000000Jj7");
        let data_ref = super::DpqlzMeta(16, 1, 1, vec![1, 0, 0, 3, 3, 2, 0, 0, 0, 0]);
        let prog_ref = vec![1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0];
        let (data, prog) = read_meta(&text);

        assert_eq!(data, data_ref);
        assert_eq!(prog, prog_ref);
    }

    #[test]
    fn write2() {
        let text = String::from("Hello world!");
        let write_text_ref = String::from("DIROPQLZ00000000Dl0RR9100001Qvv}10|W#B00000000000Q;uZwY0UigK2CM?Evi?DofM");
        let write_text = write(&text);

        assert_eq!(write_text, write_text_ref);
    }

    #[test]
    fn write_meta2() {
        let data = super::DpqlzMeta(144, 1, 339, vec![2, 1, 0, 3, 4, 4, 0, 0, 0, 0]);
        let prog = vec![1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0];
        let write_text_ref = String::from("DIROPQLZ00000000Dl0RR9100001Qvv}10|W#B00000000000Q;uZwY0UigK2CM?Evi?DofM");
        let write_text = write_meta(&data, &prog);
        assert_eq!(write_text, write_text_ref);
    }

    #[test]
    fn read_meta2() {
        let text = String::from("DIROPQLZ00000000Dl0RR9100001Qvv}10|W#B00000000000Q;uZwY0UigK2CM?Evi?DofM");
        let data_ref = super::DpqlzMeta(144, 1, 339, vec![2, 1, 0, 3, 4, 4, 0, 0, 0, 0]);
        let prog_ref = vec![1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0];
        let (data, prog) = read_meta(&text);

        assert_eq!(data, data_ref);
        assert_eq!(prog, prog_ref);
    }

    #[test]
    fn read2() {
        let text = String::from("DIROPQLZ00000000Dl0RR9100001Qvv}10|W#B00000000000Q;uZwY0UigK2CM?Evi?DofM");
        let read_text_ref = String::from("Hello world!");
        let read_text = read(&text);

        assert_eq!(read_text, read_text_ref);
    }

    #[test]
    fn write3() {
        let text = String::from("0?F E!30m enTBpWGUo.uRldrh K io  Jcx YhV$tq z@");
        let write_text_ref = String::from("DIROPQLZ00000000lb0{{R300005`~d?10t5sA00000000000QV@R3fi?Am8)eGGJ>|C)tZ8vVNHr`p+KWGHfF7q!A9DfC}Or1hO7*xVzzaIrpzcaRw=b9O(kZg*fZN;");
        let write_text = write(&text);

        assert_eq!(write_text, write_text_ref);
    }

    #[test]
    fn read3() {
        let text = String::from("DIROPQLZ00000000lb0{{R300005`~d?10t5sA00000000000QV@R3fi?Am8)eGGJ>|C)tZ8vVNHr`p+KWGHfF7q!A9DfC}Or1hO7*xVzzaIrpzcaRw=b9O(kZg*fZN;");
        let read_text_ref = String::from("0?F E!30m enTBpWGUo.uRldrh K io  Jcx YhV$tq z@");
        let read_text = read(&text);

        assert_eq!(read_text, read_text_ref);
    }

}