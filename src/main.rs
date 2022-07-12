use std::io::{self, BufRead};
use substring::Substring;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

fn main() {
    let alpabet_reverse: String = ALPHABET.chars().rev().collect::<String>();
    let mut secret: String = String::new();

    let mut total_positions: i32 = 0;

    let mut count_line: i16 = 0;

    let mut text_decoded: String = String::new();

    for line in io::stdin().lock().lines() {
        let current_line = line.unwrap().trim().to_string();
        if current_line == "" {
            continue;
        }

        if count_line == 0 {
            secret = current_line;

            // calculate total positions based on secret, make here to only doing one time
            total_positions = get_total_positions(secret);
        } else {
            text_decoded.push_str(&process_line(
                &alpabet_reverse,
                total_positions,
                &current_line,
            ));
            text_decoded.push_str("\n");
        }
        count_line += 1;
    }

    println!("{}", text_decoded);
}

fn get_total_positions(secret: String) -> i32 {
    let mut total_positions: i32 = 0;
    for c in secret.chars() {
        total_positions += c as i32;
    }
    return total_positions;
}

fn process_line(alpabet_reverse: &str, total_positions: i32, text_to_decode: &str) -> String {
    let mut final_key: String = String::new();
    let mut last_position_char_in_last_word: i32 = 0;
    let mut last_char_in_last_word: char = '\0';
    let mut first_char_in_last_word: char = '\0';

    for word in text_to_decode.split(" ") {
        let (response, position, last_char, first_char) = process_word(
            alpabet_reverse,
            total_positions,
            word,
            last_position_char_in_last_word,
            last_char_in_last_word,
            first_char_in_last_word,
        );
        last_position_char_in_last_word = position;
        last_char_in_last_word = last_char;
        first_char_in_last_word = first_char;

        final_key.push_str(&response);
        final_key.push(' ');
    }

    return final_key;
}

fn process_word(
    alpabet_reverse: &str,
    total_positions: i32,
    word: &str,
    last_position_char_in_last_word: i32,
    last_char_in_last_word: char,
    first_char_in_last_word: char,
) -> (String, i32, char, char) {
    let mut final_key: String = String::new();
    let mut positions_last_char: i32 = 0;
    let mut is_first: bool = true;
    let mut last_char: char = '\0';
    let mut first_char: char = '\0';

    for c in word.chars() {
        let mut count: i32 = 0;

        if is_first {
            if c.is_uppercase()
                || c.is_numeric()
                || first_char_in_last_word.is_numeric()
                || (!last_char_in_last_word.is_numeric() && !last_char_in_last_word.is_alphabetic())
            {
                positions_last_char = last_position_char_in_last_word;
            }
            first_char = c;
            is_first = false;
        }

        let exist = alpabet_reverse.find(c);

        if exist == None {
            final_key.push_str(&c.to_string());
            positions_last_char = 0;

            if last_char.is_lowercase() {
                positions_last_char = 1;
            }

            last_char = c;

            continue;
        }

        let current_position = exist.unwrap();
        let custom_alfaphet: String = alpabet_reverse
            .substring(current_position + 1, alpabet_reverse.len())
            .to_string();
        let mut custom_vec: Vec<char> = custom_alfaphet.chars().collect();

        while count < total_positions - 1 + positions_last_char {
            if custom_vec.len() == 0 {
                custom_vec = alpabet_reverse.chars().collect();
            }

            custom_vec.remove(0);

            count += 1;
        }

        positions_last_char = ALPHABET.find(c).unwrap() as i32;

        if custom_vec.len() > 0 {
            final_key.push_str(&custom_vec.get(0).unwrap().to_string());
        } else {
            // if doesn't exists any char in queque get first char at the alphabet
            final_key.push_str(&alpabet_reverse.chars().nth(0).unwrap().to_string());
        }
        last_char = c;
    }

    return (final_key, positions_last_char, last_char, first_char);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_positions() -> Result<(), String> {
        assert_eq!(get_total_positions("art".to_string()), 327);
        Ok(())
    }

    #[test]
    fn test_process_word() -> Result<(), String> {
        let alpabet_reverse: String = ALPHABET.chars().rev().collect::<String>();
        let total_positions = get_total_positions("art".to_string());
        assert_eq!(
            process_word(&alpabet_reverse, total_positions, "1m1", 0, '\0', '\0').0,
            "Key"
        );
        Ok(())
    }

    #[test]
    fn test_process_line() -> Result<(), String> {
        let alpabet_reverse: String = ALPHABET.chars().rev().collect::<String>();
        let total_positions = get_total_positions("Marvolo".to_string());
        assert_eq!(
            process_line(&alpabet_reverse, total_positions, "A sw Z5e9 MSVQMQW5g").trim(),
            "I Am Lord Voldemort"
        );
        Ok(())
    }

    #[test]
    fn test_process_line2() -> Result<(), String> {
        let alpabet_reverse: String = ALPHABET.chars().rev().collect::<String>();
        let total_positions = get_total_positions("Marvolo".to_string());
        assert_eq!(
            process_line(
                &alpabet_reverse,
                total_positions,
                "vBunD CuE JP TLV62b, wCvoE"
            )
            .trim(),
            "Dobby has no master, Dobby"
        );
        Ok(())
    }
}
