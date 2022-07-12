use std::io::{self, BufRead};
use substring::Substring;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

fn main() {
    let alpabet_reverse: String = ALPHABET.chars().rev().collect::<String>();
    let mut secret: String = String::new();

    let mut total_positions: u32 = 0;

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

fn get_total_positions(secret: String) -> u32 {
    let mut total_positions: u32 = 0;
    for c in secret.chars() {
        total_positions += c as u32;
    }
    return total_positions;
}

fn process_line(alpabet_reverse: &str, total_positions: u32, text_to_decode: &str) -> String {
    let mut final_key: String = String::new();

    for word in text_to_decode.split(" ") {
        final_key.push_str(&process_word(alpabet_reverse, total_positions, word));
        final_key.push(' ');
    }

    return final_key;
}

fn process_word(alpabet_reverse: &str, total_positions: u32, word: &str) -> String {
    let mut final_key: String = String::new();
    let mut positions_last_char: u32 = 0;

    for c in word.chars() {
        let mut count: u32 = 0;

        let exist = alpabet_reverse.find(c);

        if exist == None {
            final_key.push_str(&c.to_string());
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

        positions_last_char = ALPHABET.find(c).unwrap() as u32;

        if custom_vec.len() > 0 {
            final_key.push_str(&custom_vec.get(0).unwrap().to_string());
        } else {
            // if doesn't exists any char in queque get first char at the alphabet
            final_key.push_str(&alpabet_reverse.chars().nth(0).unwrap().to_string());
        }
    }

    return final_key;
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
            process_word(&alpabet_reverse, total_positions, "1m1"),
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
            "I Am 7ord Uoldemort"
        );
        Ok(())
    }
}
