//! This module has utility functions for strings.
//!
//! You can get different kind of distances between strings.
//!
//! The string distances currently provided are:
//! 1. Hamming Distance [Wiki](https://en.wikipedia.org/wiki/Hamming_distance)
//! 2. Levenshtein Distance [Wiki](https://en.wikipedia.org/wiki/Levenshtein_distance)
//! 3. Jaro Winkler Distance [Wiki](https://en.wikipedia.org/wiki/Jaro-Winkler_distance)
//!
//! The Hamming and Levenshtein give the absolute values, but the Jaro Winkler is a measure of
//! distance that is normalized (between 0 and 1; 1 is identical)

mod distance;

pub use distance::{hamming_ascii, hamming_bytes, levenshtein_ascii, levenshtein_bytes, jaro_winkler_ascii, jaro_winkler_bytes};