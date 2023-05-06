use pyo3::prelude::*;

mod types;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

fn register_child_module(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let child_module = PyModule::new(py, "child")?;
    child_module.add_function(wrap_pyfunction!(sum_as_string, child_module)?)?;
    parent_module.add_submodule(child_module)?;
    Ok(())
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
#[pyo3(name = "py_rust_playground")] // python package name
fn py_rust_playground(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;

    m.add_class::<types::Integer>()?;
    m.add_class::<types::Number>()?;
    m.add_class::<types::HttpResponse>()?;
    m.add_class::<types::MyEnum>()?;

    register_child_module(_py, m)?;
    Ok(())
}