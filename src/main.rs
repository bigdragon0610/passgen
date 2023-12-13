use rand::{seq::SliceRandom, Rng};

const LOWERCASE_LETTERS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPERCASE_LETTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &[u8] = b"0123456789";
const SPECIAL_LETTERS: &[u8] = b"!@#$%^&*()-+";
const LETTERS: [&[u8]; 4] = [
    LOWERCASE_LETTERS,
    UPPERCASE_LETTERS,
    NUMBERS,
    SPECIAL_LETTERS,
];
const PASSWORD_LENGTH: usize = 12;

fn main() {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    for letter in LETTERS {
        password.push(letter[rng.gen_range(0..letter.len())]);
    }
    while password.len() < PASSWORD_LENGTH {
        let letter = LETTERS[rng.gen_range(0..LETTERS.len())];
        password.push(letter[rng.gen_range(0..letter.len())]);
    }
    password.shuffle(&mut rng);
    print!(
        "{}",
        password.iter().map(|c| *c as char).collect::<String>()
    );
}
