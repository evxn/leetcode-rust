pub struct Codec {}

impl Codec {
    pub fn new() -> Self {
        Self {}
    }

    /// First encodes `msgs` vec size as 2 hex chars (representing 1 byte) uppercase for efficient preallocation on decoding.
    /// It's possible cause `msgs.len() <= 200`.
    ///
    /// Then encodes each `msg` sequentially:
    /// - `msg.len()` as 2 hex chars (representing 1 byte) uppercase. It's possible cause each `msg.len() <= 200`,
    /// - followed by msg content.
    ///
    /// # Examples
    ///
    /// ```
    /// use encode_and_decode_strings::Codec;
    /// let codec = Codec::new();
    /// let s = codec.encode(vec!["hello".to_string(), "aristocratically".to_string()]);
    /// assert_eq!(s, "0205hello10aristocratically");
    /// ```
    pub fn encode(&self, msgs: Vec<String>) -> String {
        debug_assert!(msgs.len() <= u8::MAX as usize);
        let mut res = Vec::with_capacity(1 + 2 * msgs.len());
        let msgs_len = Codec::encode_u8_to_hex(msgs.len() as u8);

        res.push(msgs_len);

        let payload = msgs.into_iter().flat_map(|msg| {
            let msg_len = Codec::encode_u8_to_hex(msg.len() as u8);

            [msg_len, msg]
        });

        res.extend(payload);

        res.concat()
    }

    /// Decodes a string encoded with `Codec::encode`
    ///
    /// # Examples
    ///
    /// ```
    /// use encode_and_decode_strings::Codec;
    /// let codec = Codec::new();
    /// let msgs = codec.decode("0205hello10aristocratically".to_string());
    /// assert_eq!(&msgs, &["hello", "aristocratically"]);
    /// ```
    pub fn decode(&self, mut serialized: String) -> Vec<String> {
        let msgs_len = {
            let msgs_len_token = serialized.drain(..2).collect();
            Codec::try_decode_hex_to_u8(msgs_len_token).expect("valid hex str")
        };

        let mut res = Vec::with_capacity(msgs_len as usize);

        while !serialized.is_empty() {
            let msg_len = {
                let msg_len_token = serialized.drain(..2).collect();
                Codec::try_decode_hex_to_u8(msg_len_token).expect("valid hex str")
            };

            let msg = serialized.drain(..msg_len as usize).collect();
            res.push(msg);
        }

        res
    }

    /// Returns padded with leading zeros hex uppercase of fixed len 2
    fn encode_u8_to_hex(value: u8) -> String {
        format!("{:0width$X}", value, width = 2)
    }

    /// Consumes the input
    fn try_decode_hex_to_u8(hex_string: String) -> Result<u8, std::num::ParseIntError> {
        debug_assert_eq!(hex_string.len(), 2);
        u8::from_str_radix(&hex_string, 16)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let codec = Codec::new();
        let strs = vec![String::from("Hello"), String::from("World")];
        let encoded_string: String = codec.encode(strs.clone());
        let strs2: Vec<String> = codec.decode(encoded_string);
        assert_eq!(strs, strs2);
    }

    #[test]
    fn example2() {
        let codec = Codec::new();
        let strs = vec![String::from("")];
        let encoded_string: String = codec.encode(strs.clone());
        let strs2: Vec<String> = codec.decode(encoded_string);
        assert_eq!(strs, strs2);
    }
}
