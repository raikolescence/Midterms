
/*
pub fn encode(text: &String) -> (String, usize) {
    let mut text = String::from(text);
    text.push('$'); // add sentinel character to the end of the string
    let rotations: Vec<String> = (0..text.len())
        .map(|i| text.chars().skip(i).chain(text.chars().take(i)).collect())
        .collect();
    //println!("{:?}",rotations );
    let mut sorted_rotations = rotations.clone();
    sorted_rotations.sort_by(|a, b| {
        match (a.chars().next(), b.chars().next()) {
            (Some('$'), Some('$')) => a.cmp(b),
            (Some('$'), _) => std::cmp::Ordering::Less,
            (_, Some('$')) => std::cmp::Ordering::Greater,
            _ => a.cmp(b),
        }
    });
    let index = sorted_rotations.iter().position(|s| s.ends_with('$')).unwrap() as u64;
    let encoded = sorted_rotations.iter().map(|s| s.chars().last().unwrap()).collect();
    (encoded, index as usize)
}*/
    pub fn encode(text: &String) -> (String, usize) {
    let mut text = String::from(text);

    // Check if the last character is not '\0', then append '\0'
    if !text.ends_with('\0') {
        text.push('\0');
    }

    let rotations: Vec<String> = (0..text.len())
        .map(|i| text.chars().skip(i).chain(text.chars().take(i)).collect())
        .collect();

    let mut sorted_rotations = rotations.clone();
    sorted_rotations.sort_by(|a, b| {
        match (a.chars().next(), b.chars().next()) {
            (Some('\0'), Some('\0')) => a.cmp(b),
            (Some('\0'), _) => std::cmp::Ordering::Less,
            (_, Some('\0')) => std::cmp::Ordering::Greater,
            _ => a.cmp(b),
        }
    });

    let index = sorted_rotations.iter().position(|s| s.ends_with('\0')).unwrap() as usize;
    let encoded = sorted_rotations.iter().map(|s| s.chars().last().unwrap()).collect();
    //println!("{:?}",encoded );

    (encoded, index)
}
/*
pub fn decode(encoded: &str, index: &usize) -> String {
    let mut table = vec![String::new(); encoded.len()];
    for _ in 0..encoded.len() {
        for (i, row) in table.iter_mut().enumerate() {
            let ch = encoded.chars().nth(i).unwrap();
            row.insert(0, ch);
        }
        table.sort_by(|a, b| a.chars().map(|ch| if ch == '$' { '\0' } else { ch })
                            .cmp(b.chars().map(|ch| if ch == '$' { '\0' } else { ch })));
    }
    let original_string = &table[*index];
    original_string.clone()
}*/
pub fn decode(encoded: &str, index: &usize) -> String {
    // Step 1: Create array of 2-ary tuples
    let mut tuples: Vec<(char, usize)> = encoded.chars().enumerate().map(|(i, c)| (c, i)).collect();

    // Step 2: Sort T using radix sort
    tuples.sort_by_key(|k| (k.0, k.1));

    // Step 3: Create a new array L from the second values of each element in the sorted tuple
    let l: Vec<usize> = tuples.iter().map(|&(_, idx)| idx).collect();

    // Step 4: Initialize a value Lidx
    let mut lidx = *index;
    let mut m = String::new();

    // Step 5: For a loop iterating V times
    for _ in 0..encoded.len() {
        // Get 𝐿[𝐿𝑖𝑑𝑥]
        let left_shift = l[lidx];

        // Push the character corresponding to the index 𝑉[𝐿[𝐿𝑖𝑑𝑥]]
        m.push(encoded.chars().nth(left_shift).unwrap());

        // Set the new Lidx to be equal to 𝐿[𝐿𝑖𝑑𝑥]
        lidx = left_shift;
    }

    // Step 6: M will now contain the original string
    m
}
    




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_sample() {
        let text = String::from("bananaaa!");
        let enc_text_ref = String::from("!aaannb\0aa");
        let (enc_text, enc_idx) = super::encode(&text);

        assert_eq!(enc_text, enc_text_ref);
        assert_eq!(enc_idx, 7);
    }

    #[test]
    fn decode_sample() {
        let text = String::from("!aaannb\0aa");
        let dec_text_ref = String::from("bananaaa!\0");
        let dec_text = super::decode(&text, &7);

        assert_eq!(dec_text, dec_text_ref);
    }
    
    #[test]
    fn encode2() {
        let text = String::from(" !%$\n?%%!   \n\r\0");
        let enc_text_ref = String::from("\r $\n  !\0% %%!?\n");
        let (enc_text, enc_idx) = super::encode(&text);

        assert_eq!(enc_text, enc_text_ref);
        assert_eq!(enc_idx, 7);
    }

    #[test]
    fn decode2() {
        let text = String::from("\r $\n  !\0% %%!?\n");
        let dec_text_ref = String::from(" !%$\n?%%!   \n\r\0");
        let dec_text = super::decode(&text, &7);

        assert_eq!(dec_text, dec_text_ref);
    }

    
    #[test]
    fn encode3() {
        let text = String::from(" H3llo\n  W0r1d?!\r\0");
        let enc_text_ref = String::from("\ro!\n\0 ?WrHd  13ll0");
        let (enc_text, enc_idx) = super::encode(&text);

        assert_eq!(enc_text, enc_text_ref);
        assert_eq!(enc_idx, 4);
    }

    #[test]
    fn decode3() {
        let text = String::from("\ro!\n\0 ?WrHd  13ll0");
        let dec_text_ref = String::from(" H3llo\n  W0r1d?!\r\0");
        let dec_text = super::decode(&text, &4);

        assert_eq!(dec_text, dec_text_ref);
    }
}