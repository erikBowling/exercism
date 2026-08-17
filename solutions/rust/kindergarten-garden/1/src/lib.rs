pub enum Student {
    Alice,
    Bob,
    Charlie,
    David,
    Eve,
    Fred,
    Ginny,
    Harriet,
    Ileana,
    Joseph,
    Kincaid,
    Larry,
}

impl Student {
    pub fn from_string(student: &str) -> Self {
        match student {
            "Alice" => Student::Alice,
            "Bob" => Student::Bob,
            "Charlie" => Student::Charlie,
            "David" => Student::David,
            "Eve" => Student::Eve,
            "Fred" => Student::Fred,
            "Ginny" => Student::Ginny,
            "Harriet" => Student::Harriet,
            "Ileana" => Student::Ileana,
            "Joseph" => Student::Joseph,
            "Kincaid" => Student::Kincaid,
            "Larry" => Student::Larry,
            _ => panic!("Invalid student name."),
        }
    }
}

pub enum Plants {
    Grass,
    Clover,
    Radish,
    Violet,
}

impl Plants {
    pub fn from_char(c: char) -> Self {
        match c {
            'G' => Plants::Grass,
            'C' => Plants::Clover,
            'R' => Plants::Radish,
            'V' => Plants::Violet,
            _ => panic!("Not a valid character"),
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            Plants::Grass => "grass",
            Plants::Clover => "clover",
            Plants::Radish => "radishes",
            Plants::Violet => "violets",
        }
    }
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let diagram_lines: Vec<&str> = diagram.split('\n').collect();
    let mut student_plants: Vec<&'static str> = Vec::new();

    for line in diagram_lines {
        let row: Vec<char> = line.chars().collect();
        let plants_in_row = row
            .as_chunks::<2>()
            .0
            .get(Student::from_string(student) as usize)
            .unwrap();

        for p in plants_in_row {
            let plant = Plants::from_char(*p);
            student_plants.push(plant.to_string())
        }
    }

    student_plants
}
