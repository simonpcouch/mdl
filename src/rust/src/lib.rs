use extendr_api::prelude::*;
use ndarray::{Array2, s};

#[extendr]
fn model_matrix(data: List) -> Result<Robj> {
    let nrow = data.iter().next().map(|(_, col)| col.len()).unwrap_or(0);
    let mut processed_columns: Vec<Array2<f64>> = Vec::new();
    let mut column_names: Vec<String> = Vec::new();

    for (col_name, column) in data.iter() {
        match column.rtype() {
            Rtype::Integers => {
                if column.inherits("factor") {
                    // Handle factor
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
                    
                    processed_columns.push(dummy_cols);
                    
                    // Generate names for dummy columns
                    for level in levels.iter().skip(1) {
                        column_names.push(format!("{}_{}", col_name, level));
                    }
                } else {
                    // Handle regular integer column
                    let int_col: Vec<i32> = column.as_integer_vector().unwrap();
                    let float_col: Array2<f64> = Array2::from_shape_vec((nrow, 1), int_col.into_iter().map(|x| x as f64).collect()).unwrap();
                    processed_columns.push(float_col);
                    column_names.push(col_name.to_string());
                }
            },
            Rtype::Doubles => {
                let float_col: Array2<f64> = Array2::from_shape_vec((nrow, 1), column.as_real_vector().unwrap()).unwrap();
                processed_columns.push(float_col);
                column_names.push(col_name.to_string());
            },
            Rtype::Strings | Rtype::Logicals => {
                let str_col: Vec<String> = column.as_str_vector().unwrap().into_iter().map(|s| s.to_string()).collect();
                let mut levels: Vec<String> = str_col.clone();
                levels.sort();
                levels.dedup();
                levels.pop(); // Remove the last level to avoid perfect multicollinearity

                let mut dummy_cols = Array2::<f64>::zeros((nrow, levels.len()));
                for (i, val) in str_col.iter().enumerate() {
                    if let Some(pos) = levels.iter().position(|x| x == val) {
                        dummy_cols[[i, pos]] = 1.0;
                    }
                }
                processed_columns.push(dummy_cols);
                
                // Generate names for dummy columns
                for level in levels.iter() {
                    column_names.push(format!("{}_{}", col_name, level));
                }
            },
            _ => return Err(Error::Other(format!("Unsupported column type: {:?}", column.rtype()))),
        }
    }

    // Combine all processed columns
    let ncol = processed_columns.iter().map(|arr| arr.ncols()).sum();
    let mut result = Array2::<f64>::zeros((nrow, ncol));
    let mut col_offset = 0;
    for col in processed_columns {
        let n = col.ncols();
        result.slice_mut(s![.., col_offset..col_offset+n]).assign(&col);
        col_offset += n;
    }

    let rarray = RArray::new_matrix(
        result.nrows(),
        result.ncols(),
        |r, c| result[[r, c]]
    );

    // Convert RArray to Robj
    let robj: Robj = rarray.into();

    // Create dimnames list
    let row_names: Vec<String> = (1..=result.nrows()).map(|i| i.to_string()).collect();
    let dimnames = List::from_values(&[row_names, column_names.clone()]);
    
    // Set dimnames attribute
    robj.set_attrib("dimnames", dimnames)?;

    Ok(robj)
}

// Generate exports
extendr_module! {
    mod mdl;
    fn model_matrix;
}
