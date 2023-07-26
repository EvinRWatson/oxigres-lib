use postgres::Row;

fn append_column(row: &postgres::Row, column: &postgres::Column, json_response: &mut String,
                 num_columns: &mut usize, current_column_num: &mut usize)  -> Result<(), String>{
    match column.type_().name() {
        "int4" => {
            match row.try_get::<&str, i32>(column.name()) {
                Ok(value) => {
                    json_response.push_str(&format!("\t\t\"{}\": {}", column.name(), value))
                },
                Err(_) => {
                    json_response.push_str(&format!("\t\t\"{}\": null", column.name()))
                }
            };
        },
        "varchar" => {
            match row.try_get::<&str, String>(column.name()) {
                Ok(value) => {
                    json_response.push_str(&format!("\t\t\"{}\": \"{}\"", column.name(), value))
                },
                Err(_) => {
                    json_response.push_str(&format!("\t\t\"{}\": null", column.name()))
                }
            };
        },
        _ => {
            return Err(format!("Data Type Not Supported By Deserializer: {}", column.type_().name().to_string()));
        }
    }
    
    if current_column_num < num_columns {
        json_response.push_str(",\n");
    }
    else {
        json_response.push_str("\n");
    }

    return Ok(());
}

fn append_row(row: &postgres::Row, json_response: &mut String,
              num_rows: &mut usize, current_row_num: &mut usize) -> Result<(), String> {
    let mut num_columns: usize = row.columns().len();
    let mut current_column_num : usize = 0;

    json_response.push_str("\t{\n");
    for column in row.columns() {
        current_column_num += 1;
        match append_column(&row, &column, json_response, &mut num_columns, &mut current_column_num) {
            Ok(()) => (),
            Err(error) => return Err(error)
        }
    }

    if current_row_num < num_rows {
        json_response.push_str("\t},\n");
    }
    else {
        json_response.push_str("\t}\n");
    }

    return Ok(());
}

pub fn convert_response_to_json_string(response: &Vec<Row>) -> Result<String, String> {
    let mut json_response: String = String::from("[\n");
    let mut num_rows: usize = response.len();
    let mut current_row_num: usize = 0;

    for row in response {
        current_row_num += 1;
        match append_row(row, &mut json_response, &mut num_rows, &mut current_row_num) {
            Ok(()) => (),
            Err(error) => return Err(error)
        }
    }
    json_response.push_str("]");

    return Ok(json_response);
}

pub fn error_json(error: &str) -> String {
    return format!("[\n\t{{\n\t\terror: \"{}\"\n\t}}\n]", error);
}