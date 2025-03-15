fn is_leap_year(year: u32) -> bool {
    let div_4 = year % 4 == 0

    false
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_leap_year() {

        assert!(is_leap_year(2000));
        assert!(!is_leap_year(2001));
        assert!(is_leap_year(1996));
        assert!(!is_leap_year(1900));
    }
}