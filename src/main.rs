use scraper::selectable::Selectable;
use scraper::{Html, Selector};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let output_file = &args[2];
    let html = fs::read_to_string(input_file).expect("Something went wrong!");
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
            println!("{:#?}", cells)
        }
    }

    // println!("{}", html);
}
