fn main() {
    println!("Hello, world!");
}

fn closest_to_zero(numbers: Vec<u32>) -> u32 {
    if numbers.len() == 0 {
        return 0
    }
    let mut min: u32 = numbers[0];
    for n in numbers {
        if n < min {
            min = n
        }
    }

    min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_closest_to_zero() {
        assert_eq!(closest_to_zero(vec![]), 0);
        assert_eq!(closest_to_zero(vec![1, 2, 3, 4]), 1);
        assert_eq!(closest_to_zero(vec![4, 3, 1, 2]), 1);
    }
}
