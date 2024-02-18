use std::error::Error;
use std::fs::read_to_string;

pub struct Grep {
    pub search_string: String,
    pub filename: String,
}

impl Grep {
    pub fn new(args: &Vec<String>) -> Result<Grep, String> {
        if args.len() < 2 {
            return Err(String::from("No Search Term provided."));
        }

        if args.len() == 2 {
            return Err(String::from("No Search File provided"));
        }
        return Ok(Grep {
            search_string: args[1].clone(),
            filename: args[2].clone(),
        });
    }
}

pub fn read(file: String) -> Result<String, Box<dyn Error>> {
    let file = read_to_string(file)?;
    Ok(file)
}

pub fn closure<F>(f: F) -> ()
where
    F: FnOnce() -> (),
{
    f();
    ()
}

pub fn search<'a>(text: &'a str, find: &str, flags: &Vec<Flags>) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    let mut ignore_case = false;
    for line in text.lines() {
        for flag in flags {
            if matches!(flag, Flags::I) {
                if line.to_lowercase().contains(&find.to_lowercase()) {
                    result.push(line);
                    ignore_case = true;
                }
            }
        }
        if !ignore_case {
            if line.contains(&find) {
                result.push(line);
            }
        }
    }
    result
}

#[derive(PartialEq)]
pub enum Flags {
    I,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arg_parse() {
        assert_eq!(
            Grep::new(&vec![
                String::from("x"),
                String::from("1"),
                String::from("test.txt")
            ])
            .expect("dsnkjanasd")
            .filename,
            String::from("test.txt")
        )
    }
    #[test]
    fn test_search() {
        let text = "Hello World\nHow You Doing.\n";
        assert_eq!(search(text, "W").len(), 1);
    }
}
