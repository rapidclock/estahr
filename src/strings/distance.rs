use std::cmp::min;

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
    let (mut left, mut top, mut across) = (0, 0, 0);
    let (m, n) = (vec_a.len(), vec_b.len());
    let mut cache: Vec<usize> = Vec::with_capacity(m);
    (0..m).for_each(|i| cache.push(i + 1));
    for i in 0..n {
        left = i + 1;
        across = left - 1;
        for j in 0..m {
            top = cache[j];
            if vec_a[j].eq(&vec_b[i]) {
                cache[j] = across;
            } else {
                cache[j] = min_3(left, across, top) + 1;
            }
            across = top;
            left = cache[j];
        }
    }
    return cache[m-1];
}

fn min_3(a: usize, b: usize, c: usize) -> usize {
    if (a < b) && (a < c) {
        a
    } else if (b < c) {
        b
    } else {
        c
    }
}

pub fn levenshtein_ascii(str_a: &str, str_b: &str) -> usize {
    levenshtein_distance(&mut str_a.chars(), &mut str_b.chars())
}

pub fn levenshtein_bytes(bytes_a: &[u8], bytes_b: &[u8]) -> usize {
    levenshtein_distance(&mut bytes_a.iter(), &mut bytes_b.iter())
}

pub fn hamming_ascii(str_a: &str, str_b: &str) -> usize {
    if str_a.len() <= str_b.len() {
        hamming_distance(&mut str_a.chars(), &mut str_b.chars())
    } else {
        hamming_distance(&mut str_b.chars(), &mut str_a.chars())
    }
}

pub fn hamming_bytes(bytes_a: &[u8], bytes_b: &[u8]) -> usize {
    if bytes_a.len() <= bytes_b.len() {
        hamming_distance(&mut bytes_a.iter(), &mut bytes_b.iter())
    } else {
        hamming_distance(&mut bytes_b.iter(), &mut bytes_a.iter())
    }
}

fn hamming_distance<A: PartialOrd>(it_a: &mut dyn Iterator<Item=A>, it_b: &mut dyn Iterator<Item=A>) -> usize {
    let mut distance: usize = 0;
    while let Some(item_a) = it_a.next() {
        if let Some(item_b) = it_b.next() {
            if item_a.ne(&item_b) {
                distance += 1;
            }
        }
    }
    while let Some(_) = it_b.next() {
        distance += 1;
    }
    distance
}

pub fn jaro_winkler_ascii(str_a: &str, str_b: &str) -> usize {
    todo!()
}

pub fn jaro_winkler_bytes(bytes_a: &[u8], bytes_b: &[u8]) -> usize {
    todo!()
}