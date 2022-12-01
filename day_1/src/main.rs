use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("inputs/demo")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    print!("{}",contents);
    Ok(())
}

fn get_calories(input: String) -> i32 {
    if input == String::from("") {
        return 0;
    }
    input.parse::<i32>().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::get_calories;
    #[test]
    fn test_get_calories_no_input() {
        assert_eq!(get_calories(String::from("")), 0)
    }

    #[test]
    fn test_get_calories_single_number() {
        assert_eq!(get_calories(String::from("1000")), 1000)
    }
}
