fn is_div(value: u32, divisor: u32) -> bool {
    value % divisor == 0
}

fn is_leap_year(year: u32) -> bool {
    let d4 = is_div(year, 4);
    let d100 = is_div(year, 100);
    let d400 = is_div(year, 400);

    if d400 { return true; }
    if !d100 && d4 { return true; }
    false
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_is_div() {
        assert!(is_div(12, 3));
        assert!(is_div(30, 5));
        assert!(!is_div(11, 3));
    }

    #[test]
    fn test_leap_year() {

        assert!(is_leap_year(2000));
        assert!(!is_leap_year(2001));
        assert!(is_leap_year(1996));
        assert!(!is_leap_year(1900));
    }
}