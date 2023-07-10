use bloom_filter::BloomFilter;

fn main() {
    let mut filter = BloomFilter::new(1000000, 0.01);

    filter.add(&1);
    filter.add(&2);
    filter.add(&3);

    println!("{}", filter.contains(&2));
    println!("{}", filter.contains(&4));
}

#[cfg(test)]
mod test;