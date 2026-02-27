

use std::{fs, error::Error, env};


pub struct Args {
    pub query:String,
     pub path:String,
    pub ignore_case: bool,
 }


impl Args {
    pub fn new(args:&[String]) -> Result <Args, &'static str> {

        if args.len() < 3 { return Err("String is less then 3 idiot!")}
        let query = args[1].clone();
        let path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Args{query, path, ignore_case})
    }
 pub fn run(cfg:&Args)-> Result<(),Box<dyn Error>>{
        let contents = fs::read_to_string(&cfg.path)?;
        println!("\nLooking for '{}' in this File: '{}'\n", cfg.query,cfg.path);
        let results = if cfg.ignore_case {
            search_case_insensitive(&cfg.query,&contents)
        }else{
            search(&cfg.query,&contents)
        };
        for (i,line) in results {
            println!("\nFile-Line[{i}] : {line}\n");
        }
        Ok(())

    }
}
    
pub fn search<'a>(_query:&str,_contents:&'a str) -> Vec<(i32,&'a str)>{
    let mut lines = Vec::new();
    for (i,line) in _contents.lines().enumerate(){
         if line.contains(_query) {
             lines.push((i as i32 +1, line));
         }
    }
    lines
}
pub fn search_case_insensitive<'a>(_query:&str,_contents:&'a str) -> Vec<(i32,&'a str)>{
   let query = _query.to_lowercase();
    let mut lines = Vec::new();
    for (i,line) in _contents.lines().enumerate(){
         if line.to_lowercase().contains(&query) {
             lines.push((i as i32 +1, line));
         }
    }
    lines
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec![(2,"safe, fast, productive.")], search(query, contents))
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec![(1,"Rust:"),(4,"Trust me.")], search_case_insensitive(query, contents))
    }


}

