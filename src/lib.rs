extern crate pyo3;
extern crate uuid;

use pyo3::class::PyObjectProtocol;
use pyo3::exceptions::{TypeError, ValueError};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyInt, PyTuple};

use uuid::Uuid;

#[pymodule]
fn fastuuid(py: Python, m: &PyModule) -> PyResult<()> {
    #[pyclass]
    struct UUID {
        handle: Uuid,
    }

    #[pymethods]
    impl UUID {
        #[new]
        fn new(
            obj: &PyRawObject,
            hex: Option<&str>,
            bytes: Option<Py<PyBytes>>,
            bytes_le: Option<Py<PyBytes>>,
            fields: Option<Py<PyTuple>>,
            int: Option<u128>,
            py: Python,
        ) -> PyResult<()> {
            {
                // Verbose but it works
                let mut c = 0;

                if let None = hex {
                    c += 1;
                }

                if let None = bytes {
                    c += 1;
                }

                if let None = bytes_le {
                    c += 1;
                }

                if let None = fields {
                    c += 1;
                }

                if let None = int {
                    c += 1;
                }

                if c != 4 {
                    return Err(PyErr::new::<TypeError, &str>(
                        "one of the hex, bytes, bytes_le, fields, or int arguments must be given",
                    ));
                }
            }

            // TODO: Remove default value
            let mut handle = Uuid::nil();

            if let Some(h) = hex {
                if let Ok(uuid) = Uuid::parse_str(h) {
                    handle = uuid;
                } else {
                    // TODO: Provide more context to why the string wasn't parsed correctly.
                    return Err(PyErr::new::<ValueError, &str>(
                        "badly formed hexadecimal UUID string",
                    ));
                }
            }

            if let Some(i) = int {
                handle = i.swap_bytes().into();
            }

            if let Some(b) = bytes {
                let b = b.to_object(py);
                let b = b.cast_as::<PyBytes>(py)?;
                let b = b.as_bytes();
                let mut a: [u8; 16] = Default::default();
                a.copy_from_slice(&b[0..16]);
                handle = Uuid::from_bytes(a);
            }

            if let Some(b) = bytes_le {
                let b = b.to_object(py);
                let b = b.cast_as::<PyBytes>(py)?;
                let b = b.as_bytes();
                let mut a: [u8; 16] = Default::default();
                a.copy_from_slice(&b[0..16]);
                // Convert little endian to big endian
                a[0..4].reverse();
                a[4..6].reverse();
                a[6..8].reverse();
                handle = Uuid::from_bytes(a);
            }

            if let Some(f) = fields {
                // TODO: Handle errors
                let f = f.to_object(py);
                let f = f.cast_as::<PyTuple>(py)?;
                let d1: u32 = f.get_item(0).downcast_ref::<PyInt>()?.extract::<u32>()?;
                let d2: u16 = f.get_item(1).downcast_ref::<PyInt>()?.extract::<u16>()?;
                let d3: u16 = f.get_item(2).downcast_ref::<PyInt>()?.extract::<u16>()?;
                let remaining_fields = f.split_from(3);
                let remaining_fields = remaining_fields.to_object(py);
                let remaining_fields = remaining_fields.cast_as::<PyTuple>(py)?;
                let d4 = remaining_fields
                    .iter()
                    .map(|x| x.downcast_ref::<PyInt>().unwrap().extract::<u8>().unwrap())
                    .collect::<Vec<u8>>();
                if let Ok(uuid) = Uuid::from_fields(d1, d2, d3, d4.as_slice()) {
                    handle = uuid;
                } else {
                    // TODO: Provide more context to why the fields weren't parsed correctly.
                    return Err(PyErr::new::<ValueError, &str>("fields error"));
                }
            }

            obj.init(UUID { handle: handle });
            Ok(())
        }
    }

    #[pyproto]
    impl<'p> PyObjectProtocol<'p> for UUID {
        fn __str__(&self) -> PyResult<String> {
            Ok(self
                .handle
                .to_hyphenated()
                .encode_lower(&mut Uuid::encode_buffer())
                .to_string())
        }

        fn __repr__(&self) -> PyResult<String> {
            let s = self.__str__()?;
            Ok(format!("UUID('{}')", s))
        }
    }

    // #[pyfn(m, "uuid3")]
    // fn uuid3(namespace: UUID, name: &PyBytes, py: Python) -> UUID {
    //     UUID {
    //         handle: Uuid::new_v3(&namespace.handle, name.as_bytes()),
    //     }
    // }

    #[pyfn(m, "uuid4")]
    fn uuid4() -> UUID {
        UUID {
            handle: Uuid::new_v4(),
        }
    }

    m.add_class::<UUID>()?;

    Ok(())
}
