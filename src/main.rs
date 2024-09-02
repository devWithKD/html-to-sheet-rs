use html_to_json::{
    EXPECTED_SHEET1, EXPECTED_SHEET2, EXPECTED_SHEET3, EXPECTED_SHEET4, EXPECTED_SHEET5,
};
use scraper::selectable::Selectable;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::{self, json};
use std::{
    collections::{HashMap, HashSet},
    env,
    error::Error,
    fmt, fs, process,
};

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

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
struct DataStruct {
    sheet1: Vec<(String, usize)>,
    sheet2: Vec<(String, usize)>,
    sheet3: Vec<(String, usize)>,
    sheet4: Vec<(String, usize)>,
    sheet5: Vec<(String, usize)>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Data {
    sheet1: Vec<Vec<String>>,
    sheet2: Vec<Vec<String>>,
    sheet3: Vec<Vec<String>>,
    sheet4: Vec<Vec<String>>,
    sheet5: Vec<Vec<String>>,
}

impl Data {
    fn remove_duplicate_columns(&mut self) {
        self.sheet1 = Self::filter_columns(&self.sheet1);
        self.sheet2 = Self::filter_columns(&self.sheet2);
        self.sheet3 = Self::filter_columns(&self.sheet3);
        self.sheet4 = Self::filter_columns(&self.sheet4);
        self.sheet5 = Self::filter_columns(&self.sheet5);
    }

