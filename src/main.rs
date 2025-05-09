use std::{env, fs};
use std::path::Path;
use std::path::PathBuf;
use std::error::Error;
use unicode_width::UnicodeWidthStr;

fn read_file(path: &Path) -> Result<String, Box<dyn Error>> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) => {
            eprintln!("Error reading file {}: {}", path.display(), e);
            Err(Box::new(e))
        }
    }
}

fn parse_csv(content: String, sep: &str) -> Vec<Vec<String>> {
    let mut table: Vec<Vec<String>> = Vec::new();

    // Split content line by line
    let mut lines = content.lines();

    // Get all the column
    let header: Vec<String> = match lines.next() {
        Some(line) => line.split(sep).map(|value| value.trim().to_string()).collect(),
        None => return table
    };
    
    // Get the total number of columns
    let n_fields = header.len();
    table.push(header);

    for line in lines {
        // Split line on
        let mut row: Vec<String> = line.split(",")
                                        .map(|value| value.trim().to_string())
                                        .collect();

        // Add value padding
        while row.len() < n_fields {
            row.push(" ".to_string());
        }

        table.push(row);

    }
    
    table
}

fn column_size(table: &Vec<Vec<String>>, col: usize) -> usize {
    let mut max = 0;
    for row in table {
        if let Some(cell) = row.get(col) {
            max = max.max(cell.width());
        }
    }

    max + 2
}

fn column_sizes(table: &Vec<Vec<String>>) -> Vec<usize> {
    let mut column_sizes: Vec<usize> = Vec::new();

    for i in 0..table[0].len() {
        column_sizes.push(column_size(table, i));
    }

    column_sizes
}

fn print_line(column_sizes: &Vec<usize>, start: &str, mid: &str, end: &str) {
    // Beginning of the line
    print!("{start}");

    // Line logic
    for i in 0..column_sizes.len() {
        print!("{val}", val="─".repeat(column_sizes[i]));
        if i != column_sizes.len()-1 {
            print!("{mid}");
        }
    }

    // End of the line
    println!("{end}");
}

fn print_row(table: &Vec<Vec<String>>, row: usize, column_sizes: &Vec<usize>) {
    for col in 0..table[0].len() {
        print!("│{val}", val=table[row][col]);
        print!("{val}", val=" ".repeat({
            // Return padding to fill the cell
            column_sizes[col] - table[row][col].width()
        }))
    }
    println!("│");
}

fn print_table(table: &Vec<Vec<String>>) {
    // Get the sizes of all the columns
    let column_sizes = column_sizes(table);

    // TOP
    print_line(&column_sizes, "┌", "┬", "┐");

    // MIDDLE
    for i in 0..table.len() {
        // Print value row
        print_row(table, i, &column_sizes);  
        
        // Print separator line
        if i != table.len()-1 {
            print_line(&column_sizes, "├", "┼", "┤");
        }

    }

    // BOTTOM
    print_line(&column_sizes, "└", "┴", "┘");

    // Print table size
    println!("Rows: {rows}, Cols: {cols}", rows=table.len(), cols=table[0].len());

}

fn main() {
    // Collecting files name from cmd
    let args: Vec<String> = env::args().collect();

    // Turning Strings in PathBufs
    let paths: Vec<PathBuf> = args.iter()
                                    .filter(|x| {x.ends_with(".csv")})
                                    .map(|x| {PathBuf::from(x)})
                                    .collect();

    // For each file
    for path in paths {
        // Read the content of the file
        let content = match read_file(path.as_path()) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error occured while working with the files: {e}");
                continue;
            }
        };

        // Parse the content
        let table = parse_csv(content, ",");

        // Print the content
        print_table(&table);
    }

}
