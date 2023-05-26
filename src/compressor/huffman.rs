use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Eq, PartialEq)]
struct HuffmanNode {
    freq: isize,
    value: Option<u8>,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

impl HuffmanNode {
    fn new(freq: isize, value: Option<u8>) -> Self {
        HuffmanNode {
            freq,
            value,
            left: None,
            right: None,
        }
    }

    fn with_children(freq: isize, left: Box<HuffmanNode>, right: Box<HuffmanNode>) -> Self {
        HuffmanNode {
            freq,
            value: None,
            left: Some(left),
            right: Some(right),
        }
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.freq.cmp(&self.freq).then_with(|| other.value.cmp(&self.value))
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn calculate_frequencies(input: &Vec<u8>) -> Vec<HuffmanNode> {
    let mut frequencies = vec![0; 10];
    for &item in input {
        frequencies[item as usize] += 1;
    }
    frequencies.into_iter().enumerate().filter_map(|(i, freq)| {
        if freq > 0 {
            Some(HuffmanNode::new(freq, Some(i as u8)))
        } else {
            None
        }
    }).collect()
}

fn build_huffman_tree(nodes: Vec<HuffmanNode>) -> HuffmanNode {
    let mut heap: BinaryHeap<_> = nodes.into_iter().collect();

    while heap.len() > 1 {
        let right = heap.pop().unwrap();
        let left = heap.pop().unwrap();
        let new_node = HuffmanNode::with_children(
            left.freq + right.freq,
            Box::new(left),
            Box::new(right),
        );
        heap.push(new_node);
    }
    
    heap.pop().unwrap()
}

fn traverse_tree(node: &HuffmanNode, code: Vec<u8>, huffman_codes: &mut Vec<Option<Vec<u8>>>) {
    if let Some(value) = node.value {
        if !code.is_empty() {
            huffman_codes[value as usize] = Some(code);
        }
    } else {
        if let Some(ref left) = node.left {
            let mut new_code = code.clone();
            new_code.push(1);
            traverse_tree(left, new_code, huffman_codes);
        }
        if let Some(ref right) = node.right {
            let mut new_code = code.clone();
            new_code.push(0);
            traverse_tree(right, new_code, huffman_codes);
        }
    }
}

fn huffman_codebook(input: &Vec<u8>) -> (Vec<u8>, Vec<Vec<u8>>) {
    let nodes = calculate_frequencies(input);
    let huffman_tree = build_huffman_tree(nodes);
    let mut huffman_codes = vec![None; 10];
    traverse_tree(&huffman_tree, Vec::new(), &mut huffman_codes);

    let mut values = Vec::new();
    let mut codes = Vec::new();
    for (value, code_option) in huffman_codes.into_iter().enumerate() {
        if let Some(code) = code_option {
            values.push(value as u8);
            codes.push(code);
        }
    }

    (values, codes)
}

fn huffman_encode(input: &[u8], values: &[u8], codes: &[Vec<u8>]) -> Vec<u8> {
    let code_map: std::collections::HashMap<_, _> = values.iter().zip(codes.iter()).collect();
    input.iter().map(|&value| code_map[&value].clone())
        .flatten()
        .collect()
}



// Function to get the canonical Huffman code.

fn get_canonical_huffman(values: &Vec<u8>, huffman_codes: &Vec<Vec<u8>>) -> (Vec<u8>, Vec<Vec<u8>>) {
    // Create a vector of tuples (value, code length, code).
    let mut codes: Vec<(u8, usize, Vec<u8>)> = values.iter()
                                                       .zip(huffman_codes.iter())
                                                       .map(|(&value, code)| (value, code.len(), code.clone()))
                                                       .collect();
    // Sort the vector by code length and then by value.
    codes.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
    //println!("{:?}",codes );

    // Initialize the canon code lengths and canon code book vectors with zeros.
    let mut canon_code_lengths = vec![0; 10];
    let mut canon_code_book = vec![vec![]; 10];
    
    // Iterate over the sorted codes vector.
    let mut cur_code = 0;
    let mut cur_length = 0;
    for i in 0..codes.len() {
        let &(value, code_len, _) = &codes[i];

        // Check if the code length has increased.
        if code_len > cur_length {
            cur_code <<= code_len - cur_length;
            cur_length = code_len;
        }

        // Generate the binary representation of the current code.
        let mut code = Vec::new();
        for j in 0..code_len {
            code.push((cur_code >> (code_len - j - 1)) & 1);
        }

        // Assign the current code to the value and store the code in the code book.
        canon_code_lengths[value as usize] = code_len as u8;
        canon_code_book[value as usize] = code;

        // Increment the current code.
        cur_code += 1 << (cur_length - code_len);
    }
    canon_code_book.retain(|x| !x.is_empty());
    (canon_code_lengths, canon_code_book)
}





pub fn encode(text: &Vec <u8>) -> (Vec <u8>, Vec <u8>){
    let (values, codes) = huffman_codebook(&text);
    let (canon_code_lengths, canon_code_book) = get_canonical_huffman(&values, &codes);
    let huffman_message = huffman_encode(&text,&values,&canon_code_book);
    (huffman_message, canon_code_lengths)
}



fn calculate_codebook(canonical_lengths: &Vec<u8>) -> (Vec<u8>, Vec<Vec<u8>>) {
    let alphabet: Vec<u8> = (0..10).collect();

    let mut mapping: Vec<(u8, u8)> = alphabet
        .iter()
        .zip(canonical_lengths.iter())
        .filter(|&(_, &length)| length > 0)
        .map(|(&symbol, &length)| (symbol, length))
        .collect();

    mapping.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

    let mut codebook: Vec<Vec<u8>> = vec![vec![]; mapping.len()];

    let mut c_canon = 0u8;
    let mut l_canon = mapping[0].1;

    for (i, (_, length)) in mapping.iter().enumerate() {
        if *length > l_canon {
            c_canon <<= length - l_canon;
            l_canon = *length;
        }

        let mut code = Vec::new();
        for j in 0..*length {
            code.push((c_canon >> j) & 1);
        }
        code.reverse();
        codebook[i] = code;
        
        c_canon += 1;
    }

    let alphabet: Vec<u8> = mapping.iter().map(|&(symbol, _)| symbol).collect();

    (alphabet, codebook)
}



fn retranslate(codebook: &Vec<Vec<u8>>, alphabet: &Vec<u8>, data: &Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let mut temp: Vec<u8> = Vec::new();

    for &bit in data {
        temp.push(bit);
        if let Some(index) = codebook.iter().position(|code| *code == temp) {
            result.push(alphabet[index]);
            temp.clear();
        }
    }

    result
}

pub fn decode (data: &Vec <u8>, canon_freqs: &Vec <u8>) -> Vec <u8>{
    let(alphabet, codebook) = calculate_codebook(&canon_freqs);
    let result = retranslate(&codebook,&alphabet,&data);
    result
}

fn binary_to_decimal(binary: &[u8]) -> u8 {
    binary.iter().enumerate().fold(0, |acc, (index, &value)| {
        acc + (value << (binary.len() - 1 - index))
    })
}

fn canonical_codebook_lengths_decimal(canon_freqs: &Vec<Vec<u8>>) -> Vec<u8> {
    canon_freqs.iter().map(|binary| binary_to_decimal(binary)).collect()
}

fn decimal_to_binary(mut n: u8) -> Vec<u8> {
    let mut result = Vec::new();

    while n > 0 {
        result.push(n % 2);
        n /= 2;
    }

    result.reverse();
    result
}

fn canonical_codebook_lengths_binary(canon_freqs: &Vec<u8>) -> Vec<Vec<u8>> {
    canon_freqs.iter().map(|&n| decimal_to_binary(n)).collect()
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_sample() {
        let data = vec![5, 5, 6, 3, 3, 3, 1, 6, 6];
        let enc_data_ref1 = vec![1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0];
        let enc_data_ref2 = vec![0, 3, 0, 2, 0, 3, 1, 0, 0, 0];
        let (enc_data1, enc_data2) = super::encode(&data);

        assert_eq!(enc_data1, enc_data_ref1);
        assert_eq!(enc_data2, enc_data_ref2);
        
    }

    #[test]
    fn decode_sample() {
        let data1 = vec![1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0];
        let data2 = vec![0, 3, 0, 2, 0, 3, 1, 0, 0, 0];
        let dec_data_ref = vec![5, 5, 6, 3, 3, 3, 1, 6, 6];
        let dec_data = super::decode(&data1, &data2);

        assert_eq!(dec_data, dec_data_ref);
    }

    #[test]
    fn encode1() {
        let data = vec![7, 7, 8, 0, 8, 7, 8, 8];
        let enc_data_ref1 = vec![1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0];
        let enc_data_ref2 = vec![2, 0, 0, 0, 0, 0, 0, 2, 1, 0];
        let (enc_data1, enc_data2) = super::encode(&data);

        assert_eq!(enc_data1, enc_data_ref1);
        assert_eq!(enc_data2, enc_data_ref2);
    }

    #[test]
    fn decode1() {
        let data1 = vec![1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0];
        let data2 = vec![2, 0, 0, 0, 0, 0, 0, 2, 1, ];
        let dec_data_ref = vec![7, 7, 8, 0, 8, 7, 8, 8];
        let dec_data = super::decode(&data1, &data2);

        assert_eq!(dec_data, dec_data_ref);
    }

    #[test]
    fn encode2() {
        let data = vec![5, 6, 8, 8, 7, 9, 5, 0, 6, 7, 1, 6, 9, 9];
        let enc_data_ref1 = vec![1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1];
        let enc_data_ref2 = vec![4, 4, 0, 0, 0, 3, 2, 3, 3, 2];
        let (enc_data1, enc_data2) = super::encode(&data);

        assert_eq!(enc_data1, enc_data_ref1);
        assert_eq!(enc_data2, enc_data_ref2);
    }

    #[test]
    fn decode2() {
        let data1 = vec![1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1];
        let data2 = vec![4, 4, 0, 0, 0, 3, 2, 3, 3, 2];
        let dec_data_ref = vec![5, 6, 8, 8, 7, 9, 5, 0, 6, 7, 1, 6, 9, 9];
        let dec_data = super::decode(&data1, &data2);

        assert_eq!(dec_data, dec_data_ref);
    }

    // out of range
    // #[test] 
    // fn encode2() {
    //     let data = vec![5, 11, 10, 14, 0, 15, 9, 7, 0, 14, 12, 16, 14, 16, 16, 16, 16, 16];
    //     let enc_data_ref1 = vec![1,0,1,1,0,1,0,1,1,1,1,1,1,0,0,0,1,0,1,0,1,0,1,1,0,1,0,1,1,1,0,1,0,0,0,0,1,1,1,0,1,0,0,0,1,0,1,0,1,0,1];
    //     let enc_data_ref2 = vec![3, 0, 0, 0, 0, 5, 0, 5, 0, 5, 4, 4, 4, 0, 2, 4, 2,];
    //     let (enc_data1, enc_data2) = encode(&data);

    //     assert_eq!(enc_data1, enc_data_ref1);
    //     assert_eq!(enc_data2, enc_data_ref2);
    // }

    // #[test]
    // fn decode2() {
    //     let data1 = vec![1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0];
    //     let data2 = vec![3, 0, 0, 0, 0, 5, 0, 5, 0, 5];
    //     let dec_data_ref = vec![7, 7, 8, 0, 8, 7, 8, 8];
    //     let dec_data = decode(&data1, &data2);

    //     assert_eq!(dec_data, dec_data_ref);
    // }


}