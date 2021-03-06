use std::cmp::{max, min};

fn levenshtein_distance<A: PartialOrd>(it_a: &mut dyn Iterator<Item=A>, it_b: &mut dyn Iterator<Item=A>) -> usize {
    let vec_a: Vec<_> = it_a.collect();
    let vec_b: Vec<_> = it_b.collect();
    match (vec_a.len(), vec_b.len()) {
        (0, b) => b,
        (a, 0) => a,
        (a, b) => {
            if a <= b {
                levenshtein(vec_a, vec_b)
            } else {
                levenshtein(vec_b, vec_a)
            }
        }
    }
}

fn levenshtein<A: PartialOrd>(vec_a: Vec<A>, vec_b: Vec<A>) -> usize {
    let (mut left, mut top, mut across);
    let m= vec_a.len();
    let mut cache: Vec<usize> = Vec::with_capacity(m);
    (0..m).for_each(|i| cache.push(i + 1));
    for (i, item_b) in vec_b.iter().enumerate() {
        left = i + 1;
        across = left - 1;
        for (j, item_a) in vec_a.iter().enumerate() {
            top = cache[j];
            if item_a.eq(item_b) {
                cache[j] = across;
            } else {
                cache[j] = min_3(left, across, top) + 1;
            }
            across = top;
            left = cache[j];
        }
    }
    cache[m - 1]
}

fn min_3(a: usize, b: usize, c: usize) -> usize {
    if (a < b) && (a < c) {
        a
    } else if b < c {
        b
    } else {
        c
    }
}


/// Calculates the Levenshtein distance(edit distance) between the two strings.
///
/// The levenshtein distance takes into account additions, substitutions and deletions,
/// weighted equally.
///
/// Assumes an ascii string.
pub fn levenshtein_ascii(str_a: &str, str_b: &str) -> usize {
    levenshtein_distance(&mut str_a.chars(), &mut str_b.chars())
}

/// Calculates the Levenshtein distance(edit distance) between the two byte slices.
///
/// The levenshtein distance takes into account additions, substitutions and deletions,
/// weighted equally.
///
/// Assumes an ascii string.
pub fn levenshtein_bytes(bytes_a: &[u8], bytes_b: &[u8]) -> usize {
    levenshtein_distance(&mut bytes_a.iter(), &mut bytes_b.iter())
}

/// Calculates the Hamming distance between the two strings.
///
/// Assumes an ascii string.
pub fn hamming_ascii(str_a: &str, str_b: &str) -> usize {
    if str_a.len() <= str_b.len() {
        hamming_distance(&mut str_a.chars(), &mut str_b.chars())
    } else {
        hamming_distance(&mut str_b.chars(), &mut str_a.chars())
    }
}
/// Calculates the Hamming distance between the two byte slices.
pub fn hamming_bytes(bytes_a: &[u8], bytes_b: &[u8]) -> usize {
    if bytes_a.len() <= bytes_b.len() {
        hamming_distance(&mut bytes_a.iter(), &mut bytes_b.iter())
    } else {
        hamming_distance(&mut bytes_b.iter(), &mut bytes_a.iter())
    }
}

fn hamming_distance<A: PartialOrd>(it_a: &mut dyn Iterator<Item=A>, it_b: &mut dyn Iterator<Item=A>) -> usize {
    let mut distance: usize = 0;
    for item_a in it_a {
        if let Some(item_b) = it_b.next() {
            if item_a.ne(&item_b) {
                distance += 1;
            }
        }
    }
    for _ in it_b {
        distance += 1;
    }
    distance
}

/// Calculates the Jaro Winkler distance between the two strings.
///
/// 0 indicates no distance and 1 is the max distance.
///
/// Assumes an ascii string.
pub fn jaro_winkler_ascii(str_a: &str, str_b: &str) -> f64 {
    let vec_a = str_a.chars().collect::<Vec<char>>();
    let vec_b = str_b.chars().collect::<Vec<char>>();
    jaro_winkler_distance(&vec_a, &vec_b)
}


/// Calculates the Jaro Winkler distance between the two byte slices.
///
/// 0 indicates no distance and 1 is the max distance.
pub fn jaro_winkler_bytes(bytes_a: &[u8], bytes_b: &[u8]) -> f64 {
    jaro_winkler_distance(bytes_a, bytes_b)
}

fn jaro_winkler_distance<A: PartialEq>(it_a: &[A], it_b: &[A]) -> f64 {
    let len_a = it_a.len();
    let len_b = it_b.len();

    // Basic Edge cases
    if len_a == 0 && len_b == 0 {
        return 0.0;
    } else if len_a == 0 || len_b == 0 {
        return 1.0;
    }

    let mut vec_a = vec![false; len_a];
    let mut vec_b = vec![false; len_b];
    let mut matches = 0;
    let mut transpositions = 0.0;

    let search_size: isize = ((max(len_a, len_b) / 2) - 1) as isize;

    // Matches calculation
    for i in 0..len_a {
        let start = max(0, i as isize - search_size) as usize;
        let end = min(i + search_size as usize + 1, len_b) as usize;
        for j in start..end {
            if vec_b[j] || it_a[i].ne(&it_b[j]) {
                continue;
            }
            vec_a[i] = true;
            vec_b[j] = true;
            matches += 1;
            break;
        }
    }

    if matches == 0 { return 1.0; }

    // Transpositions calculation
    let mut k = 0 as usize;
    for i in 0..len_a {
        if !vec_a[i] {
            continue;
        }
        while k < len_b && !vec_b[k] {
            k += 1;
        }
        if it_a[i].ne(&it_b[k]) {
            transpositions += 1.0;
        }
        k += 1;
    }
    transpositions /= 2.0;
    let jaro_number = ((matches as f64 / len_a as f64) + (matches as f64 / len_b as f64) + ((matches as f64 - transpositions) / matches as f64)) / 3.0;

    let mut pref_len = 0;
    for i in 0..len_a {
        if i == 4 { break; }
        if i < len_b {
            if it_a[i] == it_b[i] {
                pref_len += 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    pref_len = min(4, pref_len);

    let jaro_winkler_number = jaro_number + (pref_len as f64 * 0.1 * (1.0 - jaro_number));

    round_two_digits(1.0 - jaro_winkler_number)
}

fn round_two_digits(num: f64) -> f64 {
    (num * 100.0).round() / 100.0
}