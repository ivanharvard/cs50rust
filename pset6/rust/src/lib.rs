use pyo3::prelude::*;

mod hello;

#[pyfunction]
fn hello_main() {
    hello::main();
}

#[pymodule]
fn cs50rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello_main, m)?)?;
    Ok(())
}