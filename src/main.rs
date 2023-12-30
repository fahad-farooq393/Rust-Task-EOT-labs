use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

// Define a trait for CSV operations
trait CSVOperations {
    fn display(&self);
    fn paginate(&self, start: usize, end: usize);
    fn delete_row(&mut self, index: usize);
    fn modify_field(&mut self, row: usize, col: usize, new_value: String);
    fn save_to_file(&self, path: &str, update_existing: bool) -> Result<(), Box<dyn Error>>;
}

// Define a struct to hold CSV data
struct CSVData {
    data: Vec<Vec<String>>,
    records: usize,
    fields: usize,
}

impl CSVData {
    fn new(data: Vec<Vec<String>>) -> Self {
        let records = data.len();
        let fields = if records > 0 { data[0].len() } else { 0 };
        CSVData { data, records, fields }
    }
}

// Implement the CSVOperations trait for CSVData
impl CSVOperations for CSVData {
    fn display(&self) {
        for row in &self.data {
            for field in row {
                print!("{}, ", field);
            }
            println!();
        }
    }

    fn paginate(&self, start: usize, end: usize) {
        for i in start..=end {
            if let Some(row) = self.data.get(i - 1) {
                for field in row {
                    print!("{}, ", field);
                }
                println!();
            }
        }
    }

    fn delete_row(&mut self, index: usize) {
        if index <= self.records {
            self.data.remove(index - 1);
            self.records -= 1;
        }
    }

    fn modify_field(&mut self, row: usize, col: usize, new_value: String) {
        if row <= self.records && col <= self.fields {
            if let Some(row_vec) = self.data.get_mut(row - 1) {
                if let Some(field) = row_vec.get_mut(col - 1) {
                    *field = new_value;
                }
            }
        }
    }

    fn save_to_file(&self, path: &str, update_existing: bool) -> Result<(), Box<dyn Error>> {
        let mut file = if update_existing {
            OpenOptions::new().write(true).truncate(true).open(path)?
        } else {
            File::create(path)?
        };

        for (i, row) in self.data.iter().enumerate() {
            for (j, field) in row.iter().enumerate() {
                write!(file, "{}", field)?;
                if j < self.fields - 1 {
                    write!(file, ",")?;
                }
            }
            if i < self.records - 1 {
                writeln!(file)?;
            }
        }

        Ok(())
    }
}

// Function to read CSV data from a file
fn read_csv_file(path: &str) -> Result<CSVData, Box<dyn Error>> {
    let file = File::open(path);
    
    let mut file = match file {
        Ok(f) => f,
        Err(_) => {
            println!("Error opening CSV file.");
            return Err("Error opening CSV file.".into());
        }
    };

    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        println!("Error reading CSV file: {}", err);
        return Err(err.into());
    }

    let mut data = Vec::new();
    for line in contents.lines() {
        let fields: Vec<String> = line.split(',').map(|s| s.trim().to_string()).collect();
        data.push(fields);
    }

    Ok(CSVData::new(data))
}


// Function to read user input as usize
fn read_input() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().unwrap_or(0)
}

// Function to read user input as String
fn read_input_as_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().unwrap_or(0.to_string())
}

fn main() {
    if let Ok(csv_data) = read_csv_file("testdata.csv") {
        let mut current_data = csv_data;

        loop {
            println!("Options:");
            println!("1. Display entire file");
            println!("2. Paginate");
            println!("3. Delete a row");
            println!("4. Modify a field");
            println!("5. Save to file");
            println!("6. Quit");

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("Failed to read line");
            let choice: usize = match choice.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input. Please enter a number.");
                    continue;
                }
            };

            match choice {
                1 => current_data.display(),
                2 => {
                    println!("Enter start index: ");
                    let start: usize = read_input();
                    println!("Enter end index: ");
                    let end: usize = read_input();
                    current_data.paginate(start, end);
                }
                3 => {
                    println!("Enter row index to delete: ");
                    let index: usize = read_input();
                    current_data.delete_row(index);
                }
                4 => {
                    println!("Enter row index: ");
                    let row: usize = read_input();
                    println!("Enter column index: ");
                    let col: usize = read_input();
                    println!("Enter new value: ");
                    let new_value = read_input_as_string();
                    current_data.modify_field(row, col, new_value.to_string());
                }
                5 => {
                    println!("Do you want to save to an existing file? (y/n)");
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("Failed to read line");
                    let update_existing = answer.trim().to_lowercase() == "y";

                    let file_path = match update_existing {
                        true => "testdata.csv",
                        false => "output.csv",
                    };

                    if let Err(err) = current_data.save_to_file(file_path, update_existing) {
                        eprintln!("Error saving to file: {}", err);
                    } else {
                        println!("Data saved to file: {}", file_path);
                    }
                }
                6 => break,
                _ => println!("Invalid choice"),
            }
        }
    } else {
        println!("Error reading CSV file.");
    }
}
