const OPENERS: [char; 3] = ['(', '{', '['];
const CLOSERS: [char; 3] = [')', '}', ']'];

pub fn brackets_are_balanced(string: &str) -> bool {
    // 1. iterate through characters.
    // 2. if char is a bracket, push it to a stack
    // 3. if closing bracket, pop stack.
    // 4. if popped bracket is pair of current bracket, keep going
    // 5. if not, return false
    // 6. if string empty and stack empty, return true

    let mut stack: Vec<char> = Vec::new();

    for c in string.chars() {
        if OPENERS.contains(&c) {
            stack.push(c);
        }

        if CLOSERS.contains(&c) {
            let Some(b) = stack.pop() else { return false };
            match b {
                '(' => {
                    if c != ')' {
                        return false;
                    }
                }
                '{' => {
                    if c != '}' {
                        return false;
                    }
                }
                '[' => {
                    if c != ']' {
                        return false;
                    }
                }
                // Should never get here, but compiler angry
                _ => return false,
            }
        }
    }

    // Meaning, there were more openings than closing brackets
    if stack.len() > 0 {
        return false;
    }

    true
}
