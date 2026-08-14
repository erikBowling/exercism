pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut song: String = String::new();
    for i in ((start_bottles - take_down + 1)..start_bottles + 1).rev() {
        let first = format!(
            "{} green {} hanging on the wall,\n",
            num_word(i),
            bottle_str(i)
        );
        let third = "And if one green bottle should accidentally fall,\n";
        let fourth = format!(
            "There'll be {} green {} hanging on the wall.\n",
            num_word(i - 1).to_lowercase(),
            bottle_str(i - 1)
        );

        song.push_str(first.as_str());
        song.push_str(first.as_str());
        song.push_str(third);
        song.push_str(fourth.as_str());
        song.push('\n');
    }

    song
}

pub fn num_word(digit: u32) -> String {
    match digit {
        0 => "no".into(),
        1 => "One".into(),
        2 => "Two".into(),
        3 => "Three".into(),
        4 => "Four".into(),
        5 => "Five".into(),
        6 => "Six".into(),
        7 => "Seven".into(),
        8 => "Eight".into(),
        9 => "Nine".into(),
        10 => "Ten".into(),
        _ => "Not Allowed".into(),
    }
}

pub fn bottle_str(digit: u32) -> String {
    match digit {
        1 => "bottle".into(),
        _ => "bottles".into(),
    }
}
