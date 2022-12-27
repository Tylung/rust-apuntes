/*
El slice permite referirte a una parte de una collection o a un trozo
es un tipo de referencia por lo que no tiene ownership
*/
// Slices let you reference a contiguous sequence of elements
// in a collection rather than the whole collection.
// A slice is a kind of reference, so it does not have ownership.

// without slices
fn _first_word_example(phrase: &String) -> usize {
    let bytes = phrase.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    phrase.len()
}

// string slice is a part of string

fn _string_slice() {
    // We create slices using a range within brackets by specifying
    // [starting_index..ending_index], where starting_index is the first
    // position in the slice and ending_index is one
    // more than the last position in the slice.
    let s = String::from("hello world");
    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];
    let _s2: &String = &s; // not a slice, for comparison

    println!("s: {s}, hello: {hello}, world: {world}");
}

fn first_word(phrase: &str) -> &str {
    let bytes = phrase.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &phrase[0..i];
        }
    }

    &phrase[..]
}

fn slices_awesome() {
    let nums = [1, 2, 3, 4, 5, 6];

    let slice = &nums[1..3];

    assert_eq!(slice, &[2, 3]);

    println!("my slice: {:#?}", slice);
}

pub fn main() {
    let words = String::from("Hello Rustacean Dev!");
    let word = first_word(&words[0..10]);
    let string_literal = "Some phrase example";
    let word2 = first_word(&string_literal);
    // words.clear();
    println!("word: {word}");
    println!("word2: {word2}");
    slices_awesome();
}
