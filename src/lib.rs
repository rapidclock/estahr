mod strings;
mod hash;

pub use strings::{hamming_bytes, hamming_ascii, levenshtein_ascii, levenshtein_bytes};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hamming_ascii_test() {
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

    #[test]
    fn levenshtein_ascii_test() {
    assert_eq!(levenshtein_ascii("", ""), 0);
    assert_eq!(levenshtein_ascii("Different", "Different"), 0);
    assert_eq!(levenshtein_ascii("A", ""), 1);
    assert_eq!(levenshtein_ascii("", "ŵ"), 1);
    assert_eq!(levenshtein_ascii("Java", "JavaScript"), 6);
    assert_eq!(levenshtein_ascii("atomic", "atom"), 2);
    assert_eq!(levenshtein_ascii("object", "inject"), 2);
    assert_eq!(levenshtein_ascii("flaw", "lawn"), 2);
    assert_eq!(levenshtein_ascii("A", "Z"), 1);
    assert_eq!(levenshtein_ascii("gattaca", "tataa"), 3);
    assert_eq!(levenshtein_ascii("attaca", "tataa"), 3);
    assert_eq!(levenshtein_ascii("bullfrog", "frogger"), 7);
    }

    #[test]
    fn levenshtein_bytes_test() {
        assert_eq!(levenshtein_bytes(b"", b""), 0);
        assert_eq!(levenshtein_bytes(b"Different", b"Different"), 0);
        assert_eq!(levenshtein_bytes(b"A", b""), 1);
        assert_eq!(levenshtein_bytes(b"Java", b"JavaScript"), 6);
        assert_eq!(levenshtein_bytes(b"atomic", b"atom"), 2);
        assert_eq!(levenshtein_bytes(b"object", b"inject"), 2);
        assert_eq!(levenshtein_bytes(b"flaw", b"lawn"), 2);
        assert_eq!(levenshtein_bytes(b"A", b"Z"), 1);
        assert_eq!(levenshtein_bytes(b"gattaca", b"tataa"), 3);
        assert_eq!(levenshtein_bytes(b"attaca", b"tataa"), 3);
        assert_eq!(levenshtein_bytes(b"bullfrog", b"frogger"), 7);
    }
}
