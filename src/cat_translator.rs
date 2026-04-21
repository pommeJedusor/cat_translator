const BIN_TO_CHAR: [&str; 128] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "x", "y", "z", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "=",
    "[", "]", ";", "'", "#", "|", ",", ".", "/", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ",
    " ", " ", " ", " ", " ", " ", " ", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L",
    "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "!", "€", "£", "$", "%",
    "^", "&", "*", "(", ")", "_", "+", "{", "}", ":", "@", "~", "|", "<", ">", "?", ")", "\"", " ",
    " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", "", "",
];

const BIN_TO_CAT: [&str; 16] = [
    "meow", "meoww", "meowww", "meowwww", "mrow", "mroww", "mrowww", "mrowwww", "mrp", "mrrp",
    "mrrrp", "mrrrrp", "purr", "purrr", "purrrr", "purrrrr",
];

// translate a {length} bit number to its String binary representation with a length of {length} bits
fn number_to_bin(number: u8, length: usize) -> String {
    let result = format!("{number:b}");
    "0".repeat(length - result.len()) + &result
}

fn cat_noise_to_bin(cat_noise: &str) -> usize {
    assert_ne!(cat_noise.len(), 0);

    let first_letter = cat_noise.get(0..1).unwrap();
    let is_purr = first_letter == "p";
    let is_meow = first_letter == "m" && cat_noise.contains("e");
    let is_mrp = first_letter == "m" && !cat_noise.contains("o");
    let is_mrow = !(is_purr || is_meow || is_mrp);

    let default_cat_noise_length = if is_mrp {3} else {4};

    // length value min bound to default_size and max to default size + 3
    let len = usize::max(cat_noise.len(), default_cat_noise_length);
    let len = usize::min(len, default_cat_noise_length + 3);

    if is_purr {
        return 12 | (len - 4);
    } else if is_mrp {
        return 8 | (len - 3);
    } else if is_mrow {
        return 4 | (len - 4);
    } else if is_meow {
        return len - 4;
    } 

    unreachable!()
}

// translate cat noises to their bit representations, each noise is 4 bits and their bit
// representation is their index in the BIN_TO_CAT array
fn cat_noises_to_bin(text: &str) -> String {
    text.split(" ")
        .map(|x| match x == ":3" || x == ":3c" {
            true => number_to_bin(if x == ":3" { 0 } else { 1 }, 1),
            false => number_to_bin(cat_noise_to_bin(x) as u8, 4)
        })
        .collect::<Vec<String>>()
        .join("")
}

fn bin_to_cat_noises(bin: &str) -> String {
    let offset_bits_index = bin.len() - (bin.len() % 4);

    let main_cat_noises = (0..offset_bits_index)
        .step_by(4)
        .map(|i| bin.get(i..i + 4).unwrap())
        .map(|cat_bin| BIN_TO_CAT[usize::from_str_radix(cat_bin, 2).unwrap()]);
    let offset_cat_noises = (offset_bits_index..bin.len())
        .map(|i| bin.get(i..=i).unwrap())
        .map(|cat_bin| match cat_bin {
            "0" => ":3",
            "1" => ":3c",
            _ => unreachable!(),
        });
    let cat_noises = main_cat_noises.chain(offset_cat_noises);

    cat_noises.collect::<Vec<&str>>().join(" ")
}

// translate a text to its bit representation, each character is 7 bits and their bit
// representation is their index in the BIN_TO_CHAR array
fn text_to_bin(text: &str) -> Result<String, String> {
    let unvalid_characters = text
        .chars()
        .filter(|x| BIN_TO_CHAR.iter().find(|y| **y == &x.to_string()).is_none())
        .collect::<Vec<char>>();

    if unvalid_characters.len() != 0 {
        return Err(format!(
            "{} is not a valid character\nhere is a list of all valid characters: \nabcdefghijklmnopqrstuvwxyz1234567890-=[];'#|,./ ABCDEFGHIJKLMNOPQRSTUVWXYZ!€£$%^&*()_+{}:@~|<>?)\"",
            unvalid_characters[0], "{}"
        ));
    }

    Ok(text
        .chars()
        .map(|x| {
            BIN_TO_CHAR
                .iter()
                .enumerate()
                .find(|y| y.1 == &x.to_string())
                .map(|x| x.0)
                .unwrap()
        })
        .map(|x| number_to_bin(x as u8, 7))
        .collect::<Vec<String>>()
        .join(""))
}

