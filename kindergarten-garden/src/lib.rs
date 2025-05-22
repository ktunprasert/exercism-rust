pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let student_idx: usize = (student.as_bytes()[0] - b'A') as usize;

    diagram
        .lines()
        .flat_map(|line| line.chars().skip(student_idx * 2).take(2))
        .map(|c| -> &'static str {
            match c {
                'V' => "violets",
                'R' => "radishes",
                'C' => "clover",
                'G' => "grass",
                _ => panic!("Unknown plant"),
            }
        })
        .collect()
}
