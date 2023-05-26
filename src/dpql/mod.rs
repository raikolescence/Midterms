pub mod zip;

use std::collections::VecDeque;
pub fn write(text: String) -> String{
    let mut program = String::new();

    for character in text.chars(){
        let character_ascii = character as u8;
        if character_ascii <128 {
            for _ctr_1 in 0..(character_ascii){
                program.push_str("i");
            }}
        else if character_ascii >=128{
            for _ctr_2 in 0..(255-character_ascii){
                program.push_str("d");
            }}
        program.push_str("o"); 
        program.push_str("r");
    }
    program
}

pub fn read(prog: String) -> String  {  
    let mut mp = 0;
    let mut ip:usize = 0;
    let mut arr:[u8; 10000] = [0; 10000];
    
    let mut output_queue: VecDeque<char> = VecDeque::new();

    let mut p_stack: Vec<usize> = Vec::new();
    let mut q_stack: Vec<usize> = Vec::new();

    let prog_chars: Box<[char]> = prog.chars().collect::<Vec<char>>().into_boxed_slice();
    let prog_len = prog_chars.len();
    let mut skipping = false;

    
    //println!("{:?}",prog);
    while ip < prog_len{

        //println!("ip: {:?}",ip);

        let instruction = prog_chars[ip];

        if instruction == 'r' && skipping == false {
            mp +=1;
            if mp == 10000{
                mp = 0;}    // wrap around
        }

        else if instruction == 'l' && skipping == false{
            if mp == 0{
                mp = 9999;} // wrap around
            else{
                mp -=1;}
            }    

        else if instruction == 'd' && skipping == false {
            if arr[mp]  == 0{
                arr[mp] = 255; }  //wrap around
            else{
                arr[mp]-=1;}
            }

        else if instruction == 'i' && skipping == false {
            if arr[mp]  == 255{
                arr[mp] = 0; }  //wrap around
            else{
                arr[mp]+=1;}
            }

        else if instruction == 'o' && skipping == false {
            let decimal_value = arr[mp] as u32;
            let ascii_char = std::char::from_u32(decimal_value).unwrap();
            output_queue.push_back(ascii_char);
        }
        // assumes that no nesting of pq's are present in the program
        else if instruction == 'p' {
            p_stack.push(ip);
            if arr[mp] != 0  {}
            else if arr[mp] == 0 {
                if skipping == false{
                    skipping = true; // skip through the next instructions!
                }}}

        else if instruction == 'q'{
            q_stack.push(ip);
            if skipping == true {
                if p_stack.len() == q_stack.len() {
                    //matching q has been found
                    p_stack.clear();
                    q_stack.clear();
                    skipping = false;}}

            else if arr[mp] == 0 {} //do nothing
            
            else if arr[mp] != 0 {
                // find index of next q! which is pstack pop!
                ip = p_stack.pop().unwrap();
        
                q_stack.pop();
                p_stack.push(ip);
            }
        }

        // assuming the diropql program is valid
        ip+=1;
    }

    let mut output = String::new();      
    for symb in &output_queue {        //put contents of output queue into a string
        output += &symb.to_string();   
    }

    output //return string
    //println!("{:?}",output_queue);

}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write1() {
        let text = String::from("Hello world!");
        let write_text_ref = String::from("iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiioriiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiioriiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiioriiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiioriiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiioriiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiioriiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiioriiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiioriiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiioriiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiioriiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiioriiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiior");
        let write_text = super::write(text);

        assert_eq!(write_text, write_text_ref);
    }

    #[test]
    fn read1() {
        let text = String::from("iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiioriiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiioriiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiioriiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiioriiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiioriiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiioriiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiioriiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiioriiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiioriiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiioriiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiioriiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiior");
        let dec_text_ref = String::from("Hello world!");
        let dec_text = read(text);

        assert_eq!(dec_text, dec_text_ref);
    }

    #[test]
    fn write2() {
        let text = String::from("");
        let write_text_ref = String::from("");
        let write_text = super::write(text);

        assert_eq!(write_text, write_text_ref);
    }

    #[test]
    fn read2() {
        let text = String::from("");
        let read_text_ref = String::from("");
        let read_text = super::read(text);

        assert_eq!(read_text, read_text_ref);
    }

    #[test]
    fn write3() {
        let text = String::from(" ");
        let write_text_ref = String::from("iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiior");
        let write_text = super::write(text);

        assert_eq!(write_text, write_text_ref);
    }

    #[test]
    fn read3() {
        let text = String::from("iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiior");
        let read_text_ref = String::from(" ");
        let read_text = super::read(text);

        assert_eq!(read_text, read_text_ref);
    }
    
    #[test]
    fn read4() {
        let text = String::from("ddpoiqriiiiiipdoqrddddppioqqirldddo");
        let read_text_ref = String::from("þÿ\u{5}\u{4}\u{3}\u{2}\u{1}\0ýþÿ\0þ");
        let read_text = super::read(text);

        assert_eq!(read_text, read_text_ref);
    }
    
    #[test]
    fn read5() {
        let text = String::from("iiipodddqrddddddpoiiqpdddddddoqldddddddddddddddddddddoiirrdddddddoriiiiiipdpdqqo");
        let read_text_ref = String::from("\u{3}úüþëù\0");
        let read_text = super::read(text);

        assert_eq!(read_text, read_text_ref);
    }

    #[test]
    fn read6() {
        let text = String::from("ppiipqio0oo~qqddorddddpioqriiiiiifiiiiii #$iiiiigjsiiiiii\0iiiiiyuiiiiii\niiiiiiii5iiiiiii//8iii4ii3i1iiiiii4iiii=iiiorpdddd3qdddoliiii`iiiiibhjiiiiiii]iiiii\rio");
        let read_text_ref = String::from("þýþÿ\0DýZ");
        let read_text = super::read(text);

        assert_eq!(read_text, read_text_ref);
    }

}