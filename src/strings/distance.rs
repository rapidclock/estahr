pub fn levenshtein_ascii(str_a: &str, str_b: &str) -> usize {
    todo!()
}

pub fn levenshtein_bytes(bytes_a: &[u8], bytes_b: &[u8]) -> usize {
    todo!()
}

pub fn hamming_ascii(str_a: &str, str_b: &str) -> usize {
    if str_a.len() <= str_b.len() {
        hamming(&mut str_a.chars(), &mut str_b.chars())
    } else {
        hamming(&mut str_b.chars(), &mut str_a.chars())
    }
}

fn hamming<A: PartialOrd>(it_a: &mut dyn Iterator<Item=A>, it_b: &mut dyn Iterator<Item=A>) -> usize {
    let mut distance :usize = 0;
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
    return distance;
}

pub fn hamming_bytes(bytes_a: &[u8], bytes_b: &[u8]) -> usize {
    if bytes_a.len() <= bytes_b.len() {
        hamming(&mut bytes_a.iter(), &mut bytes_b.iter())
    } else {
        hamming(&mut bytes_b.iter(), &mut bytes_a.iter())
    }
}

pub fn jaro_winkler_ascii(str_a: &str, str_b: &str) -> usize {
    todo!()
}

pub fn jaro_winkler_bytes(bytes_a: &[u8], bytes_b: &[u8]) -> usize {
    todo!()
}