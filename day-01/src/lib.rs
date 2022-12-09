pub fn process_part1(input: &str) -> String {
    return (input.to_owned() + ", part 1").to_string();
}

pub fn process_part2(input: &str) -> String {
    return (input.to_owned() + ", part 2").to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000
4000
5000
6000
7000
8000
9000
10000";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "24000");
    }

    #[test]
    fn part2_works() {
        // let result = process_part1(INPUT);
        // assert_eq!(result, "24000");
        assert_eq!(0, 1);
    }
}