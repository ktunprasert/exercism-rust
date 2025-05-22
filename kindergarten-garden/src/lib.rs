pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let student_idx: usize = (student.as_bytes()[0] - 'A' as u8).into();

    let plants: Vec<char> = diagram
        .lines()
        .flat_map(|line| line.chars().skip(student_idx * 2).take(2))
        .collect();

    println!("plants: {:?}", plants);

    plants.iter().map(|c| -> &'static str {
        match c {
            'V' => "violets",
            'R' => "radishes",
            'C' => "clover",
            'G' => "grass",
            _ => panic!("Unknown plant"),
        }
    }).collect()
}
