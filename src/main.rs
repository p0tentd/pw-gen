use rand::Rng;

fn get_password_length() -> usize {
    loop {
        println!("password length (max 64 min 8):");
        let mut password_length = String::new();
        std::io::stdin().read_line(&mut password_length).unwrap();
        let password_length: usize = match password_length.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        if password_length >= 8 && password_length <= 64 {
            break password_length;
        }
    }
}

fn main() {
    let password_length = get_password_length();
    let mut rng = rand::thread_rng();
    let password: String = (0..password_length)
        .map(|_| {
            let mut char_set = vec![
                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
                'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
                'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
                '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
                '!', '@', '#', '$', '%', '^', '&', '*', '-', '+', '_', '=', '<', '>', '?',
            ];
            let char_idx = rng.gen_range(0..char_set.len());
            let char = char_set.swap_remove(char_idx);
            char
        })
        .collect();

    println!("{}", password);
}