
pub fn date_str_to_int(date: &str) -> i32 {

    assert_eq!(date.len(), 10, "provided str length is not 10");

    let mut total: i32 = 0;
    let split_date: Vec<&str> = date.split('/').collect();

    for n in split_date {
        // if first char of str is 0, only add second char
        if &n[0..1] == "0" && n.len() == 2 {
            let second_char = &n[1..2];
            total += second_char.parse::<i32>().unwrap();
        } else {
            total += n.parse::<i32>().unwrap();
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use crate::utils::date_str_to_int;

    #[test]
    fn test_date_str_to_int() {
        assert_eq!(date_str_to_int("10/09/2017"), 2036);
        assert_eq!(date_str_to_int("01/13/1997"), 2011);
    }

    #[test]
    #[should_panic]
    fn test_any_panic() {
        assert_eq!(date_str_to_int("100/09/2017"), 111);
    }
}