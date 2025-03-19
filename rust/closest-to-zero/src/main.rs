fn main() {
    println!("Hello, world!");
}

fn closest_to_zero(numbers: Vec<u32>) -> u32 {
    numbers[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_closest_to_zero() {
        assert_eq!(closest_to_zero(vec![1, 2, 3, 4]), 1);
        assert_eq!(closest_to_zero(vec![4, 3, 1, 2]), 1);
    }
}
