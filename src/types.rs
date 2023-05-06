use pyo3::prelude::*;

#[pyclass]
pub struct Integer {
    #[pyo3(get, set)]
    inner: i32,
}

#[pymethods]
impl Integer {
    #[new]
    fn new(value: i32) -> Integer {
        Integer { inner: value }
    }
}

// A "tuple" struct
#[pyclass]
pub struct Number(i32);

// PyO3 supports custom discriminants in enums
#[pyclass]
pub enum HttpResponse {
    Ok = 200,
    NotFound = 404,
    Teapot = 418,
    // ...
}

#[pyclass]
pub enum MyEnum {
    Variant,
    OtherVariant = 30, // PyO3 supports custom discriminants.
}