/**
 * BASIC SINTAX
 * ============
 *
 *
 */

// Return the sum of a and b
pub fn sum(a: u32, b: u32) -> u32 {
    a + b
}

// Return the multiplication of a and b
pub fn multiply(a: u32, b: u32) -> u32 {
    a * b
}

// Return the maximum number between a and b
// Eg: max(3, 4) = 4
pub fn max(a: u32, b: u32) -> u32 {
    if a < b {
        b
    } else {
        a
    }
}

// Return the average for the two given numbers
// Eg: average(2, 3) = 2.5
pub fn average(a: u32, b: u32) -> f32 {
    let sum = (a + b) as f32;

    sum / 2.0
}

// TODO: some exercise that requires while

// Return the factorial for the given number
// The factorial of a number is the product of all its previous numbers
// Eg: factorial(4) = 1 * 2 * 3 * 4 = 24
pub fn factorial(n: u32) -> u32 {
    let mut f = 1;

    for i in 1..=n {
        f = f * i;
    }

    f
}

// Return whether the given number is a prime number
// Eg: is_prime(7) = true
// Eg: is_prime(8) = false
pub fn is_prime(n: u32) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }

    true
}

// Return the nth prime, assuming that the first prime number is 2
// Eg: nth_prime(1) = 2
// Eg: nth_prime(2) = 3
// Eg: nth_prime(3) = 5
// QUESTION: Maybe extra curriculum?
pub fn nth_prime(n: u32) -> u32 {
    let mut primes_found = 0;
    let mut i = 1;

    while primes_found < n {
        i += 1;

        if is_prime(i) {
            primes_found += 1;
        }
    }

    i
}

// QUESTION: Closure exercise? Seems too advanced here?

/**
 * MOVE SEMANTICS
 * ==============
 *
 *
 */

/// Ownership

//
pub fn double(n: u32) -> u32 {
    n + n
}

// Appends the given char "c" to the given String "s"
// Eg: append_char(String::from("hello"), '0') = String::from("hello0")
pub fn append_char(s: String, c: char) -> String {
    let mut s = s;

    s.push(c);

    s
}

fn merge_strings(mut s1: String, s2: String) -> String {
    s1.push_str(s2.as_str());

    s2
}

// Eg: double_append_char(String::from("hello"), '1', '0') = String::from("hello0hello1")
pub fn double_append_char(s: String, c1: char, c2: char) -> String {
    let s1 = append_char(s.clone(), c1);
    let s2 = append_char(s, c2);

    merge_strings(s1, s2)
}

// TODO: how to make exercise for which you have to exercise ownership

/// References

// Appends the given char "c" to the given String "s"
// Eg: let s = String::from("hello"); append_char(, '0') = String::from("hello0")
pub fn append_char_mut(s: &mut String, c: char) -> () {
    s.push(c);
}

// TODO: add more exercises for references

/**
 * PRIMITIVE COMPOUND TYPES
 * ========================
 *
 *
 */

/// Tuples

// Return the first
pub fn return_first_member(tuple: (u32, u32)) -> u32 {
    tuple.0
}

// Return the sum of all the members of the tuple
// Eg: sum((1, 2, 3)) = 6
pub fn sum_members(tuple: (u32, u32, u32)) -> u32 {
    let (first, second, third) = tuple;

    first + second + third
}

// Build a tuple of 3 members that are all equal to the given number
// Eg: triplicate_number(1) = (1, 1, 1)
pub fn triplicate_number(n: u32) -> (u32, u32, u32) {
    (n, n, n)
}

// Build a tuple of 3 members that are all equal to the given string
// Eg: triplicate_string(String::from("a")) = (String::from("a"), String::from("a"), String::from("a"))
pub fn triplicate_string(s: String) -> (String, String, String) {
    (s.clone(), s.clone(), s)
}

/// Arrays

// Return an array of 10 elements, and fill it with the given number
// Eg: build_array(2) = [2, 2, 2, 2, 2]
pub fn build_array(n: u32) -> [u32; 5] {
    [n; 5]
}

// Return the nth element of the array
// Eg: nth_element([2, 5, 3, 8, 1], 2) = 3
pub fn nth_element(array: [u32; 5], n: usize) -> u32 {
    array[n]
}

