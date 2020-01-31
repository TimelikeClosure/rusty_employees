use std::io::{Write, stdin, stdout};
use crate::db::Table;

pub fn get_query() -> String {
    let mut input = String::new();
    loop {
        print!("> ");
        stdout().flush().expect("Error writing to stdout");
        match stdin().read_line(&mut input) {
            Err(_) => continue,
            Ok(_) => return input,
        }
    }
}

pub fn print_message(message: String) {
    println!("{}", message);
}

pub fn print_table(table: Table) {
    println!("\n{}\n", table.title);

    // Calculate width of columns based on contents
    let mut column_widths = table.headers.iter().map(|header| header.chars().count()).collect::<Vec<usize>>();
    table.data.iter().for_each(|data_map| {
        table.headers.iter().enumerate().for_each(|(header_index, header_name)| {
            if let Some(data_name) = data_map.get(header_name) {
                let data_width = data_name.len();
                if data_width > column_widths[header_index] {
                    column_widths[header_index] = data_width;
                }
            }
        });
    });
    let column_widths = column_widths;

    table.headers.iter().enumerate().for_each(|(index, name)| {
        if index > 0 {
            print!("|");
        }
        print!(" {name:width$} ", name = name, width = column_widths[index]);
    });
    println!("");
    column_widths.iter().enumerate().for_each(|(index, width)| {
        if index > 0 {
            print!("|");
        }
        print!("-{:-<width$}-", "-", width = width);
    });
    println!("");

    table.data.iter().for_each(|row| {
        table.headers.iter().enumerate().for_each(|(index, column_name)| {
            if index > 0 {
                print!("|");
            }
            let data = match row.get(column_name) {
                None => String::from(" "),
                Some(value) => String::from(value),
            };
            print!(" {:<width$} ", data, width = column_widths[index]);
        });
        println!("");
    });
    println!("");
}
