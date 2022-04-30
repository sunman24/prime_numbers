#[cfg(test)]

#[allow(unused_imports)]
use crate::{is_prime1, is_prime2, is_prime3, is_prime4};

#[test]
fn test_check_prime1() {
    let prime_numbers = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    let not_prime_numbers = [0, 1, 4, 6, 8, 9, 10, 12, 14, 15,
        16, 18, 20, 21, 22, 24, 25, 26, 27, 28];

    prime_numbers.iter().for_each(|&num|
        assert_eq!(is_prime1(num), true, "check_prime1 failed at number {}", num));
    not_prime_numbers.iter().for_each(|&num|
        assert_eq!(is_prime1(num), false, "check_prime1 failed at number {}", num));
}

#[test]
fn test_check_prime2() {
    let prime_numbers = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    let not_prime_numbers = [0, 1, 4, 6, 8, 9, 10, 12, 14, 15,
        16, 18, 20, 21, 22, 24, 25, 26, 27, 28];

    prime_numbers.iter().for_each(|&num|
        assert_eq!(is_prime2(num), true, "check_prime2 failed at number {}", num));
    not_prime_numbers.iter().for_each(|&num|
        assert_eq!(is_prime2(num), false, "check_prime2 failed at number {}", num));
}

#[test]
fn test_check_prime3() {
    let prime_numbers = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    let not_prime_numbers = [0, 1, 4, 6, 8, 9, 10, 12, 14, 15,
        16, 18, 20, 21, 22, 24, 25, 26, 27, 28];

    prime_numbers.iter().for_each(|&num|
        assert_eq!(is_prime3(num), true, "check_prime3 failed at number {}", num));
    not_prime_numbers.iter().for_each(|&num|
        assert_eq!(is_prime3(num), false, "check_prime3 failed at number {}", num));
}

#[test]
fn test_check_prime4() {
    let prime_numbers = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    let not_prime_numbers = [0, 1, 4, 6, 8, 9, 10, 12, 14, 15,
        16, 18, 20, 21, 22, 24, 25, 26, 27, 28];

    prime_numbers.iter().for_each(|&num|
        assert_eq!(is_prime4(num), true, "check_prime4 failed at number {}", num));
    not_prime_numbers.iter().for_each(|&num|
        assert_eq!(is_prime4(num), false, "check_prime4 failed at number {}", num));
}