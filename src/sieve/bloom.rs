pub struct Filter {
    size: usize,
    d: u32,
    bits: Vec<u128>,
    set_bits: u32,
}

impl Default for Filter {
    fn default() -> Self {
        Self::new(1_000_000, 0.01)
    }
}

impl Filter {
    pub fn new(n: usize, p: f64) -> Filter {
        let log_2 = 2_f64.ln();
        let log_p = p.ln();

        let size = ((-(n as f64) * log_p) / (log_2 * log_2)) as usize;
        let d = (-log_p / log_2).ceil() as u32;
        let bits = vec![0; (size as f64 / 128.0).ceil() as usize];
        let set_bits = 0;

        Filter {
            size,
            d,
            bits,
            set_bits,
        }
    }

    pub fn add(&mut self, data: &[u8]) {
        let (h1, h2) = Self::hash(data);

        for i in 0..self.d {
            let bit = (h1 as u128 + h2 as u128 * i as u128) as usize % self.size;
            self.set_bits -= self.bits[bit / 128].count_ones();
            self.bits[bit / 128] |= 1 << (bit % 128);
            self.set_bits += self.bits[bit / 128].count_ones();
        }
    }

    pub fn contains(&mut self, data: &[u8]) -> bool {
        let (h1, h2) = Self::hash(data);

        for i in 0..self.d {
            let bit = (h1 as u128 + h2 as u128 * i as u128) as usize % self.size;
            if self.bits[bit / 128] & (1 << (bit % 128)) == 0 {
                return false;
            }
        }
        true
    }

    pub fn estimated_size(&self) -> usize {
        (-(self.size as f64 / self.d as f64)
            * (1f64 - self.set_bits as f64 / self.size as f64).ln()) as usize
    }

    fn hash(data: &[u8]) -> (u64, u64) {
        let h = fastmurmur3::hash(data);
        let mask: u128 = (1 << 64) - 1;
        ((h & mask) as u64, (h >> 64) as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_add_contains() {
        let mut b = Filter::new(100, 0.001);

        let el = &10_u32.to_be_bytes();

        b.add(el);
        assert!(b.contains(el));

        let el = &11_u32.to_be_bytes();
        assert!(!b.contains(el));
    }

    #[test]
    pub fn test_precision() {
        let n = 100_000;
        let p = 0.1;
        let mut b = Filter::new(n, p);

        for i in 0..n {
            b.add(&(i as u32).to_be_bytes());
        }

        let m = 100_000;
        let mut positive_count = 0;
        for i in n..n + m {
            positive_count += if b.contains(&(i as u32).to_be_bytes()) {
                1
            } else {
                0
            };
        }

        let rate = positive_count as f64 / m as f64;
        assert!(rate <= p * 1.1 && rate >= p * 0.9);
    }
}
