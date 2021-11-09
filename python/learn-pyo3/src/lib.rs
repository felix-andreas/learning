use pyo3::{prelude::*, PyObjectProtocol};

#[pyfunction]
fn count_rs(text: &str, value: &str) -> usize {
    text.matches(value).count()
}

#[pyfunction]
fn count_iter_rs(text: &str, value: &str) -> usize {
    text.lines()
        .map(|line| line.split(' ').filter(|word| word == &value).count())
        .sum()
}

#[pyfunction]
fn count_naive_rs(text: &str, value: &str) -> usize {
    let mut total = 0;
    for line in text.lines() {
        for word in line.split(' ') {
            if word == value {
                total += 1;
            }
        }
    }
    total
}

#[pyclass]
struct Point2D {
    #[pyo3(get, set)]
    x: f64,
    #[pyo3(get, set)]
    y: f64,
}

#[pymethods]
impl Point2D {
    #[new]
    fn new(x: f64, y: f64) -> Point2D {
        Point2D { x, y }
    }
}

#[pyproto]
impl PyObjectProtocol for Point2D {
    fn __repr__(&self) -> String {
        format!("Point2D(x={},y={})", self.x, self.y)
    }
}

#[pymodule]
fn sample_rs(_python: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(count_rs, module)?)?;
    module.add_function(wrap_pyfunction!(count_iter_rs, module)?)?;
    module.add_function(wrap_pyfunction!(count_naive_rs, module)?)?;
    module.add_class::<Point2D>()?;
    Ok(())
}
