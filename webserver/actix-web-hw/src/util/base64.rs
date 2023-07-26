use base64::{engine::general_purpose, DecodeError, Engine as _};

pub fn encode(s: &str) -> String {
    general_purpose::URL_SAFE.encode(s)
}

pub fn decode(s: &str) -> Result<Vec<u8>, DecodeError> {
    general_purpose::URL_SAFE.decode(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DECODED: &str = "yay-it-works";
    const ENCODED: &str = "eWF5LWl0LXdvcmtz";

    #[test]
    fn test_encode() {
        assert_eq!(encode(DECODED), ENCODED);
    }

    #[test]
    fn test_decode() {
        assert_eq!(
            String::from_utf8(decode(ENCODED).unwrap()).unwrap(),
            DECODED
        );
    }
}
