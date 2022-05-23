use malachi::Match;
use pyo3::{
	exceptions,
	prelude::*,
	types::PyDict,
};

#[pyclass]
pub struct Command(malachi::Command);

#[pymethods]
impl Command {
	#[new]
	fn new(s: &str) -> PyResult<Self> {
		malachi::Command::new(s)
			.map(Self)
			.map_err(|e| PyErr::new::<exceptions::PyValueError, _>(e.to_string()))
	}

	fn get_matches<'a>(&self, py: Python<'a>, s: &str) -> PyResult<Option<&'a PyDict>> {
		let m = match self.0.get_matches(s) {
			None => return Ok(None),
			Some(x) => x,
		};
		let dict = PyDict::new(py);
		let rest = m.rest.to_string();
		for (key, val) in m.into_matches() {
			match val {
				Match::Once(x) => dict.set_item(key, x)?,
				Match::Many(x) => dict.set_item(key, x)?,
			}
		}
		dict.set_item("_", rest)?;
		Ok(Some(dict))
	}

	fn has_prefix(&self, s: &str) -> bool {
		self.0.has_prefix(s)
	}
}

#[pymodule]
fn malachi(_py: Python, m: &PyModule) -> PyResult<()> {
	m.add_class::<Command>()?;
	Ok(())
}
