pub const DIRECTIONS_2D_ARRAY: [(isize, isize); 8] = [
    (0, 1),   // down
    (0, -1),  // up
    (1, 0),   // right
    (-1, 0),  //left
    (1, 1),   // down-right
    (1, -1),  // up-right
    (-1, 1),  // down-left
    (-1, -1), // up-left
];

// " " => 32
// * => 42
pub const SPACE_CHAR_VAL: u8 = 32;
pub const STAR_CHAR_VAL: u8 = 42;

pub fn annotate(garden: &[&str]) -> Vec<String> {
    // Setup byte "garden"
    let mut byte_garden: Vec<Vec<u8>> = Vec::new();

    for row in garden {
        let mut r: Vec<u8> = Vec::new();
        for c in row.as_bytes() {
            r.push(*c);
        }

        byte_garden.push(r);
    }

    // Process byte garden
    let mut result: Vec<String> = Vec::new();

    for (r, row) in byte_garden.iter().enumerate() {
        let mut row_str = String::new();
        for (c, spot) in row.iter().enumerate() {
            // If flower then push and move on
            if *spot == STAR_CHAR_VAL {
                row_str.push('*');
                continue;
            }

            // If empty space, count surrounding flowers
            let mut flower_count = 0;
            for (x, y) in DIRECTIONS_2D_ARRAY {
                // Check if valid usize (index)
                // Casting negative isize values using "as" causes wraparound and can introduce bugs
                // This method is more verbose but ensures the intended index
                let xr: Result<usize, _> = (x + r as isize).try_into(); // row
                let yc: Result<usize, _> = (y + c as isize).try_into(); // column

                let mut is_valid_index = true;

                match xr {
                    Ok(_) => {}
                    Err(_) => {
                        is_valid_index = false;
                    }
                }

                match yc {
                    Ok(_) => {}
                    Err(_) => {
                        is_valid_index = false;
                    }
                }

                if !is_valid_index {
                    continue;
                }

                // Valid index
                if let Some(cur_row) = byte_garden.get(xr.unwrap())
                    && let Some(cur_col) = cur_row.get(yc.unwrap())
                    && *cur_col == STAR_CHAR_VAL
                {
                    flower_count += 1;
                }
            }
            if flower_count > 0 {
                row_str.push_str(&flower_count.to_string());
            } else {
                row_str.push(' ');
            }
        }
        result.push(row_str);
    }

    result
}
