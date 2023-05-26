
fn binary_without_msb(mut n: u64) -> Vec<u8> {
    let mut result = Vec::new();
    if n == 0 {
        return result;
    }
    n += 1;
    while n > 1 {
        result.push((n % 2) as u8);
        n /= 2;
    }
    result
}
pub fn encode(text: &[u8]) -> Vec<u8> {
    let mut n_zero = 0u64;  // Change to u64 here            
    let mut output = Vec::new();
    for (i, &value) in text.iter().enumerate() {
        if i <= text.len() - 1 && value == 0 {
            n_zero += 1;
        } else {
            let binary_n_zero = binary_without_msb(n_zero);
            for bit in binary_n_zero.into_iter().rev() {
                output.push(bit);
            }
            if let Some(val) = text.get(i) {
                output.push(val + 2);
            }
            n_zero = 0;
        }
    }
    let binary_n_zero = binary_without_msb(n_zero);
    for bit in binary_n_zero.into_iter().rev() {
        output.push(bit);
    }
    output
}
fn binary_to_decimal(binary: &[u8]) -> u64 {
    binary.iter().enumerate().fold(0_u64, |acc, (index, &value)| {
        acc + ((value as u64) << (binary.len() - 1 - index) as u64)
    }) as u64
}

pub fn decode(data: &[u8]) -> Vec<u8> {
    let mut n_zero = Vec::new();
    let mut output = Vec::new();
    let l = data.to_vec();
    let l_len = l.len();

    let mut i = 0;
    while i < l_len +1 {
        if i <= l_len - 1 && (l[i] == 0 || l[i] == 1) {
            n_zero.push(l[i]);
            i += 1;
        } else {
            n_zero.reverse();
            n_zero.push(1);
            n_zero.reverse();
            let n_zero_decimal = binary_to_decimal(&n_zero) - 1;
            let zeros_to_push = if n_zero_decimal > 0 { n_zero_decimal} else { 0 };
            for _ in 0..zeros_to_push {
                output.push(0);
            }
            n_zero.clear();

            if i < l_len {
                output.push(l[i] - 2);
            }
            i += 1;
        }
    }

    if !n_zero.is_empty() {
        n_zero.push(1);
        n_zero.reverse();
        let n_zero_decimal = binary_to_decimal(&n_zero) - 1;
        for _ in 0..n_zero_decimal {
            output.push(0);
        }
    }

    output
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_sample() {
        let data = vec![3, 3, 4, 1, 1, 1, 0, 0, 4, 4];
        let enc_data_ref = vec![5, 5, 6, 3, 3, 3, 1, 6, 6];
        let enc_data = super::encode(&data);

        assert_eq!(enc_data, enc_data_ref);
    }

    #[test]
    fn decode_sample() {
        let data = vec![5, 5, 6, 3, 3, 3, 1, 6, 6];
        let dec_data_ref = vec![3, 3, 4, 1, 1, 1, 0, 0, 4, 4];
        let dec_data = super::decode(&data);

        assert_eq!(dec_data, dec_data_ref);
    }

    #[test]
    fn encode1() {
        let data = vec![5, 5, 6, 0, 6, 5, 6, 6];
        let enc_data_ref = vec![7, 7, 8, 0, 8, 7, 8, 8];
        let enc_data = super::encode(&data);

        assert_eq!(enc_data, enc_data_ref);
    }

    #[test]
    fn decode1() {
        let data = vec![7, 7, 8, 0, 8, 7, 8, 8];
        let dec_data_ref = vec![5, 5, 6, 0, 6, 5, 6, 6];
        let dec_data = super::decode(&data);

        assert_eq!(dec_data, dec_data_ref);
    }

    #[test]
    fn encode2() {
        let data = vec![3, 4, 6, 6, 5, 7, 3, 0, 4, 5, 0, 0, 4, 7, 7];
        let enc_data_ref = vec![5, 6, 8, 8, 7, 9, 5, 0, 6, 7, 1, 6, 9, 9];
        let enc_data = super::encode(&data);

        assert_eq!(enc_data, enc_data_ref);
    }

    #[test]
    fn decode2() {
        let data = vec![5, 6, 8, 8, 7, 9, 5, 0, 6, 7, 1, 6, 9, 9];
        let dec_data_ref = vec![3, 4, 6, 6, 5, 7, 3, 0, 4, 5, 0, 0, 4, 7, 7];
        let dec_data = super::decode(&data);

        assert_eq!(dec_data, dec_data_ref);
    }
    #[test]
    fn encode3() {
        let data = vec![3, 9, 8, 12, 0, 13, 7, 5, 0, 12, 10, 14, 12, 14, 14, 14, 14, 14];
        let enc_data_ref = vec![5, 11, 10, 14, 0, 15, 9, 7, 0, 14, 12, 16, 14, 16, 16, 16, 16, 16];
        let enc_data = super::encode(&data);

        assert_eq!(enc_data, enc_data_ref);
    }

    #[test]
    fn decode3() {
        let data = vec![5, 11, 10, 14, 0, 15, 9, 7, 0, 14, 12, 16, 14, 16, 16, 16, 16, 16];
        let dec_data_ref = vec![3, 9, 8, 12, 0, 13, 7, 5, 0, 12, 10, 14, 12, 14, 14, 14, 14, 14];
        let dec_data = super::decode(&data);

        assert_eq!(dec_data, dec_data_ref);
    }
}