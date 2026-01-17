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

fn char_to_bin(char: &str) -> usize {
    BIN_TO_CHAR
        .iter()
        .enumerate()
        .find(|x| x.1 == &char)
        .map(|x| x.0)
        .unwrap()
}

fn cat_noise_to_bin(cat_noise: &str) -> (u8, u8) {
    if cat_noise == ":3" {
        return (0, 1);
    }
    if cat_noise == ":3c" {
        return (1, 1);
    }
    // if purr
    if cat_noise.get(0..1).unwrap() == "p" {
        // length min bound to 4
        let len = usize::max(cat_noise.len(), 4);
        // length max bound to 7
        let len = usize::min(len, 7);
        return (12 | (len - 4) as u8, 4);
        // if meow
    } else if cat_noise.get(0..1).unwrap() == "m" && cat_noise.contains("e") {
        // length min bound to 4
        let len = usize::max(cat_noise.len(), 4);
        // length max bound to 7
        let len = usize::min(len, 7);
        return ((len - 4) as u8, 4);
        // if mrrp
    } else if cat_noise.get(0..1).unwrap() == "m" && !cat_noise.contains("o") {
        // length min bound to 4
        let len = usize::max(cat_noise.len(), 4);
        // length max bound to 7
        let len = usize::min(len, 7);
        return (8 | (len - 4) as u8, 4);
        // if mrow
    } else {
        // length min bound to 4
        let len = usize::max(cat_noise.len(), 4);
        // length max bound to 7
        let len = usize::min(len, 7);
        return (4 | (len - 4) as u8, 4);
    }
}

fn cat_to_text(cat: &str) -> String {
    let cat_vec = cat
        .split(' ')
        .map(|x| cat_noise_to_bin(x))
        .collect::<Vec<(u8, u8)>>();
    let mut result = String::new();
    let mut temp_bin = 0;
    let mut temp_bin_length = 0;
    for (noise, length) in cat_vec {
        if length < 7 - temp_bin_length {
            temp_bin |= noise << (7 - temp_bin_length - length);
            temp_bin_length += length;
        } else if length + temp_bin_length == 7 {
            temp_bin |= noise;
            result.push_str(BIN_TO_CHAR[temp_bin as usize]);
            temp_bin = 0;
            temp_bin_length = 0;
        } else {
            let length = length - (7 - temp_bin_length);
            temp_bin |= noise >> length;
            let noise = noise & ((1 << length) - 1);
            result.push_str(BIN_TO_CHAR[temp_bin as usize]);

            temp_bin_length = 0;
            temp_bin = 0;
            temp_bin |= noise << (7 - temp_bin_length - length);
            temp_bin_length += length;
        }
    }
    result
}

fn text_to_cat(text: &str) -> String {
    let mut result = vec![];
    let bins = text
        .chars()
        .map(|x| char_to_bin(&x.to_string()))
        .collect::<Vec<usize>>();
    let mut temp_bin = 0;
    let mut temp_bin_length = 0;
    for bin in bins {
        let mut bin = bin;
        let mut bin_length = 7;
        result.push(
            BIN_TO_CAT[temp_bin | ((bin >> (7 - (bin_length - 3 - temp_bin_length))) & 0b1111)],
        );
        bin_length -= 4 - temp_bin_length;
        bin &= (1 << bin_length) - 1;
        temp_bin = 0;
        temp_bin_length = 0;

        if bin_length > 4 {
            result.push(BIN_TO_CAT[bin >> (bin_length - 4)]);
            bin_length -= 4;
            bin &= (1 << bin_length) - 1;
        }
        if bin_length <= 3 {
            temp_bin = bin << (4 - bin_length);
            temp_bin_length = bin_length;
        } else if bin_length == 4 {
            result.push(BIN_TO_CAT[bin]);
        }
    }
    for i in (0..temp_bin_length).rev() {
        if (temp_bin >> (i + temp_bin_length)) & 1 != 0 {
            result.push(":3c")
        } else {
            result.push(":3")
        }
    }
    result.join(" ")
}

fn main() {
    let result = text_to_cat("abcdefghijklmnopqrstuvwxyz");
    println!("{}", result);
    let result = cat_to_text(&result);
    println!("{}", result);
}
