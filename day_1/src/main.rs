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
    0
}

#[cfg(test)]
mod tests {
    use crate::get_calories;
    #[test]
    fn test_get_calories() {
        assert_eq!(get_calories(String::from("")), 0)
    }
}
