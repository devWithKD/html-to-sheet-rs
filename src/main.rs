use html_to_sheets_rust::{
    EXPECTED_SHEET1, EXPECTED_SHEET2, EXPECTED_SHEET3, EXPECTED_SHEET4, EXPECTED_SHEET5,
};
use scraper::selectable::Selectable;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::{env, error::Error, fmt, fs, process};

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

#[derive(Serialize, Deserialize, Debug, Default)]
struct Form {
    sheet1: Vec<(String, String)>,
    sheet2: Vec<(String, String)>,
    sheet3: Vec<(String, String)>,
    sheet4: Vec<(String, String)>,
    sheet5: Vec<(String, String)>,
}

#[derive(Debug)]
struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for CustomError {}

#[derive(Default, Debug)]
struct Data {
    sheet1: Vec<Vec<String>>,
    sheet2: Vec<Vec<String>>,
    sheet3: Vec<Vec<String>>,
    sheet4: Vec<Vec<String>>,
    sheet5: Vec<Vec<String>>,
}

struct Config {
    input: String,
    // output: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("Insufficient Arguments!");
        } else if args.len() > 4 {
            return Err("Too many arguments!");
        }
        Ok(Config {
            input: args[1].clone(),
            // output: args[2].clone(),
        })
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        let html = fs::read_to_string(&self.input)?;
        let document = Html::parse_document(&html);

        let table_selector = Selector::parse("table").unwrap();
        let row_selector = Selector::parse("tr").unwrap();
        let col_selector = Selector::parse("th,td").unwrap();

        let mut data = Data::default();

        for (sheet_num, table) in document.select(&table_selector).enumerate() {
            let sheet_ref = match sheet_num {
                0 => &mut data.sheet1,
                1 => &mut data.sheet2,
                2 => &mut data.sheet3,
                3 => &mut data.sheet4,
                4 => &mut data.sheet5,
                _ => return Err(Box::new(CustomError("Sheet Limit Exceeded".into()))),
            };

            for row in table.select(&row_selector) {
                let cells: Vec<String> = row
                    .select(&col_selector)
                    .map(|cell| cell.text().collect::<Vec<_>>().concat().trim().to_owned())
                    .collect();
                sheet_ref.push(cells);
            }
        }
        println!("{:#?}", data);

        Ok(())
    }
}
