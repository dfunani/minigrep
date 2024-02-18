use minigrep::read;
use minigrep::search;
use minigrep::closure;
use minigrep::Flags;
use minigrep::Grep;
use std::env::args;
use std::env::var;
use std::error::Error;

/// Entry Point
/// ```let x = 5;```
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().collect();
    // let grep = match Grep::new(&args){
    //     Ok(t) => t,
    //     Err(e) => panic!("{e}")
    // };

    let grep = Grep::new(&args).expect("GrepError");

    // let file2 = read_to_string(&args[3]).expect("No file to read");

    let text = match read(grep.filename) {
        Ok(t) => t,
        Err(_) => panic!("No File to read."),
    };

    let mut flags = Vec::new();
    let case = var("LOWER_CASE").is_ok_and(|x| x == "true");
    
    if case {
        flags.push(Flags::I);
    }
    let res = search(&text, &grep.search_string, &flags);
    for t in res {
        println!("{t}");
    }
    closure(|| println!("Hello World"));
    Ok(())
}