// Return the maximum number in the given array
// Eg: max_in_array([2, 5, 3, 8, 1]) = 8
pub fn max_in_array(array: [u32; 5]) -> u32 {
    let mut max = 0;

    for n in array {
        if n > max {
            max = n;
        }
    }

    max
}

/**
 * NON PRIMITIVE NATIVE TYPES
 * ==========================
 */

/// String literal

// Convert the given string literal to a string struct
pub fn convert_to_string_struct(string_literal: &str) -> String {
    String::from(string_literal)
}

// Append the word "world" to the given string
// Eg: append_world(String::from("hello, ")) = String::from("hello, world")
pub fn append_world(mut s: String) -> String {
    s.push_str("world");

    s
}

// Returns true if the given string literal is either "foo" or "bar"
// Eg: is_foo_or_bar("foo") = true
// Eg: is_foo_or_bar("bar") = true
// Eg: is_foo_or_bar("another string") = false
pub fn is_foo_or_bar(s: &str) -> bool {
    s == "foo" || s == "bar"
}

/// Structs

pub struct Person {
    pub name: String,
    pub age: u32,
}

//
// Eg: longest_name(Person::new(String::from("Marcus"), 23), Person::new(String::from("William"), 33)) = String::from("William")
pub fn longest_name(person1: &Person, person2: &Person) -> String {
    if person1.name.len() < person2.name.len() {
        person2.name.clone()
    } else {
        person1.name.clone()
    }
}

impl Person {
    // Returns whether this person is less than 18 years old
    // Eg: is_underage(Person::new(String::from("Niki"), 19)) = false
    pub fn is_underage(&self) -> bool {
        self.age < 18
    }

    // Sets the age for this person
    pub fn set_age(&mut self, age: u32) -> () {
        self.age = age;
    }
}

// TODO: tuple and unit type variants

/// Strings

// Returns a greeting for the given person
// Eg: greet(Person::new(String::from("Tony"), 43)) = String::from("Hello Tony, you are 43 years old!")
pub fn greet(person: Person) -> String {
    format!("Hello {}, you are {} years old!", person.name, person.age)
}

// TODO: Add more exercises

pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    // Return a new instance of the Rectangle struct
    pub fn new(width: f32, height: f32) -> Rectangle {
        Rectangle { width, height }
    }

    // Compute the area for this shape
    pub fn area(&self) -> f32 {
        self.width * self.height
    }
}

/// Enums

pub enum Operation {
    Increment,
    Decrement,
}

// Count the number of Increment operations for the given array
// Eg: count_increments([Operation::Increment, Operation::Decrement, Operation::Increment]) = 2
pub fn count_increments(operations: [Operation; 3]) -> u32 {
    let mut increments = 0;

    for op in operations {
        if let Operation::Increment = op {
            increments += 1;
        }
    }

    increments
}

// Perform the given operation on the given counter
// Eg: perform_operation(4, Operation::Increment) = 5
// Eg: perform_operation(0, Operation::Decrement) = 0 (!!)
pub fn perform_operation(counter: u32, operation: Operation) -> u32 {
    match operation {
        Operation::Increment => counter + 1,
        Operation::Decrement if counter == 0 => 0,
        Operation::Decrement => counter - 1,
    }
}

///

pub enum Shape {
    // TODO: remove variants
    Rectangle { width: f32, height: f32 },
    Square { size: f32 },
    Triangle { base: f32, height: f32 },
}

impl Shape {
    // Compute the area for this shape
    pub fn area(&self) -> f32 {
        match self {
            Self::Triangle { base, height } => base * height / 2.0,
            Self::Rectangle { width, height } => width * height,
            Self::Square { size } => size * size,
        }
    }
}

pub enum BiggerResult {
    ShapesAreEqual,
    OneShapeIsBigger(Shape),
}

pub fn biggest(shape1: Shape, shape2: Shape) -> BiggerResult {
    let area1 = shape1.area();
    let area2 = shape2.area();

    if area1 == area2 {
        BiggerResult::ShapesAreEqual
    } else if area1 > area2 {
        BiggerResult::OneShapeIsBigger(shape1)
    } else {
        BiggerResult::OneShapeIsBigger(shape2)
    }
}
