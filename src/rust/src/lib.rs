use extendr_api::prelude::*;
use std::iter::zip;

#[cfg(feature = "rayon")]
#[allow(unused_imports)]
use rayon::prelude::*;

#[extendr]
fn model_matrix(data: List) -> Result<Robj> {
    let nrow = data.iter().next().map(|(_, col)| col.len()).unwrap_or(0);
    let columns = data.into_iter().map(|(id, col)| {
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
    let mut processed_columns = processed_columns_matrix.as_real_slice_mut().unwrap();
    let mut column_names: Vec<String> = Vec::with_capacity(ncol);

    let intercept;
    (intercept, processed_columns) = processed_columns.split_at_mut(nrow);
    // Add intercept column
    intercept.fill(1.);
    column_names.push("(Intercept)".to_string());

    let mut data_iter = columns;

    let mut calcs = Vec::new();

    loop {
        let (col_name, column): (&str, Robj) = if let Some(next_column) = data_iter.next() {
            next_column
        } else {
            break;
        };

        match column.rtype() {
            // Note that factors match this first condition
            Rtype::Integers if column.is_factor() => {
                let nlevels = column.levels().unwrap().len() - 1;
                column_names.extend(
                    column
                        .levels()
                        .unwrap()
                        .skip(1)
                        .map(|level| format!("{}{}", col_name, level)),
                );

                let o;
                (o, processed_columns) = processed_columns.split_at_mut(nlevels * nrow);

                calcs.push(Calculation::FactorColumn {
                    column: column.into(),
                    nrow,
                    output: o,
                });
            }
            Rtype::Integers => {
                column_names.push(col_name.to_string());

                let o;
                (o, processed_columns) = processed_columns.split_at_mut(nrow);
                calcs.push(Calculation::IntegerColumn {
                    column: column.into(),
                    output: o,
                });
            }
            Rtype::Doubles => {
                column_names.push(col_name.to_string());

                let o;
                (o, processed_columns) = processed_columns.split_at_mut(nrow);
                calcs.push(Calculation::DoubleColumn {
                    column: column.into(),
                    output: o,
                });
            }
            Rtype::Logicals => {
                column_names.push(format!("{}TRUE", col_name));

                let o;
                (o, processed_columns) = processed_columns.split_at_mut(nrow);

                calcs.push(Calculation::LogicalColumn {
                    column: column.into(),
                    output: o,
                });
            }
            _ => {
                return Err(Error::Other(format!(
                    "Unsupported column type: {:?}",
                    column.rtype()
                )))
            }
        };
    }

    // TODO: Do this in parallel with Rayon
    if cfg!(feature = "rayon") {
        #[cfg(feature = "rayon")]
        calcs.into_par_iter().for_each(|x| x.calculate());
    } else {
        calcs.into_iter().for_each(|x| x.calculate());
    }

    let mut robj: Robj = processed_columns_matrix.into();

    // Create dimnames list
    let row_names = R!("seq_len({{nrow}})").unwrap();
    let dimnames = List::from_values(&[row_names, column_names.into()]);

    // Set dimnames attribute
    robj.set_attrib("dimnames", dimnames)?;

    Ok(robj)
}

enum Calculation<'b> {
    FactorColumn {
        column: Robj,
        nrow: usize,
        output: &'b mut [f64],
    },
    IntegerColumn {
        column: Robj,
        output: &'b mut [f64],
    },
    DoubleColumn {
        column: Robj,
        output: &'b mut [f64],
    },
    LogicalColumn {
        column: Robj,
        output: &'b mut [f64],
    },
}
unsafe impl<'a> Send for Calculation<'a> {}
unsafe impl<'a> Sync for Calculation<'a> {}

impl<'a> Calculation<'a> {
    fn calculate(self) {
        match self {
            Calculation::FactorColumn {
                column,
                nrow,
                output,
            } => {
                process_factor_column(&column, nrow, output);
            }
            Calculation::IntegerColumn { column, output } => {
                process_integer_column(&column, output);
            }
            Calculation::DoubleColumn { column, output } => {
                process_double_column(&column, output);
            }
            Calculation::LogicalColumn { column, output } => {
                process_logical_column(&column, output);
            }
        }
    }
}

fn process_factor_column(column: &Robj, nrow: usize, output: &mut [f64]) {
    // remove the first level from all the factors
    let level_indices = column.as_integer_slice().unwrap();
    let num_levels = column.levels().unwrap().len() - 1;
    output.fill(0.);
    for (row_id, &level_index) in level_indices.iter().enumerate() {
        // we should have skipped level 1 tags anyways, so we do that here.
        if level_index.is_na() {
            for i in 0..num_levels {
                let linear_id = row_id + i * nrow;
                output[linear_id] = f64::na();
            }
        } else {
            let col_id: Option<usize> = (level_index - 2).try_into().ok();
            if let Some(col_id) = col_id {
                if col_id < num_levels {
                    let linear_id = row_id + col_id * nrow;
                    output[linear_id] = 1.0;
                }
            }
        }
    }
}

fn process_integer_column(column: &Robj, output: &mut [f64]) {
    zip(column.as_integer_slice().unwrap().iter(), output.iter_mut()).for_each(
        |(integer_element, output)| {
            if integer_element.is_na() {
                *output = f64::na();
            } else {
                *output = *integer_element as f64;
            }
        },
    );
}

fn process_double_column(column: &Robj, output: &mut [f64]) {
    output.copy_from_slice(column.as_real_slice().unwrap())
}

fn process_logical_column(column: &Robj, output: &mut [f64]) {
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
