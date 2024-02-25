use fastmurmur3::hash;

pub fn simhash(s: String, shingle_size: usize) -> u128 {
    let n = s.len();
    let s = s.to_lowercase();

    let mut counts = [0; 128];

    // for each shingle_size-gram, compute a 128 bit hash
    for i in 0..(n - shingle_size + 1) {
        let gram = &s[i..i + shingle_size];

        let h = hash(gram.as_bytes());

        for (i, c) in counts.iter_mut().enumerate() {
            *c += if h & (1 << i) == 0 { -1 } else { 1 };
        }
    }

    // if the majority of grams have one set in position i, set one as i-th bit, zero otherwise.
    let mut res = 0;
    for (i, c) in counts.iter().enumerate() {
        if *c >= 0 {
            res |= 1 << i;
        }
    }

    res
}

pub fn hamming_distance(x: u128, y: u128) -> u32 {
    (x ^ y).count_ones()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_equal() {
        let s1 = simhash("hello world".to_string(), 3);
        let s2 = simhash("hello world".to_string(), 3);

        assert_eq!(s1, s2);
    }

    #[test]
    pub fn test_different() {
        let s1 = simhash("the quick brown fox jumps over the lazy dog".to_string(), 3);
        let s2 = simhash(
            "the quick brown fox jumps over the lazy doggg".to_string(),
            3,
        );

        assert!(hamming_distance(s1, s2) < 10);
    }

    #[test]
    pub fn test_hamming() {
        let x = (1 << 10) - 1;
        let y = (1 << 5) - 1;

        assert_eq!(hamming_distance(x, y), 5);
    }
}
