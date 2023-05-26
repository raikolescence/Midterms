
/*
pub fn encode(text: &str) -> (Vec<u8>, String) {
    let mut alphabet: Vec<char> = text.chars().collect();
    alphabet.sort_by(|a, b| a.cmp(b));
    alphabet.sort_by(|a, b| b.is_ascii_punctuation().cmp(&a.is_ascii_punctuation()));
    let sentinel_pos = alphabet.iter().position(|&x| x == '$').unwrap();
    alphabet.remove(sentinel_pos);  // places chosen sentinel to starting position
    alphabet.insert(0, '$');
    alphabet.dedup();
    let dictionary = alphabet.clone().into_iter().collect::<String>();
    let mut output = Vec::new();
        for c in text.chars() {
            let index = alphabet.iter().position(|&x| x == c).unwrap() as u8;
            output.push(index);
            alphabet.remove(index as usize);
            alphabet.insert(0, c);
        }
    (output, dictionary)
}*/
pub fn alphabet_sort(text: &str) -> String {
    let mut alphabet: Vec<char> = text.chars().collect();
    alphabet.sort_by(|a, b| a.cmp(b));
    alphabet.sort_by(|a, b| b.is_ascii_punctuation().cmp(&a.is_ascii_punctuation()));
    let sentinel_pos = alphabet.iter().position(|&x| x == '\0').unwrap();
    alphabet.remove(sentinel_pos);  // places chosen sentinel to starting position
    alphabet.insert(0, '\0');
    alphabet.dedup();
    let dictionary = alphabet.clone().into_iter().collect::<String>();
    dictionary
}



pub fn encode(text: &str, alpha:&str) -> Vec<u8> {
        let mut alphabet: Vec<char> = alpha.chars().collect();            
        let mut output = Vec::new();
        for c in text.chars() {
            let index = alphabet.iter().position(|&x| x == c).unwrap() as u8;
            output.push(index);
            alphabet.remove(index as usize);
            alphabet.insert(0, c);
        }
    output
}
pub fn decode(encoded: &[u8], alphabet: &str) -> String {
    let mut mtf_alphabet: Vec<char> = alphabet.chars().collect();
    let mut message = String::new();
    for &index in encoded {
        let c = mtf_alphabet.remove(index as usize);
        message.push(c);
        if let Some(pos) = mtf_alphabet.iter().position(|&x| x == c) {
            mtf_alphabet.remove(pos);
        }
        mtf_alphabet.insert(0, c);
    }
    message
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_sample() {
        let alpha = String::from("$!abn");
        let text = String::from("bananaaa!$");
        let enc_vec_ref = vec![3, 3, 4, 1, 1, 1, 0, 0, 4, 4];
        let enc_vec = super::encode(&text, &alpha);

        assert_eq!(enc_vec, enc_vec_ref);
    }

    #[test]
    fn decode_sample() {
        let alpha = String::from("$!abn");
        let data = vec![3, 3, 4, 1, 1, 1, 0, 0, 4, 4];
        let dec_text_ref = String::from("bananaaa!$");
        let dec_text = super::decode(&data, &alpha);

        assert_eq!(dec_text, dec_text_ref);
    }

    // output from bwt
    #[test]
    fn encode_sample2() {
        let alpha = String::from("$!abn");
        let text = String::from("!aaannb$aa");
        let enc_vec_ref = vec![1,2,0,0,4,0,4,4,3,0];
        let enc_vec = super::encode(&text, &alpha);

        assert_eq!(enc_vec, enc_vec_ref);
    }

    #[test]
    fn decode_sample2() {
        let alpha = String::from("$!abn");
        let data = vec![1,2,0,0,4,0,4,4,3,0];
        let dec_text_ref = String::from("!aaannb$aa");
        let dec_text = super::decode(&data, &alpha);

        assert_eq!(dec_text, dec_text_ref);
    }

    #[test]
    fn encode1() {
        let alpha = String::from("\0\n\r !$%?");
        let text = String::from(" !%$\n?%%!   \n\r\0");
        let enc_vec_ref = vec![3, 4, 6, 6, 5, 7, 3, 0, 4, 5, 0, 0, 4, 7, 7];
        let enc_vec = super::encode(&text, &alpha);

        assert_eq!(enc_vec, enc_vec_ref);
    }

    #[test]
    fn decode1() {
        let alpha = String::from("\0\n\r !$%?");
        let data = vec![3, 4, 6, 6, 5, 7, 3, 0, 4, 5, 0, 0, 4, 7, 7];
        let dec_text_ref = String::from(" !%$\n?%%!   \n\r\0");
        let dec_text = super::decode(&data, &alpha);

        assert_eq!(dec_text, dec_text_ref);
    }

    #[test]
    fn encode2() {
        let alpha = String::from("\0\r\n !013?HWdlor");
        let text = String::from(" H3llo\n  W0r1d?!\r\0");
        let enc_vec_ref = vec![3, 9, 8, 12, 0, 13, 7, 5, 0, 12, 10, 14, 12, 14, 14, 14, 14, 14];
        let enc_vec = super::encode(&text, &alpha);

        assert_eq!(enc_vec, enc_vec_ref);
    }

    #[test]
    fn decode2() {
        let alpha = String::from("\0\r\n !013?HWdlor");
        let data = vec![3, 9, 8, 12, 0, 13, 7, 5, 0, 12, 10, 14, 12, 14, 14, 14, 14, 14];
        let dec_text_ref = String::from(" H3llo\n  W0r1d?!\r\0");
        let dec_text = super::decode(&data, &alpha);

        assert_eq!(dec_text, dec_text_ref);
    }

}