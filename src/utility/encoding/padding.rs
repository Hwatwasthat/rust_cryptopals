pub fn pkcs7(buffer: &[u8], block_len: usize) -> Vec<u8> {
    let mut ret = buffer.to_vec();
    // Can't very well modulo by 0 can you
    if block_len == 0 {
        return ret;
    }

    let pad_length = (block_len - buffer.len()) % block_len;
    ret.extend(std::iter::repeat(pad_length as u8).take(pad_length));
    ret
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn pad_1_to_4_correct_length() {
        let mut buf: Vec<u8> = vec![0];
        pkcs7(&mut buf, 4);
        assert_eq!(buf.len(), 4);
    }

    #[test]
    pub fn pad_1_to_4_correct_value() {
        let mut buf: Vec<u8> = vec![0];
        pkcs7(&mut buf, 4);
        assert_eq!(buf, vec![0, 3, 3, 3]);
    }

    #[test]
    pub fn pad_0_correct_length() {
        let mut buf: Vec<u8> = vec![];
        pkcs7(&mut buf, 0);
        assert_eq!(buf.len(), 0);
    }

    #[test]
    pub fn pad_0_correct_value() {
        let mut buf: Vec<u8> = vec![];
        pkcs7(&mut buf, 0);
        assert_eq!(buf, vec![]);
    }

    #[test]
    pub fn pad_1_to_1_correct_length() {
        let mut buf: Vec<u8> = vec![0];
        pkcs7(&mut buf, 1);
        assert_eq!(buf.len(), 1);
    }

    #[test]
    pub fn pad_1_to_1_correct_value() {
        let mut buf: Vec<u8> = vec![0];
        pkcs7(&mut buf, 1);
        assert_eq!(buf, vec![0]);
    }
}
