use rust_sprint1::*;

#[test]
fn sum_test() {
    assert_eq!(sum(2, 3), 5);
    assert_eq!(sum(5, 4), 9);
    assert_eq!(sum(6, 7), 13);
    assert_eq!(sum(9, 8), 17);
    assert_eq!(sum(5, 2), 7);
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
fn max_test() {
    assert_eq!(max(2, 3), 3);
    assert_eq!(max(5, 4), 5);
    assert_eq!(max(10, 44), 44);
    assert_eq!(max(100, 90), 100);
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

#[test]
fn return_first_member_test() {
    assert_eq!(return_first_member((4, 5)), 4);
    assert_eq!(return_first_member((4, 6)), 4);
    assert_eq!(return_first_member((10, 1)), 10);
    assert_eq!(return_first_member((10, 4)), 10);
    assert_eq!(return_first_member((7, 4)), 7);
}

#[test]
fn sum_members_test() {
    assert_eq!(sum_members((4, 5, 1)), 10);
    assert_eq!(sum_members((4, 5, 2)), 11);
    assert_eq!(sum_members((1, 2, 3)), 6);
    assert_eq!(sum_members((1, 1, 1)), 3);
}

#[test]
fn triplicate_number_test() {
    assert_eq!(triplicate_number(4), (4, 4, 4));
    assert_eq!(triplicate_number(410), (410, 410, 410));
    assert_eq!(triplicate_number(0), (0, 0, 0));
}

#[test]
fn triplicate_string_test() {
    assert_eq!(
        triplicate_string(String::from("hi")),
        (String::from("hi"), String::from("hi"), String::from("hi"))
    );
    assert_eq!(
        triplicate_string(String::from("world")),
        (
            String::from("world"),
            String::from("world"),
            String::from("world")
        )
    );
}

#[test]
fn build_array_test() {
    assert_eq!(build_array(5), [5, 5, 5, 5, 5]);
    assert_eq!(build_array(1), [1, 1, 1, 1, 1]);
    assert_eq!(build_array(3), [3, 3, 3, 3, 3]);
}

#[test]
fn nth_element_test() {
    assert_eq!(nth_element([1, 2, 3, 4, 5], 0), 1);
    assert_eq!(nth_element([1, 2, 3, 4, 5], 1), 2);
    assert_eq!(nth_element([1, 2, 3, 4, 5], 2), 3);
    assert_eq!(nth_element([1, 2, 3, 4, 5], 3), 4);
    assert_eq!(nth_element([1, 2, 3, 4, 5], 4), 5);
}

#[test]
fn max_in_array_test() {
    assert_eq!(max_in_array([1, 2, 3, 4, 5]), 5);
    assert_eq!(max_in_array([10, 10, 3, 4, 5]), 10);
    assert_eq!(max_in_array([4, 4, 4, 5, 4]), 5);
    assert_eq!(max_in_array([1, 1, 1, 1, 1]), 1);
}

#[test]
fn is_foo_or_bar_test() {
    assert!(is_foo_or_bar("foo"));
    assert!(is_foo_or_bar("bar"));
    assert_eq!(is_foo_or_bar("foor"), false);
    assert_eq!(is_foo_or_bar("bas"), false);
    assert_eq!(is_foo_or_bar("f"), false);
    assert_eq!(is_foo_or_bar("b"), false);
}

#[test]
fn contains_char_test() {
    assert_eq!(contains_char("foo", 'f'), true);
    assert_eq!(contains_char("foo", 's'), false);
    assert_eq!(contains_char("bar", 'b'), true);
    assert_eq!(contains_char("bar", 't'), false);
}

#[test]
fn is_first_half_repeated_test() {
    assert_eq!(is_first_half_repeated("ff"), true);
    assert_eq!(is_first_half_repeated("fofo"), true);
    assert_eq!(is_first_half_repeated("fof"), false);
    assert_eq!(is_first_half_repeated("bar"), false);
    assert_eq!(is_first_half_repeated("foo"), false);
    assert_eq!(is_first_half_repeated("asas"), true);
    assert_eq!(is_first_half_repeated("123451234"), false);
    assert_eq!(is_first_half_repeated("12341234"), true);
    assert_eq!(is_first_half_repeated("12241234"), false);
    assert_eq!(is_first_half_repeated("1234123"), false);
}
