use scraper::selectable::Selectable;
use scraper::{Html, Selector};
use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: { }", err);
        process::exit(1);
    });

    // println!("{}", html);
    if let Err(e) = config.run() {
        println!("Application Error: {}", e);
        process::exit(1);
    }
}

struct Config {
    input: String,
    output: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Insufficient Arguments!");
        } else if args.len() > 4 {
            return Err("Too many arguments!");
        }
        Ok(Config {
            input: args[1].clone(),
            output: args[2].clone(),
        })
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        let html = fs::read_to_string(&self.input)?;
        let document = Html::parse_document(&html);

        let table_selector = Selector::parse("table").unwrap();
        let row_selector = Selector::parse("tr").unwrap();
        let col_selector = Selector::parse("th,td").unwrap();

        for table in document.select(&table_selector) {
            for row in table.select(&row_selector) {
                // let row_data:Vec<_> = Vec::new();
                let cells: Vec<_> = row
                    .select(&col_selector)
                    .map(|cell| cell.text().collect::<Vec<_>>().concat())
                    .collect();
                let cells: Vec<String> = cells
                    .iter()
                    .map(|cell| {
                        if cell.starts_with("$") {
                            let cell = cell.replace("$", "");
                            let cell = cell.replace(",", "");
                            return cell.to_owned();
                        }
                        cell.to_owned()
                    })
                    .collect();
                println!("{:#?}", cells)
            }
        }
        Ok(())
    }
}
