pub enum Bob {
    Question,
    Yell,
    YellQuestion,
    Silence,
    Whatever,
}

pub fn reply(message: &str) -> &str {
    match categorize_msg(message.trim()) {
        Bob::Question => "Sure.",
        Bob::Yell => "Whoa, chill out!",
        Bob::YellQuestion => "Calm down, I know what I'm doing!",
        Bob::Silence => "Fine. Be that way!",
        Bob::Whatever => "Whatever.",
    }
}

pub fn categorize_msg(message: &str) -> Bob {
    // Return silence if message is empty
    if message.is_empty() {
        return Bob::Silence;
    }

    let mut msg: String = message.to_string();
    let last_char: char = msg.pop().unwrap();

    // Remove all non letters
    msg.retain(|c| c.is_alphabetic());

    // Yelling
    if !msg.is_empty() && msg.chars().all(|c| c.is_uppercase()) {
        match last_char {
            '?' => return Bob::YellQuestion,
            _ => return Bob::Yell,
        }
    }

    // Regular question
    if last_char == '?' {
        return Bob::Question;
    }

    Bob::Whatever
}
