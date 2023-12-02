use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;
use std::vec;

/// Find the first and last digit (1-9) of a given string slice
/// Solution: get a vector of all matching numerical chars in the string slice
/// Then extract the first and last of that vector
fn find_first_last_digit_1(s: &str) -> (&str, &str) {
    let matches: Vec<_> = s.matches(char::is_numeric).collect();
    (matches.first().unwrap(), matches.last().unwrap())
}

/// Find the first and last digit of a given string slice
/// The digit can a spelled out ("nine") or numerical (1-9)
/// Solution: get the indexes of "one", "two", ... "nine" and digits.
/// Find the smallest and biggest index of all to get the two digits
fn find_first_last_digit_2(s: &str) -> (&str, &str) {
    let digits: Vec<_> = s.match_indices(char::is_numeric).collect();
    let ones: Vec<_> = s.match_indices("one").collect();
    let twos: Vec<_> = s.match_indices("two").collect();
    let threes: Vec<_> = s.match_indices("three").collect();
    let fours: Vec<_> = s.match_indices("four").collect();
    let fives: Vec<_> = s.match_indices("five").collect();
    let sixs: Vec<_> = s.match_indices("six").collect();
    let sevens: Vec<_> = s.match_indices("seven").collect();
    let eights: Vec<_> = s.match_indices("eight").collect();
    let nines: Vec<_> = s.match_indices("nine").collect();

    let mut vec_firsts: Vec<(usize, &str)> = vec![];
    let mut vec_lasts: Vec<(usize, &str)> = vec![];
    // If there is a first, there is necessarily a last
    if let Some(digit) = digits.first() {
        vec_firsts.push(*digit);
        vec_lasts.push(*digits.last().unwrap());
    }
    if let Some(one) = ones.first() {
        vec_firsts.push(*one);
        vec_lasts.push(*ones.last().unwrap());
    }
    if let Some(two) = twos.first() {
        vec_firsts.push(*two);
        vec_lasts.push(*twos.last().unwrap());
    }
    if let Some(three) = threes.first() {
        vec_firsts.push(*three);
        vec_lasts.push(*threes.last().unwrap());
    }
    if let Some(four) = fours.first() {
        vec_firsts.push(*four);
        vec_lasts.push(*fours.last().unwrap());
    }
    if let Some(five) = fives.first() {
        vec_firsts.push(*five);
        vec_lasts.push(*fives.last().unwrap());
    }
    if let Some(six) = sixs.first() {
        vec_firsts.push(*six);
        vec_lasts.push(*sixs.last().unwrap());
    }
    if let Some(seven) = sevens.first() {
        vec_firsts.push(*seven);
        vec_lasts.push(*sevens.last().unwrap());
    }
    if let Some(eight) = eights.first() {
        vec_firsts.push(*eight);
        vec_lasts.push(*eights.last().unwrap());
    }
    if let Some(nine) = nines.first() {
        vec_firsts.push(*nine);
        vec_lasts.push(*nines.last().unwrap());
    }

    // Sort the vecs
    vec_firsts.sort_by(|a, b| a.0.cmp(&b.0));
    vec_lasts.sort_by(|a, b| a.0.cmp(&b.0));

    (vec_firsts.first().unwrap().1, vec_lasts.last().unwrap().1)
}

/// Transform a digit in string slice to an actual u32
/// The digit can a spelled out ("nine") or numerical (1-9)
fn get_number_from_digits(s: &str) -> u32 {
    match s {
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => {
            println!("Unknown digit!");
            0
        }
    }
}

fn main() -> io::Result<()> {
    let start: Instant = Instant::now();

    let file = File::open("./inputs/input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut sum_1: u32 = 0;
    let mut sum_2: u32 = 0;

    for line in lines {
        let line = line.unwrap();
        // Problem 1
        let (first_1, last_1) = find_first_last_digit_1(line.as_str());
        sum_1 += (String::from(first_1) + last_1).parse::<u32>().unwrap();

        // Problem 2
        let (first_2, last_2) = find_first_last_digit_2(line.as_str());
        sum_2 += get_number_from_digits(first_2) * 10 + get_number_from_digits(last_2);
    }

    // Problem 1 solution: 54953
    println!("Problem 1 solution is {}", sum_1);
    // Problem 2 solution: 53868
    println!("Problem 2 solution is {}", sum_2);

    println!("Execution time: {:?}", start.elapsed());

    Ok(())
}
