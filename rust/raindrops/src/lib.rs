fn is_factor(n: u32, factor: u32) -> bool {
    n % factor == 0
}

pub fn raindrops(n: u32) -> String {
    let mut sound = String::new();

    if is_factor(n, 3) {
        sound.push_str("Pling");
    }

    if is_factor(n, 5) {
        sound.push_str("Plang");
    }

    if is_factor(n, 7) {
        sound.push_str("Plong");
    }

    if !sound.is_empty() {
        sound
    } else {
        n.to_string()
    }
}
