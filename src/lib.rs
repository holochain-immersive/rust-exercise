/**
 * BASIC SINTAX
 * ============
 *
 *
 */

// Return the sum of a and b
pub fn sum(a: u32, b: u32) -> u32 {
    unimplemented!()
}

// Return the multiplication of a and b
pub fn multiply(a: u32, b: u32) -> u32 {
    unimplemented!()
}

// Return the maximum number between a and b
// Eg: max(3, 4) = 4
pub fn max(a: u32, b: u32) -> u32 {
    unimplemented!()
}

// Return the average for the two given numbers
// Eg: average(2, 3) = 2.5
pub fn average(a: u32, b: u32) -> f32 {
    unimplemented!()
}

// Return the factorial for the given number
// The factorial of a number is the product of all its previous numbers
// Eg: factorial(4) = 1 * 2 * 3 * 4 = 24
pub fn factorial(n: u32) -> u32 {
    unimplemented!()
}

// Return whether the given number is a prime number
// Eg: is_prime(7) = true
// Eg: is_prime(8) = false
pub fn is_prime(n: u32) -> bool {
    unimplemented!()
}

// Return the nth prime, assuming that the first prime number is 2
// Eg: nth_prime(1) = 2
// Eg: nth_prime(2) = 3
// Eg: nth_prime(3) = 5
pub fn nth_prime(n: u32) -> u32 {
    unimplemented!()
}

/**
 * PRIMITIVE COMPOUND TYPES
 * ========================
 *
 *
 */

/// Tuples

// Return the first
pub fn return_first_member(tuple: (u32, u32)) -> u32 {
    unimplemented!()
}

// Return the sum of all the members of the tuple
// Eg: sum((1, 2, 3)) == 6
pub fn sum_members(tuple: (u32, u32, u32)) -> u32 {
    unimplemented!()
}

// Build a tuple of 3 members that are all equal to the given number
// Eg: triplicate_number(1) == (1, 1, 1)
pub fn triplicate_number(n: u32) -> (u32, u32, u32) {
    unimplemented!()
}

// Build a tuple of 3 members that are all equal to the given string
// Eg: triplicate_string(String::from("a")) == (String::from("a"), String::from("a"), String::from("a"))
pub fn triplicate_string(s: String) -> (String, String, String) {
    unimplemented!()
}

/// Arrays

// Return an array of 5 elements, and fill it with the given number
// Eg: build_array(2) == [2, 2, 2, 2, 2]
pub fn build_array(n: u32) -> [u32; 5] {
    unimplemented!()
}

// Return the nth element of the array
// Eg: nth_element([2, 5, 3, 8, 1], 2) == 3
pub fn nth_element(array: [u32; 5], n: usize) -> u32 {
    unimplemented!()
}

// Return the maximum number in the given array
// Eg: max_in_array([2, 5, 3, 8, 1]) == 8
pub fn max_in_array(array: [u32; 5]) -> u32 {
    unimplemented!()
}

// Returns true if the given string literal is either "foo" or "bar"
// Eg: is_foo_or_bar("foo") == true
// Eg: is_foo_or_bar("bar") == true
// Eg: is_foo_or_bar("another string") == false
pub fn is_foo_or_bar(s: &str) -> bool {
    unimplemented!()
}

// Returns true if the given string literal contains the given character
// Eg: contains_char("foo", 'f') == true
// Eg: contains_char("foo", 's') == false
pub fn contains_char(s: &str, character: char) -> bool {
    unimplemented!()
}

// Returns true if the given string literal consists of just its first half repeteated twice
// Eg: is_first_half_repeated("fofo") == true
// Eg: is_first_half_repeated("ff") == true
// Eg: is_first_half_repeated("fof") == false
// Eg: is_first_half_repeated("fofoo") == false
// Eg: is_first_half_repeated("foo") == false
pub fn is_first_half_repeated(s: &str) -> bool {
    unimplemented!()
}
