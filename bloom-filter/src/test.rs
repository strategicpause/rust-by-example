

#[cfg(test)]
mod test {
    use bloom_filter::BloomFilter;

    #[test]
    fn add_str() {
        let mut bloom = BloomFilter::new(100, 0.01);

        bloom.add("test");
        assert!(bloom.contains("test"));
    }

    #[test]
    fn add_num() {
        let mut bloom = BloomFilter::new(100, 0.01);

        bloom.add(&1);
        assert!(bloom.contains(&1));

    }

    #[test]
    fn contains_str() {
        let mut bloom = BloomFilter::new(100, 0.01);

        assert!(!bloom.contains("foo"));

        bloom.add("foo");
        assert!(bloom.contains("foo"));
    }

    #[test]
    fn contains_num() {
        let mut bloom = BloomFilter::new(100, 0.01);

        assert!(!bloom.contains(&2));

        bloom.add(&2);
        assert!(bloom.contains(&2));
    }
}