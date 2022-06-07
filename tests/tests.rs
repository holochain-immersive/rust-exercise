use rust_sprint1::*;

#[test]
fn max_test() {
    assert_eq!(max(2, 3), 3);
    assert_eq!(max(5, 4), 5);
    assert_eq!(max(10, 44), 44);
    assert_eq!(max(100, 90), 100);
}

#[test]
fn multiply_test() {
    assert_eq!(multiply(2, 3), 6);
    assert_eq!(multiply(5, 4), 20);
    assert_eq!(multiply(6, 7), 42);
    assert_eq!(multiply(9, 8), 72);
    assert_eq!(multiply(5, 2), 10);
}

#[test]
fn average_test() {
    assert_eq!(average(3, 4), 3.5);
    assert_eq!(average(5, 8), 6.5);
    assert_eq!(average(1, 0), 0.5);
    assert_eq!(average(5, 54), 29.5);
}

#[test]
fn factorial_test() {
    assert_eq!(factorial(4), 24);
    assert_eq!(factorial(5), 120);
    assert_eq!(factorial(6), 720);
    assert_eq!(factorial(7), 5040);
}

#[test]
fn is_prime_test() {
    assert_eq!(is_prime(2), true);
    assert_eq!(is_prime(3), true);
    assert_eq!(is_prime(4), false);
    assert_eq!(is_prime(5), true);
    assert_eq!(is_prime(6), false);
    assert_eq!(is_prime(7), true);
    assert_eq!(is_prime(9), false);
    assert_eq!(is_prime(21), false);
    assert_eq!(is_prime(23), true);
}

#[test]
fn nth_prime_test() {
    assert_eq!(nth_prime(2), 3);
    assert_eq!(nth_prime(3), 5);
    assert_eq!(nth_prime(4), 7);
    assert_eq!(nth_prime(5), 11);
    assert_eq!(nth_prime(6), 13);
    assert_eq!(nth_prime(7), 17);
    assert_eq!(nth_prime(9), 23);
    assert_eq!(nth_prime(21), 73);
    assert_eq!(nth_prime(23), 83);
}

/**
 * MOVE SEMANTICS
 * ==============
 */

#[test]
fn append_char_test() {
    assert_eq!(
        append_char(String::from("hello"), '0'),
        String::from("hello0")
    );
    assert_eq!(
        append_char(String::from("hello"), '1'),
        String::from("hello1")
    );
}

#[test]
fn append_char_mut_test() {
    let mut s = String::from("hello");
    append_char_mut(&mut s, '0');
    assert_eq!(s, String::from("hello0"));

    let mut s = String::from("asdf");
    append_char_mut(&mut s, '1');
    assert_eq!(s, String::from("asdf1"));
}