    fn filter_columns(sheet: &Vec<Vec<String>>) -> Vec<Vec<String>> {
        if sheet.is_empty() || sheet[0].is_empty() {
            return sheet.clone();
        }

        let mut unique_columns: Vec<Vec<String>> = Vec::new();
        let mut seen: HashSet<Vec<String>> = HashSet::new();

        for col_idx in 0..sheet[0].len() {
            let mut column: Vec<String> = Vec::new();
            for row in sheet {
                column.push(row[col_idx].clone());
            }

            if !seen.contains(&column) {
                seen.insert(column.clone());
                unique_columns.push(column);
            }
        }

        // Reconstruct the sheet with unique columns
        let mut filtered_sheet = Vec::new();
        for row_idx in 0..sheet.len() {
            let mut new_row = Vec::new();
            for column in &unique_columns {
                new_row.push(column[row_idx].clone());
            }
            filtered_sheet.push(new_row);
        }

        filtered_sheet
    }
}

#[derive(PartialEq)]
enum OutputOpt {
    Vector,
    Hashmap,
}

struct Config {
    input: String,
    output: String,
    option: OutputOpt,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Insufficient Arguments!");
        } else if args.len() > 5 {
            return Err("Too many arguments!");
        }
        let opt = args.get(3);
        let opt = match opt {
            Some(str) => {
                if str == "-v" {
                    OutputOpt::Vector
                } else {
                    OutputOpt::Hashmap
                }
            }
            None => OutputOpt::Vector,
        };
        Ok(Config {
            input: args[1].clone(),
            output: args[2].clone(),
            option: opt,
        })
    }

    // fn get_index_of_element(vec: &mut Vec<(String, bool)>, element: &str) -> Option<usize> {
    //     for (i, x) in vec.iter_mut().enumerate() {
    //         if x.0 == element {
    //             x.1 = true;
    //             return Some(i);
    //         }
    //     }
    //     None
    // }

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

        data.remove_duplicate_columns();

        let mut current_data_structure = DataStruct::default();

        {
            let fields_sheet1 = data.sheet1[0].clone();
            let mut fields_sheet1: Vec<(String, bool)> = fields_sheet1
                .iter()
                .map(|val| (val.to_string(), false))
                .collect();

            for element in EXPECTED_SHEET1 {
                let actual_idx = fields_sheet1.iter_mut().position(|x| {
                    if x.0 == element && x.1 == false {
                        x.1 = true;
                        return true;
                    } else {
                        return false;
                    }
                });

                let actal_idx = match actual_idx {
                    Some(idx) => idx,
                    None => 10000,
                };
                if actal_idx != 10000 {
                    current_data_structure
                        .sheet1
                        .push((element.into(), actal_idx))
                }
            }
        }

        {
            let fields_sheet2 = data.sheet2[0].clone();
            let mut fields_sheet2: Vec<(String, bool)> = fields_sheet2
                .iter()
                .map(|val| (val.to_string(), false))
                .collect();

            for element in EXPECTED_SHEET2 {
                let actual_idx = fields_sheet2.iter_mut().position(|x| {
                    if x.0 == element && x.1 == false {
                        x.1 = true;
                        return true;
                    } else {
                        return false;
                    }
                });
                let actal_idx = match actual_idx {
                    Some(idx) => idx,
                    None => 10000,
                };
                if actal_idx != 10000 {
                    current_data_structure
                        .sheet2
                        .push((element.into(), actal_idx))
                }
            }
        }

        {
            let fields_sheet3 = data.sheet3[0].clone();
            let mut fields_sheet3: Vec<(String, bool)> = fields_sheet3
                .iter()
                .map(|val| (val.to_string(), false))
                .collect();

            for element in EXPECTED_SHEET3 {
                let actual_idx = fields_sheet3.iter_mut().position(|x| {
                    if x.0 == element && x.1 == false {
                        x.1 = true;
                        return true;
                    } else {
                        return false;
                    }
                });
                let actal_idx = match actual_idx {
                    Some(idx) => idx,
                    None => 10000,
                };
                if actal_idx != 10000 {
                    current_data_structure
                        .sheet3
                        .push((element.into(), actal_idx))
                }
            }
        }

        {
            let fields_sheet4 = data.sheet4[0].clone();
            let mut fields_sheet4: Vec<(String, bool)> = fields_sheet4
                .iter()
                .map(|val| (val.to_string(), false))
                .collect();

            for element in EXPECTED_SHEET4 {
                let actual_idx = fields_sheet4.iter_mut().position(|x| {
                    if x.0 == element && x.1 == false {
                        x.1 = true;
                        return true;
                    } else {
                        return false;
                    }
                });
                let actal_idx = match actual_idx {
                    Some(idx) => idx,
                    None => 10000,
                };
                if actal_idx != 10000 {
                    current_data_structure
                        .sheet4
                        .push((element.into(), actal_idx))
                }
            }
        }

        {
            let fields_sheet5 = data.sheet5[0].clone();
            let mut fields_sheet5: Vec<(String, bool)> = fields_sheet5
                .iter()
                .map(|val| (val.to_string(), false))
                .collect();

            for element in EXPECTED_SHEET5 {
                let actual_idx = fields_sheet5.iter_mut().position(|x| {
                    if x.0 == element && x.1 == false {
                        x.1 = true;
                        return true;
                    } else {
                        return false;
                    }
                });
                let actal_idx = match actual_idx {
                    Some(idx) => idx,
                    None => 10000,
                };
                if actal_idx != 10000 {
                    current_data_structure
                        .sheet5
                        .push((element.into(), actal_idx))
                }
            }
        }

        if self.option == OutputOpt::Hashmap {
            let mut forms: HashMap<String, Form> = HashMap::new();

            for idx in 1..data.sheet1.len() {
                let application_number_idx = current_data_structure.sheet1[2].1;
                let application_number = data.sheet1[idx].get(application_number_idx).unwrap();

                let sheet_ds = current_data_structure.clone();

                let mut form = Form::default();
                for field in sheet_ds.sheet1 {
                    let val = data.sheet1[idx][field.1].clone();
                    form.sheet1.push((field.0, val));
                }
                for field in sheet_ds.sheet2 {
                    let val = data.sheet2[idx][field.1].clone();
                    form.sheet2.push((field.0, val))
                }
                for field in sheet_ds.sheet3 {
                    let val = data.sheet3[idx][field.1].clone();
                    form.sheet3.push((field.0, val))
                }
                for field in sheet_ds.sheet4 {
                    let val = data.sheet4[idx][field.1].clone();
                    form.sheet4.push((field.0, val))
                }
                for field in sheet_ds.sheet5 {
                    let val = data.sheet5[idx][field.1].clone();
                    form.sheet5.push((field.0, val))
                }

                forms.insert(application_number.to_owned(), form);
            }

            fs::write(&self.output, json!(forms).to_string()).unwrap();
        }

        if self.option == OutputOpt::Vector {
            let mut forms: Vec<Form> = Vec::new();

            for idx in 1..data.sheet1.len() {
                // let application_number_idx = current_data_structure.sheet1[2].1;
                // let application_number = data.sheet1[idx].get(application_number_idx).unwrap();

                let sheet_ds = current_data_structure.clone();

                let mut form = Form::default();
                for field in sheet_ds.sheet1 {
                    let val = data.sheet1[idx][field.1].clone();
                    form.sheet1.push((field.0, val));
                }
                for field in sheet_ds.sheet2 {
                    let val = data.sheet2[idx][field.1].clone();
                    form.sheet2.push((field.0, val))
                }
                for field in sheet_ds.sheet3 {
                    let val = data.sheet3[idx][field.1].clone();
                    form.sheet3.push((field.0, val))
                }
                for field in sheet_ds.sheet4 {
                    let val = data.sheet4[idx][field.1].clone();
                    form.sheet4.push((field.0, val))
                }
                for field in sheet_ds.sheet5 {
                    let val = data.sheet5[idx][field.1].clone();
                    form.sheet5.push((field.0, val))
                }

                forms.push(form);
            }

            fs::write(&self.output, json!(forms).to_string()).unwrap();
        }

        Ok(())
    }
}
