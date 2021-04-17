// given a list of integers, use a vector & return
// - mean (avg value)
// - median (when sorted, the value in the middle position)
// - mode (most occurring value)
use rand::Rng;
use std::collections::HashMap;

fn gen_rand_number() -> Vec<u32> {
    let mut number_list: Vec<u32> = Vec::new();
    let mut count = 0;

    while count < 100 {
        let number = rand::thread_rng().gen_range(0..101);
        number_list.push(number);
        count += 1;
    }
    number_list
}

fn get_mean(list: &[u32]) -> f64{
    let mut sum = 0;

    for l  in list {
        sum = *l + sum;
    }

    let mean: f64 = (sum as f64) / (list.len() as f64);
    mean
}

fn get_median(list: &[u32]) -> u32{
    // list is sorted

    let list_len = list.len();
    let mid = list_len / 2;

    if list_len % 2 == 0 {
        get_mean(&list[(mid - 1)..(mid + 1)]) as u32
    } else {
        list[mid] as u32
    }
}

fn get_mode(list: &[u32]) -> u32{
    // list is sorted

    let  mut map = HashMap::new();

    for l in list {
        let count = map.entry(l).or_insert(0);
        *count += 1;
    }

    let max_value = map.iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap();

    **max_value.0
}


// Convert strings to pig latin. The first consonant of each word is moved to
// the end of the word and "ay" is added to the end instead.
// "first" becomes "irst-fay"
// Words beginning with vowels have "hay" added to the end instead
// "apple" becomes "apple-hay".
// Keep in mind details of  UTF-8 encoding.

fn convert_to_pig_latin(s: &mut String) -> String {
    let consonants = ["b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "w", "x", "y", "z"];
    let vowels = ["a", "e", "i", "o", "u"];
    let first_char = &s[0..1].to_lowercase();
    let mut pig_latin_string = String::new();

    for c in consonants.iter() {
        if c == first_char {
            s.remove(0);
            pig_latin_string = format!("{}-{}ay", s, first_char);
        }
    }

    for v in vowels.iter() {
        if v == first_char {
            pig_latin_string = format!("{}-hay", s);
        }
    }
    pig_latin_string
}


// Using a hash map & vectors, create a text interface to allow a user to add
// employee names to a department in a company.
// eg. "Add Sally to Engineering" or  "Add Amir to Sales"
// They let the user retrieve a list of all people in a department or all
// people in the company by department, sorted alphabetically.

fn main() {
    let mut number_list = gen_rand_number();
    // sort list in place
    number_list.sort();

    let mean = get_mean(&number_list);
    println!("Mean: {} ", mean);


    let median = get_median(&number_list);
    println!("Median : {}", median);

    let mode = get_mode(&number_list);
    println!("Mode: {}", mode);

    let mut str_starts_with_consonant = String::from("Kirimi");
    let mut str_starts_with_vowel = String::from("Aeroplane");
    let converted_str_starts_with_consonant = convert_to_pig_latin(&mut str_starts_with_consonant);
    let converted_str_starts_with_vowel = convert_to_pig_latin(&mut str_starts_with_vowel);
    println!("Converted string that starts with a consonant: {}", converted_str_starts_with_consonant);
    println!("Converted string that starts with a vowel: {}", converted_str_starts_with_vowel);
}
