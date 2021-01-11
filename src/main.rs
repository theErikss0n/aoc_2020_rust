use std::env;
use std::process;
use std::error::Error;
use std::fs;

struct Config {
    day: String,
    filename: String,
}

impl Config {
    fn new (mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let day = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't get a day"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("didnt get a filename"),
        };

        Ok(Config {
            day,
            filename,
        })
    }
}

fn main() {
    let args = env::args();
    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

fn run(config: Config) ->  Result<(), Box<dyn Error>> {
    let filepath = format!("./Files/{}/{}",config.day, config.filename);

    let contents = fs::read_to_string(filepath)?;


    match config.day.as_str() {
        "day1" => day1(&contents),
        "day2" => day2(&contents),
        _ => (),
    }


    Ok(())
}

fn day1(contents: &str) {
    let mut result = 0;
    for line in contents.lines() {
        for line1 in contents.lines() {
            let line_as_int = line.parse::<i32>().unwrap();
            let line1_as_int = line1.parse::<i32>().unwrap();

            if line_as_int + line1_as_int == 2020 {
                //println!{"{}, {}", line_as_int, line1_as_int}
                result = line_as_int * line1_as_int
            }
        }
    }

    println!{"{}", result}


    let mut result1 = 0;
    for line in contents.lines() {
        for line1 in contents.lines() {
            for line2 in contents.lines() {
                let line_as_int = line.parse::<i32>().unwrap();
                let line1_as_int = line1.parse::<i32>().unwrap();
                let line2_as_int = line2.parse::<i32>().unwrap();

                if line_as_int + line1_as_int + line2_as_int== 2020 {
                    result1 = line_as_int * line1_as_int * line2_as_int
                }
            }
        }
    }

    println!{"{}", result1}

}

fn day2(contents: &str) {
    #[derive(Debug)]
    struct Entry<'a>{
        policy: (u32, u32),
        letter: char,
        password: &'a str,
    }

    impl Entry<'_> {
        fn validate_password(&self) -> bool {
            let count = self.password.matches(|ch| ch == self.letter).count();
            //println!("{}", count);

            let count = count as u32;
            self.policy.0 <= count && count <= self.policy.1 
        }
    }

    let mut count = 0;
    for line in contents.lines() {
        let mut iter = line.split_ascii_whitespace();

        let mut policy: (u32, u32)  = (0,0);

        if let Some(first) = iter.next() {
            let mut char_iter = first.split("-");
            // println!("{}", first);

            let lower_bound =  char_iter.next().expect("no lower bound");
            let upper_bound = char_iter.next().expect("no upper bound");
            
            // println!("{}-{}", upper_bound, lower_bound);

            policy = (lower_bound.parse().unwrap(), upper_bound.parse().unwrap());
        };

        let mut letter: char = ' ';
        if let Some(given_letter) = iter.next() {
            let mut char_iter = given_letter.chars();
            letter = char_iter.next().expect("no given char");
        }

        let mut password: &str = " ";
        if let Some(pw) = iter.next() {
            password = pw;
        }

        let entry = Entry {
            policy,
            letter,
            password,
        };

        let result = entry.validate_password();

        //println!("{}", result);

        if result {
            count = count + 1;
        }
    }
    println!("{}", count)

}