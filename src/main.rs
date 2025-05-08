use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::path::PathBuf;
use std::error::Error;
use unicode_width::UnicodeWidthStr;

fn read_file(path: &Path) -> Result<String, Box<dyn Error>> {
    let mut content: String = String::from("");

    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error while opening file {file_name}: {e}", file_name=path.display());
            return Err(Box::new(e));
        }
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line + "\n",
            Err(e) => {
                eprintln!("Error while reading file {file_name}: {e}", file_name=path.display());
                return Err(Box::new(e));
            }
        };

        content.push_str(line.as_str());
    }

    Ok(content)

}

fn parse_csv(content: String) -> Vec<Vec<String>> {
    let mut parsed: Vec<Vec<String>> = Vec::new();

    let lines: Vec<String> = content.split("\n").map(|x| {String::from(x)}).collect();

    let fields: Vec<String> = lines[0].split(",").map(|x| {String::from(x.trim())}).collect();
    
    let n_fields = fields.len();

    parsed.push(
        fields
    );

    for line in &lines[1..lines.len()-1] {
        
        let mut intermediate_parsing: Vec<String> = line.split(",").map(|x| {String::from(x.trim())}).collect();

        if intermediate_parsing.len() < n_fields {
            for _ in 0..(n_fields-intermediate_parsing.len()+1) {
                intermediate_parsing.push(String::from(" "));
            }
        }

        parsed.push(
            intermediate_parsing
        );

    }
    
    parsed
}

fn maxlen_column(table: &Vec<Vec<String>>, col: usize) -> usize {
    let mut max = 0;
    for row in table {
        if let Some(cell) = row.get(col) {
            max = max.max(cell.width());
        }
    }
    max + 2
}

fn print_csv(table: &Vec<Vec<String>>) {

    let mut columns_len: Vec<usize> = Vec::new();

    for i in 0..table[0].len() {
        columns_len.push(maxlen_column(table, i));
    }

    // TOP
    print!("┌");
    for i in 0..columns_len.len() {
        print!("{val}", val="─".repeat(columns_len[i]));
        if i != columns_len.len()-1 {
            print!("┬");
        }
    }
    println!("┐");

    // MIDDLE
    for i in 0..table.len() {
        for j in 0..table[0].len() {
            print!("│{val}", val=table[i][j]);
            print!("{val}", val=" ".repeat(columns_len[j] - table[i][j].width()))
        }
        println!("│");  
        
        // UNDER VALUES
        if i != table.len()-1 {
            print!("├");
            for i in 0..columns_len.len() {
                print!("{val}", val="─".repeat(columns_len[i]));
                if i != columns_len.len()-1 {
                    print!("┼");
                }
            }
            println!("┤");
        }

    }

    // BOTTOM
    print!("└");
    for i in 0..columns_len.len() {
        print!("{val}", val="─".repeat(columns_len[i]));
        if i != columns_len.len()-1 {
            print!("┴");
        }
    }
    println!("┘");

}

fn main() {
    // Collecting files name from cmd
    let args: Vec<String> = env::args().collect();

    let paths: Vec<PathBuf> = args.iter().filter(|x| {x.ends_with(".csv")}).map(|x| {PathBuf::from(x)}).collect();

    // Read CSVs line by line
    let mut files: Vec<String> = Vec::new();
    for path in paths {
        match read_file(path.as_path()) {
            Ok(content) => files.push(content),
            Err(e) => eprintln!("Error occured while working with the files: {e}")
        };
    }
    
    for file in files {
        let table = parse_csv(file);

        print_csv(&table);

        println!("Rows: {rows}, Cols: {cols}", rows=table.len(), cols=table[0].len());
    }

}
