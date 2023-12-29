use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let roman_to_int_map: HashMap<char, i32> = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    let mut sum: i32 = 0;
    let mut prev_value: i32 = 0;

    for char in s.chars() {
        let value = match roman_to_int_map.get(&char) {
            Some(v) => *v,
            None => panic!()
        };

        if value > prev_value {
            sum += value - 2 * prev_value;
        } else {
            sum += value;
        }

        prev_value = value;
    }

    sum
}

#[cfg(test)]
mod test_roman_to_integer {
    use crate::problems::roman_to_integer::roman_to_int;

    #[test]
    fn test1() {
        assert_eq!(3, roman_to_int("III".to_owned()));
    }

    #[test]
    fn test2() {
        assert_eq!(58, roman_to_int("LVIII".to_owned()));
    }

    #[test]
    fn test3() {
        assert_eq!(1994, roman_to_int("MCMXCIV".to_owned()));
    }
}
