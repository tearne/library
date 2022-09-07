
use pyo3::prelude::*;
use dict_derive::{FromPyObject, IntoPyObject};

#[derive(FromPyObject, IntoPyObject)]
struct User {
    name: String,
    email: String,
    age: u32,
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyinterop(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(get_default_user, m)?)?;
    m.add_function(wrap_pyfunction!(get_contact_info, m)?)?;
    Ok(())
}

#[pyfunction]
fn get_default_user() -> PyResult<User> {
    Ok(User {
        name: "Default".to_owned(),
        email: "default@user.com".to_owned(),
        age: 27,
    })
}

#[pyfunction]
fn get_contact_info(user: User) -> PyResult<String> {
    Ok(format!("{} - {}", user.name, user.email))
}