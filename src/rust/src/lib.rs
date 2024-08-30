use extendr_api::prelude::*;
use ndarray::{Array2, s};

#[extendr]
fn model_matrix(data: List) -> Result<RMatrix<f64>> {
    let nrow = data.iter().next().map(|(_, col)| col.len()).unwrap_or(0);
    let mut processed_columns: Vec<Array2<f64>> = Vec::new();

    for (_, column) in data.iter() {
        match column.rtype() {
            Rtype::Integers => {
                let int_col: Vec<i32> = column.as_integer_vector().unwrap();
                let float_col: Array2<f64> = Array2::from_shape_vec((nrow, 1), int_col.into_iter().map(|x| x as f64).collect()).unwrap();
                processed_columns.push(float_col);
            },
            Rtype::Doubles => {
                let float_col: Array2<f64> = Array2::from_shape_vec((nrow, 1), column.as_real_vector().unwrap()).unwrap();
                processed_columns.push(float_col);
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
            },
            _ => return Err(Error::Other(format!("Unsupported column type: {:?}", column.rtype()))),
        }
    }

    // Combine all processed columns
    let ncol = processed_columns.iter().map(|arr| arr.ncols()).sum();
    let mut result= Array2::<f64>::zeros((nrow, ncol));
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
    Ok(rarray.into())
}

// Generate exports
extendr_module! {
    mod mdl;
    fn model_matrix;
}
