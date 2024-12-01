// Coming from Python, it's interesting that you don't seem to import specific pieces of code.
// For example: By 'use'-ing itertools, the 'counts' method becomes available on any iterator.
use std::env;
use std::fs;
use std::collections::HashMap;

use itertools::Itertools;

/**
 * This is my first ever Rust program. 
 * But, I do have experience programming in Python, Java, C and C++.
 * 
 * Some things stood out to me: 
 *  (1) Rust is well documented. Got started in minutes through: https://www.rust-lang.org/learn/get-started. 
 *  (2) Package management seems a lot easier through Rust's Cargo.
 *  (3) Creating a new project is as simple as running 'cargo new <project-name>'.
 *  (4) The compiler is suuuuper helpful. It can often tell you exactly what's wrong and how to fix it.
 *      It also helps spot potential bugs before they happen.
 *  (5) The language is more strict. It doesn't allow you to do things that are potentially unsafe.
 *      Will definitely take some getting used to again after spending so much time in Python.
 *
 *  This first AoC challenge was easy to solve with the std and itertools libraries.
 */
fn main() {
    
    // I setup the bare bones of this program by following: 
    // "An I/O Project: Building a Command Line Program"
    // https://doc.rust-lang.org/book/ch12-00-an-io-project.html
    // Those docs are a great resource that I should read through more carefully at some point.

    // There's multiple keywords here: const, let, mut.
    // I'm guessing that 'const' is for constants, 'let' is for variables and 'let mut' is for mutable variables.
    // I noticed that for 'const' you have to specify the type hint.

    // ===== File Parsing =====

    // The .collect() method reminds me of the Polars syntax in Python.
    // (although Rust for sure came first here)

    // The Vec seems to be the Rust equivalent to a Python list.
    let args: Vec<String> = env::args().collect();

    // Rust seems to distinguish references, pointers and values explicitly.
    // That will take some getting used to again.
    let file_path = &args[1];

    // The .expect() method seems like an interesting way to jointly handle errors and document code.
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // I found it a little surprising that this didn't return a Vec<String>
    let lines = contents.split("\n");

    // There's no generic 'number' or 'int' type. You have to specify the specific dtype. 
    let mut left_list: Vec<i64> = Vec::new();
    let mut right_list: Vec<i64> = Vec::new();
    
    for line in lines {
        let mut split = line.split("   ");
        let left = split.next().expect("The split should have at least two parts");
        let right = split.next().expect("The split should have at least two parts");

        // Rust supports type casting using the 'as' keyword, but this is not transitive. 
        // This thus wouldn't work here, but we can easily use the built-in .parse() method.
        let left_int: i64 = left.parse::<i64>().expect("The left part should be an unsigned integer");
        let right_int: i64 = right.parse::<i64>().expect("The right part should be an unsigned integer");
        
        left_list.push(left_int);
        right_list.push(right_int);
    }

    // ===== Part I =====
    // Solving the first part of the problem was very easy. 
    // We can just sort the list and compute the sum of the pairwise, absolute differences

    left_list.sort();
    right_list.sort();

    // .zip(), .map() and .collect() is nomenclature I recognize from Python!
    let sum: i64 = left_list
        .iter()
        .zip(right_list.clone())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("The sum of the differences is: {sum}");

    // ===== Part II =====
    // The second part of the problem is a little more involved.
    // But using the .counts() method from itertools, we can solve it in a few lines of code.

    // The HashMap seems to be the Rust equivalent to a Python dictionary.
    let counts: HashMap<i64, usize> = right_list.into_iter().counts();
    
    let similarity_score: i64 = left_list
        .iter()
        .map(|value| {
            // Unwrap is similar to expect, but it doesn't require a message.
            // The _or_ method provides a succinct way of writing a try-except of some sort.
            let count = counts.get(value).unwrap_or(&0);

            // We use * here to dereference a pointer, I believe. 
            if *count == 0 {
                return 0;
            }
            // Type casting! 
            return value * (*count as i64);
        })
        .sum();

    println!("The similarity score is: {similarity_score}");

}
