fn main() {
    println!("Hello, world!");
}

fn parse_line(line: &str) -> (usize, usize) {
    let mut parts = line.split_whitespace();
    let x = parts.next().unwrap().parse().unwrap();
    let y = parts.next().unwrap().parse().unwrap();
    (x, y)
}

fn parse_step1(input: impl AsRef<str>) -> usize {
    let mut vec_a = Vec::new();
    let mut vec_b = Vec::new();
    for line in input.as_ref().lines() {
        if line.trim().is_empty() {
            continue;
        }
        let (x, y) = parse_line(line);
        vec_a.push(x);
        vec_b.push(y);
    }
    vec_a.sort();
    vec_b.sort();
    vec_a.iter().zip(vec_b.iter()).map(|(a, b)| b - a).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1() {
        let input = r#"
3   4
4   3
2   5
1   3
3   9
3   3
"#;

        assert_eq!(parse_step1(input), 11);
    }
}
