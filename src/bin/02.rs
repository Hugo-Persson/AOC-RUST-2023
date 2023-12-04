advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    for ele in input.split('\n') {
        let chunks: Vec<&str> = ele.split(" ").collect();
        if chunks.len() == 0 {
            break;
        }
        let id: i32 = chunks[1].trim_end_matches(':').parse().unwrap();
        let mut i = 2;
        while i < chunks.len(){
            let count: i32 = chunks[i].parse().unwrap();
            i+=1;
            let color = chunks[++i].trim_end_matches(',').trim_end_matches(';');
        }
        println!("{}", id);
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
