fn btl(b: u32) -> &'static str {
    if b == 1 { return "bottle" } else { "bottles" }
}

fn bnum(count: u32) -> &'static str {
    const NUMS: [&str; 11] = [
        "No", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
    ];
    NUMS.get(count as usize)
        .copied()
        .unwrap_or_else(|| panic!("Invalid bottle count: {count}"))
}

fn verse(count: u32, remaining: u32) -> String {
    if remaining == 0 {
        return String::new();
    }

    let rep = format!("{} green {} hanging on the wall,", bnum(count), btl(count));

    let msg = format!(
        "{rep}\n{rep}\nAnd if one green bottle should accidentally fall,\nThere'll be {} green {} hanging on the wall.",
        bnum(count.saturating_sub(1)).to_lowercase(),
        btl(count.saturating_sub(1))
    );

    if remaining == 1 {
        msg
    } else {
        format!("{msg}\n\n{}", verse(count - 1, remaining - 1))
    }
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    verse(start_bottles, take_down)
}
