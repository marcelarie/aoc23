fn main() {
    let input = include_str!("./in-1.txt");
    println!("{}", part1(input));
}

fn part1(input: &str) -> String {
    let mut sum = 0;
    input.lines().for_each(|line| {
        let mut first = 0;
        let mut last = 0;
        for char in line.chars() {
            if let Ok(char) = char.to_string().parse::<i32>() {
                first = char;
                break;
            }
        }
        for char in line.chars().rev() {
            if let Ok(char) = char.to_string().parse::<i32>() {
                last = char;
                break;
            }
        }
        sum += format!("{}{}", first, last).parse::<i32>().unwrap();
    });

    return sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = include_str!("./in-test-1.txt");
        let result = part1(test_input);
        assert_eq!(result, "142".to_string());
    }
}
