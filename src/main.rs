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


    match &config.day[..] {
        "day1" => day1(&contents),
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

