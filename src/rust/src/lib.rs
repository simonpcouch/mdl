use extendr_api::prelude::*;
use std::iter::zip;

#[extendr]
fn model_matrix(data: List) -> Result<Robj> {
    let nrow = data.iter().next().map(|(_, col)| col.len()).unwrap_or(0);
    let columns = data.iter().map(|(id, col)| {
        if col.is_string() {
            // convert strings (character-vector) to factor, via R,
            // as R can do this more efficiently than us
            (id, R!("factor({{col}})").unwrap())
        } else {
            (id, col)
        }
    });

    // every factor level but the first is turned into a one-hot vector, 
    // thus every factor results in levels()-1 many more columns
    let ncol: usize = columns
        .clone()
        .map(|(_id, x)| {
            if x.is_factor() {
                x.levels().unwrap().len() - 1
            } else {
                1
            }
        })
        .sum();

    // add intercept
    let ncol = ncol + 1;

    let mut processed_columns_matrix: RMatrix<f64> = RMatrix::new(nrow, ncol);
    let processed_columns = processed_columns_matrix.as_real_slice_mut().unwrap();
    let mut column_names: Vec<String> = Vec::with_capacity(ncol);

    // Add intercept column
    processed_columns[0..nrow].fill(1.);
    column_names.push("intercept".to_string());

    // Iterate through columns
    let mut current_column = 1; // we passed the intercept
    let mut data_iter = columns;
    loop {
        let (col_name, column) = if let Some(next_column) = data_iter.next() {
            next_column
        } else {
            break;
        };

        match column.rtype() {
            // Note that factors match this first condition
            Rtype::Integers if column.is_factor() => {
                let nlevels = column.levels().unwrap().len() - 1;
                process_factor_column(
                    &column,
                    col_name,
                    nrow,
                    &mut column_names,
                    &mut processed_columns
                        [(current_column * nrow)..((current_column + nlevels) * nrow)],
                )
            }
            Rtype::Integers => process_integer_column(
                &column,
                col_name,
                &mut column_names,
                &mut processed_columns[(current_column * nrow)..((current_column + 1) * nrow)],
            ),
            Rtype::Doubles => process_double_column(
                &column,
                col_name,
                &mut column_names,
                &mut processed_columns[(current_column * nrow)..((current_column + 1) * nrow)],
            ),
            Rtype::Logicals => process_logical_column(
                &column,
                col_name,
                &mut column_names,
                &mut processed_columns[(current_column * nrow)..((current_column + 1) * nrow)],
            ),
            _ => {
                return Err(Error::Other(format!(
                    "Unsupported column type: {:?}",
                    column.rtype()
                )))
            }
        };

        if column.is_factor() {
            current_column += column.levels().unwrap().len() - 1;
        } else {
            current_column += 1;
        }
    }
    let mut robj: Robj = processed_columns_matrix.into();

    // Create dimnames list
    let row_names = R!("seq_len({{nrow}})").unwrap();
    let dimnames = List::from_values(&[row_names, column_names.into()]);

    // Set dimnames attribute
    robj.set_attrib("dimnames", dimnames)?;

    Ok(robj)
}

fn process_integer_column(
    column: &Robj,
    col_name: &str,
    output_column_names: &mut Vec<String>,
    output: &mut [f64],
) {
    output_column_names.push(col_name.to_string());
    zip(column.as_integer_slice().unwrap().iter(), output.iter_mut()).for_each(
        |(integer_element, output)| {
            *output = *integer_element as f64;
        },
    );
}

fn process_factor_column(
    column: &Robj,
    col_name: &str,
    nrow: usize,
    output_column_names: &mut Vec<String>,
    output: &mut [f64],
) {
    output_column_names.extend(
        column
            .levels()
            .unwrap()
            .skip(1)
            // this is a different separator than the one used in `model.matrix` in R
            .map(|level| format!("{}_{}", col_name, level)),
    );
    // remove the first level from all the factors
    let level_index = column.as_integer_slice().unwrap().iter().map(|x| *x - 2);
    output.fill(0.);
    for (row_id, level_index) in level_index.enumerate() {
        // we should have skipped level 1 tags anyways, so we do that here.
        let col_id: Option<usize> = level_index.try_into().ok();
        if let Some(col_id) = col_id {
            let linear_id = row_id + col_id * nrow;

            output[linear_id] = 1.0;
        }
    }
}

fn process_double_column(
    column: &Robj,
    col_name: &str,
    output_column_names: &mut Vec<String>,
    output: &mut [f64],
) {
    output_column_names.push(col_name.to_string());
    output.copy_from_slice(column.as_real_slice().unwrap())
}

fn process_logical_column(
    column: &Robj,
    col_name: &str,
    output_column_names: &mut Vec<String>,
    output: &mut [f64],
) {
    output_column_names.push(col_name.to_string());
    zip(column.as_logical_iter().unwrap(), output.iter_mut()).for_each(
        |(logical_element, output)| {
            *output = logical_element.to_bool() as i32 as f64;
        },
    );
}

// Generate exports
extendr_module! {
    mod mdl;
    fn model_matrix;
}

#[cfg(test)]
mod tests {
    use extendr_engine::with_r;

    use super::*;

    #[test]
    fn test_character_vector_conversion() {
        with_r(|| {
            let dd = R!(r#"data.frame(x = (c("a", "b", "c")))"#).unwrap();
            let _ = model_matrix(dd.as_list().unwrap());
        });
    }
}
