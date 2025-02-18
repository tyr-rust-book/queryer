use pyo3::{exceptions, prelude::*};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn example_sql() -> PyResult<String> {
    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";

    let sql = format!(
        "SELECT location name, total_cases, new_cases, total_deaths, new_deaths \
      FROM {} where new_deaths >= 500 ORDER BY new_cases DESC LIMIT 6 OFFSET 5",
        url
    );

    Ok(sql)
}

#[pyfunction]
#[pyo3(name = "query", signature = (sql, output = "csv"))]
pub fn query_py(sql: &str, output: &str) -> PyResult<String> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut data = rt.block_on(async { queryer::query(sql).await.unwrap() });
    match output {
        "csv" => Ok(data.to_csv().unwrap()),
        v => Err(exceptions::PyTypeError::new_err(format!(
            "Output type {} not supported",
            v
        ))),
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn queryer_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(example_sql, m)?)?;
    m.add_function(wrap_pyfunction!(query_py, m)?)?;
    Ok(())
}
