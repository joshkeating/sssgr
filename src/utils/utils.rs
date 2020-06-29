
// an overly complex way to not deal with importing the chrono lib
// weights days < months < years
pub fn date_str_to_int(date: &str) -> i32 {

    assert_eq!(date.len(), 10, "provided str length is not 10");

    let mut total: i32 = 0;
    let mut split_date: Vec<&str> = date.split('/').collect();

    for i in 0..3 {
        let mut segment = split_date.pop().unwrap();
        if &segment[0..1] == "0" && segment.len() == 2 {
            segment = &segment[1..2];
        }
        if i == 2 {
            total += segment.parse::<i32>().unwrap() * 100
        } else if i == 0 {
            total += segment.parse::<i32>().unwrap() * 1000
        } else {
            total += segment.parse::<i32>().unwrap();
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use crate::utils::date_str_to_int;

    #[test]
    fn test_date_str_to_int() {
        assert_eq!(date_str_to_int("10/09/2017"), 2018009);
        assert_eq!(date_str_to_int("01/13/1997"), 1997113);
        assert_eq!(date_str_to_int("12/19/2016"), 2017219);
        assert_eq!(date_str_to_int("12/04/2018"), 2019204);
    }

    #[test]
    #[should_panic]
    fn test_any_panic() {
        assert_eq!(date_str_to_int("100/09/2017"), 111);
    }
}