fn bin_to_text(bin: &str) -> String {
    (0..bin.len())
        .step_by(7)
        .filter(|x| bin.len() >= x + 7) // in case the number of bit is not a multiple of 7
        .map(|i| bin.get(i..i + 7).unwrap())
        .map(|letter_bin| BIN_TO_CHAR[usize::from_str_radix(letter_bin, 2).unwrap()])
        .collect::<Vec<&str>>()
        .join("")
}

pub fn text_to_cat(text: &str) -> Result<String, String> {
    match text_to_bin(text) {
        Ok(bin) => Ok(bin_to_cat_noises(&bin)),
        Err(e) => Err(e),
    }
}

pub fn cat_noises_to_text(cat_noises: &str) -> String {
    bin_to_text(&cat_noises_to_bin(cat_noises))
}


#[cfg(test)]
mod tests {
    use super::*;
    const INPUT_TEXT_TEST: &str = "the incredible cat encoding system was created by Freya (thenonymous), this rust client and its implementation of it is made by pomme jkqvxz1234567890-=[];'#|./ABCDEGHIJKLMNOPQRSTUVWXYZ!€£$%^&*_+{}:@~|<>?\"";
    const INPUT_CAT_NOISES_TEST: &str = "meowww mrowww meoww purr meowww meowww purrrrr meoww meow meowwww mrow meoww meoww meoww meow mrp meow purr mrow meow meoww meoww mrowww meoww meoww mrowwww mrp meowww meow meow mrow purrr mrowwww mrp mrow meoww mrrrp meow mrp mrowwww meow meowwww meoww meow meowwww mrow meowwww meowww purrrrr meowww mrow mrowww meow mrrp meoww meowwww meow mrp meowwww meoww mrowwww mrrp mrowww meow meow mrow mrrp mrowwww mrp meowww meowww meowww meoww meow meow meoww meowwww meow mrp meow purrr mrowwww mrp meoww meowwww meow mrrrrp purrrr meowww mrrp meoww meow mrp mrowww meow meow meowww purrrrr purr mrow mrow purr meowwww mrp mrow meoww mrrrp meowwww mrp mrowww mrrp mrp meoww mrp meowwww mrp mrrrp meoww meowww purr mrowww mrrrrp meoww mrowwww mrrp meowwww meow purrrr meowww meow mrrp meowww purrrrr meowww meowww mroww meow mrrp meoww meowwww mroww purrrr meow mrp mroww mrp mrp meow mrp meowwww mrow mrrp mrrrp purrrrr meow meow meowwww mrow meoww mrrrp purrrrr meoww meow mrow purr mrrp meowww purrrrr meoww meow meowwww meow mrowwww mrp mrrrrp meow mrp meowwww meow meowww meow purrr meowww mrowww meow meow mrrp mrp mrp meoww purr meowwww mroww mrowwww mrp purrrr meow mrrrp mrrrrp purr mrow meoww meowwww mroww purrrr meowww meow mrrp meowww purrrrr meoww mrp meow meow meoww mrp mrow mroww purrrr meow mrow purr meowww purrrrr meoww purrrr meowwww mrp mrowww meow purr meow mrp mrrrrp purr mrow mrp mrrrp meowww meow mroww mrow mrrrrp mrrp mrrp meowwww mrow mrowww purr purrrr meoww purrr meowwww purr mrowwww purrr meow meowww meoww mrow mrow mrp purrr meowww meowww mroww mrow purr mrrp purrr mrow meowww mrrp mroww mrow mrrrp purrr mrowww mrrrp purrrr mrp meoww meow mrowww meoww mrow meowwww mrp mrrp meoww mrrrp meowwww purr mrp mrrp meowwww meowww mrrrp mroww purr purr mrrp mrrrrp meowwww mrrrp mrowwww purrr meow mrrrp meowwww mrow mrrrp mrrp purrr mrow mrrrp mrrrrp mroww mrrrp mrrrrp purrr mrp mrrrrp meowwww mrowww mrrrp purrr purrr purr mrrrrp mrrrrp mrowwww mrrrp purrrrr purrrr meow purr meowwww mrrp meowwww meowww purrrr mrowww purr purrrrr mrrrp meowwww mrow purrrr mrrrp mroww mrowwww mrrrrp meowwww mrowww purrrr purrrr purrrr :3 :3 :3";

    #[test]
    fn cat_to_text_test() {
        assert_eq!(cat_noises_to_text(INPUT_CAT_NOISES_TEST), INPUT_TEXT_TEST);
    }

    #[test]
    fn text_to_cat_test() {
        assert_eq!(&text_to_cat(INPUT_TEXT_TEST).unwrap(), INPUT_CAT_NOISES_TEST);
    }
}
