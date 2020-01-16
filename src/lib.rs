mod strings;
mod hash;

pub use strings::{hamming_bytes, hamming_ascii};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hamming_str_test() {
        assert_eq!(hamming_ascii("", ""), 0);
        assert_eq!(hamming_ascii("test", "text"), 1);
        assert_eq!(hamming_ascii("behemoth", "chan"), 8);
        assert_eq!(hamming_ascii("ascii", "skc"), 4);
        assert_eq!(hamming_ascii("skc", "ascii"), 4);
        assert_eq!(hamming_ascii("lodge", "lodge"), 0);
        assert_eq!(hamming_ascii("lodge", ""), 5);
        assert_eq!(hamming_ascii("", "lodge"), 5);
        assert_eq!(hamming_ascii("temptation", "tamper"), 7);
    }

    #[test]
    fn hamming_bytes_test() {
        assert_eq!(hamming_bytes(b"", b""), 0);
        assert_eq!(hamming_bytes(b"test", b"text"), 1);
        assert_eq!(hamming_bytes(b"behemoth", b"chan"), 8);
        assert_eq!(hamming_bytes(b"ascii", b"skc"), 4);
        assert_eq!(hamming_bytes(b"skc", b"ascii"), 4);
        assert_eq!(hamming_bytes(b"lodge", b"lodge"), 0);
        assert_eq!(hamming_bytes(b"lodge", b""), 5);
        assert_eq!(hamming_bytes(b"", b"lodge"), 5);
        assert_eq!(hamming_bytes(b"temptation", b"tamper"), 7);
    }
}
