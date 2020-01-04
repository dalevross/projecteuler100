
extern crate reqwest;
extern crate select;
extern crate clap;


use clap::{Arg, App};
use select::document::Document;
use select::predicate::Name;

pub struct EulerProblem {
    title: String,
    Description: String,
}

fn main() -> Result<(), ()> {

    let mut problem_num : u32 = 0;

    let matches = App::new("Get Project Euler Problem")
    .version("0.1.0")
    .author("Dale V. Ross <dalevross@gmail.com>")
    .about("Gets a Euler problem by Id and creates Rust files")
    .arg(Arg::with_name("num")
             .short("n")
             .long("number")
             .takes_value(true)
             .help("The Euler problem number"))
    .get_matches();

    let num_str = matches.value_of("num");
    match num_str {
        None => println!("Please pass a valid problem number."),
        Some(s) => {
            match s.parse::<u32>() {
                Ok(n) => problem_num = n,
                Err(_) => println!("That's not a number! {}", s),
            }
        }
    }

    let most_recent : Result<i32> = get_most_recent()?;

    if problem_num > most_recent
    {
        println!("{} is not a valid problem", problem_num);
    }

    println!("Most recent: {}",most_recent);
    generate_problems(problem_num)?;
}


fn generate_problems(problem_num: i32) -> Result<(),()>
{
    let url = format!("https://projecteuler.net/problem={}",problem_num);
    let res = reqwest::get(&url)?;

    Document::from_read(res)?
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));

    Ok(())
}

fn get_most_recent() -> i32
{
    let most_recent :i32 = 0;
    let url = format!("https://projecteuler.net/progress");
    let res = reqwest::get(&url)?;

    let id = Document::from_read(res)?
        .find(Attr("id","id_column")).next().text;

    let test = id.parse::<u32>();

    match test {
        Ok(ok) => most_recent = ok,
        Err(e) => println!("Invalid problem number ({})", e), 
    }
    most_recent  
}