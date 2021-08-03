fn main() {
    let sentence = "this time the exam is very serious".to_string();

    for word in sentence.split(' ') {
        if is_three_vowels(word) {
            println!("{} is: a three-vowel word", word);
        }
    }
}

fn is_three_vowels(word: &str) -> bool {
    let mut counter = 0;

    for char in word.chars() {
        match char {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                counter += 1;
                if counter == 3 {
                    return true
                }
            }

            _ => {
                counter = 0
            }
        }
    }
    false
}
