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
    "meow", "meoww", "meowww", "meowwww", "mrow", "mroww", "mrowww", "mrowwww", "mrrp", "mrrrp",
    "mrrrrp", "mrrrrrp", "purr", "purrr", "purrrr", "purrrrr",
];

// translate a {length} bit number to its String binary representation with a length of {length} bits
fn number_to_bin(number: u8, length: usize) -> String {
    let result = format!("{number:b}");
    "0".repeat(length - result.len()) + &result
}

// translate a text to its bit representation, each character is 7 bits and their bit
// representation is their index in the BIN_TO_CHAR array
fn text_to_bin(text: &str) -> String {
    text.chars()
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
        .join("")
}

// translate cat noises to their bit representations, each noise is 4 bits and their bit
// representation is their index in the BIN_TO_CAT array
fn cat_to_bin(text: &str) -> String {
    text.split(" ")
        .filter(|x| x != &":3" && x != &":3c")
        .map(|x| BIN_TO_CAT.iter().enumerate().find(|y| y.1 == &x).unwrap())
        .map(|x| x.0)
        .map(|x| number_to_bin(x as u8, 4))
        .collect::<Vec<String>>()
        .join("")
        + &text
            .split(" ")
            .filter(|x| x == &":3" || x == &":3c")
            .map(|x| if x == ":3" { 0 } else { 1 })
            .map(|x| number_to_bin(x, 1))
            .collect::<Vec<String>>()
            .join("")
}

fn bin_to_cat(bin: &str) -> String {
    let mut result = vec![];
    for i in (0..bin.len()).filter(|x| x % 4 == 0 || bin.len() - x <= bin.len() % 4) {
        if bin.len() - i <= bin.len() % 4 {
            result.push(if bin.get(i..=i).unwrap() == "0" {
                ":3"
            } else {
                ":3c"
            })
        } else {
            let cat_bin = bin.get(i..i + 4).unwrap();
            let cat_noise = BIN_TO_CAT[usize::from_str_radix(cat_bin, 2).expect("")];
            result.push(cat_noise);
        }
    }
    result.join(" ")
}

fn bin_to_text(bin: &str) -> String {
    let mut result = vec![];
    for i in (0..bin.len())
        .filter(|x| x % 7 == 0)
        .filter(|x| bin.len() >= x + 7)
    {
        let letter_bin = bin.get(i..i + 7).unwrap();
        let letter = BIN_TO_CHAR[usize::from_str_radix(letter_bin, 2).expect("")];
        result.push(letter);
    }
    result.join("")
}

fn text_to_cat(text: &str) -> String {
    bin_to_cat(&text_to_bin(text))
}

fn cat_noises_to_text(cat_noises: &str) -> String {
    bin_to_text(&cat_to_bin(cat_noises))
}

fn main() {
    let text = "abcdefghijklmnopqrstuvwxyz";
    let cat_noise = "meow meow meow mrow meoww meow meowwww meow mrrp meoww mrow meowwww meow mrowwww meoww meow meowww mrow mroww meow mrrrrrp meoww mrrp meowwww mrow mrowwww meow purrrrr meowww meow mrow mrow mrrrp meoww meowwww meowww mrrp mroww mrow mrrrrrp meoww mrowwww meowwww meow mrowww :3 :3c";
    println!("{}", text_to_cat(text));
    println!("{}", cat_noises_to_text(cat_noise));
}
