use extendr_api::prelude::*;
use std::collections::HashMap;

#[extendr]
fn model_matrix(data: List) -> Result<Robj> {
    let nrow = data.iter().next().map(|(_, col)| col.len()).unwrap_or(0);
    let mut processed_columns: Vec<Array2<f64>> = Vec::new();
    let mut column_names: Vec<String> = Vec::new();

    // Add intercept column
    processed_columns.push(Array2::ones((nrow, 1)));
    column_names.push("intercept".to_string());

    // Iterate through columns
    for (col_name, column) in data.iter() {
        let (processed_column, mut new_column_names) = match column.rtype() {
            // Note that factors match this first condition
            Rtype::Integers => process_integer_column(&column, col_name, nrow),
            Rtype::Doubles => process_double_column(&column, col_name, nrow),
            Rtype::Strings => process_string_column(&column, col_name, nrow),
            Rtype::Logicals => process_logical_column(&column, col_name, nrow),
            _ => return Err(Error::Other(format!("Unsupported column type: {:?}", column.rtype()))),
        };

        processed_columns.push(processed_column);
        column_names.append(&mut new_column_names);
    }

    // Combine all processed columns.
    // Note that entries in processed_columns may have more than one column.
    let ncol = processed_columns.iter().map(|arr| arr.ncols()).sum();
    let mut result = Array2::<f64>::zeros((nrow, ncol));
    let mut col_offset = 0;
    for col in processed_columns {
        let n = col.ncols();
        result.slice_mut(s![.., col_offset..col_offset+n]).assign(&col);
        col_offset += n;
    }
    let nrows = result.nrows();
    let mut robj: Robj = result.try_into().unwrap();

    // Create dimnames list
    let row_names: Vec<String> = (1..=nrows).map(|i| i.to_string()).collect();
    let dimnames = List::from_values(&[row_names, column_names.clone()]);

    // Set dimnames attribute
    robj.set_attrib("dimnames", dimnames)?;

    Ok(robj)
}

fn process_integer_column(column: &Robj, col_name: &str, nrow: usize) -> (Array2<f64>, Vec<String>) {
    if column.inherits("factor") {
        process_factor_column(column, col_name, nrow)
    } else {
        let int_col: Vec<i32> = column.as_integer_vector().unwrap();
        let float_col: Array2<f64> = Array2::from_shape_vec((nrow, 1), int_col.into_iter().map(|x| x as f64).collect()).unwrap();
        (float_col, vec![col_name.to_string()])
    }
}

fn process_factor_column(column: &Robj, col_name: &str, nrow: usize) -> (Array2<f64>, Vec<String>) {
    let int_col: Vec<i32> = column.as_integer_vector().unwrap();
    let levels: Vec<String> = column.levels().unwrap().map(|s| s.to_string()).collect();
    let mut dummy_cols = Array2::<f64>::zeros((nrow, levels.len() - 1));

    for (i, &val) in int_col.iter().enumerate() {
        if val > 1 && val <= levels.len() as i32 {
            let level_index = (val - 2) as usize;
            if level_index < dummy_cols.ncols() {
                dummy_cols[[i, level_index]] = 1.0;
            }
        }
    }

    let column_names: Vec<String> = levels.iter().skip(1)
        .map(|level| format!("{}_{}", col_name, level))
        .collect();

    (dummy_cols, column_names)
}

fn process_double_column(column: &Robj, col_name: &str, nrow: usize) -> (Array2<f64>, Vec<String>) {
    let float_col: Array2<f64> = Array2::from_shape_vec((nrow, 1), column.as_real_vector().unwrap()).unwrap();
    (float_col, vec![col_name.to_string()])
}

fn process_string_column(column: &Robj, col_name: &str, nrow: usize) -> (Array2<f64>, Vec<String>) {
    let str_col: Vec<&str> = column.as_str_vector().unwrap();
    let mut level_map: HashMap<&str, usize> = HashMap::with_capacity(str_col.len());
    let mut levels: Vec<&str> = Vec::new();

    // First pass: create level mapping
    for &s in &str_col {
        if !level_map.contains_key(s) {
            level_map.insert(s, levels.len());
            levels.push(s);
        }
    }

    let num_levels = levels.len();
    let mut dummy_cols = Array2::<f64>::zeros((nrow, num_levels - 1));

    // Second pass: fill dummy columns
    for (i, &s) in str_col.iter().enumerate() {
        let level_index = level_map[s];
        if level_index > 0 {
            dummy_cols[[i, num_levels - level_index - 1]] = 1.0;
        }
    }

    let column_names: Vec<String> = levels.iter().skip(1).rev()
        .map(|&level| format!("{}_{}", col_name, level))
        .collect();

    (dummy_cols, column_names)
}

fn process_logical_column(column: &Robj, col_name: &str, nrow: usize) -> (Array2<f64>, Vec<String>) {
    let bool_col: Vec<Rbool> = column.as_logical_vector().unwrap();
    let float_col: Array2<f64> = Array2::from_shape_vec(
        (nrow, 1),
        bool_col.into_iter().map(|x| if x.is_true() { 1.0 } else { 0.0 }).collect()
    ).unwrap();
    (float_col, vec![col_name.to_string()])
}

// Generate exports
extendr_module! {
    mod mdl;
    fn model_matrix;
}
