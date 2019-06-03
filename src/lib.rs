extern crate byteorder;
extern crate pyo3;
extern crate uuid;

use byteorder::ByteOrder;
use pyo3::class::basic::CompareOp;
use pyo3::class::PyObjectProtocol;
use pyo3::exceptions::{NotImplementedError, TypeError, ValueError};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyBytes, PyTuple};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::iter;
use uuid::{Builder, Uuid, Variant, Version};

#[pymodule]
fn fastuuid(_py: Python, m: &PyModule) -> PyResult<()> {
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
            version: Option<u8>,
            py: Python,
        ) -> PyResult<()> {
            let version = match version {
                Some(1) => Ok(Some(Version::Mac)),
                Some(2) => Ok(Some(Version::Dce)),
                Some(3) => Ok(Some(Version::Md5)),
                Some(4) => Ok(Some(Version::Random)),
                Some(5) => Ok(Some(Version::Sha1)),
                None => Ok(None),
                _ => Err(PyErr::new::<ValueError, &str>("illegal version number")),
            }?;

            let result: PyResult<Uuid> = match (hex, bytes, bytes_le, fields, int) {
                (Some(hex), None, None, None, None) => {
                    if let Ok(uuid) = Uuid::parse_str(hex) {
                        Ok(uuid)
                    } else {
                        // TODO: Provide more context to why the string wasn't parsed correctly.
                        Err(PyErr::new::<ValueError, &str>(
                            "badly formed hexadecimal UUID string",
                        ))
                    }
                }
                (None, Some(bytes), None, None, None) => {
                    let b = bytes.to_object(py);
                    let b = b.cast_as::<PyBytes>(py)?;
                    let b = b.as_bytes();
                    let mut a: [u8; 16] = Default::default();
                    a.copy_from_slice(&b[0..16]);

                    let mut builder = Builder::from_bytes(a);
                    if let Some(v) = version {
                        builder.set_version(v);
                    }
                    Ok(builder.build())
                }
                (None, None, Some(bytes_le), None, None) => {
                    let b = bytes_le.to_object(py);
                    let b = b.cast_as::<PyBytes>(py)?;
                    let b = b.as_bytes();
                    let mut a: [u8; 16] = Default::default();
                    a.copy_from_slice(&b[0..16]);
                    // Convert little endian to big endian
                    a[0..4].reverse();
                    a[4..6].reverse();
                    a[6..8].reverse();

                    let mut builder = Builder::from_bytes(a);
                    if let Some(v) = version {
                        builder.set_version(v);
                    }
                    Ok(builder.build())
                }
                (None, None, None, Some(fields), None) => {
                    Err(PyErr::new::<NotImplementedError, &str>("Not implemented"))
                    // TODO: Handle errors
                    // TODO: Make this work
                    // let f = fields.to_object(py);
                    // let f = f.cast_as::<PyTuple>(py)?;
                    // let d1: u32 = f.get_item(0).downcast_ref::<PyInt>()?.extract::<u32>()?;
                    // let d2: u16 = f.get_item(1).downcast_ref::<PyInt>()?.extract::<u16>()?;
                    // let d3: u16 = f.get_item(2).downcast_ref::<PyInt>()?.extract::<u16>()?;
                    // let remaining_fields = f.split_from(3);
                    // let remaining_fields = remaining_fields.to_object(py);
                    // let remaining_fields = remaining_fields.cast_as::<PyTuple>(py)?;
                    // let d4 = remaining_fields
                    //     .iter()
                    //     .map(|x| x.downcast_ref::<PyInt>().unwrap().extract::<u8>().unwrap())
                    //     .collect::<Vec<u8>>();
                    //
                    // if let Ok(uuid) = Uuid::from_fields(d1, d2, d3, d4.as_slice()) {
                    //     Ok(uuid)
                    // } else {
                    //     // TODO: Provide more context to why the fields weren't parsed correctly.
                    //     Err(PyErr::new::<ValueError, &str>("fields error"))
                    // }
                }
                (None, None, None, None, Some(int)) => Ok(int.swap_bytes().into()),
                _ => Err(PyErr::new::<TypeError, &str>(
                    "one of the hex, bytes, bytes_le, fields, or int arguments must be given",
                )),
            };

            match result {
                Ok(handle) => {
                    obj.init(UUID { handle: handle });
                    Ok(())
                }
                Err(e) => {
                    obj.init(UUID {
                        handle: Uuid::nil(),
                    });
                    Err(e)
                }
            }
        }

        #[getter]
        fn int(&self) -> PyResult<u128> {
            Ok(byteorder::BigEndian::read_u128(self.handle.as_bytes()))
        }

        //#[getter]
        // TODO: Figure out how to make this a property
        fn bytes(&self, py: Python) -> PyResult<Py<PyBytes>> {
            let b = PyBytes::new(py, self.handle.as_bytes().as_ref());
            Ok(b)
        }

        //#[getter]
        // TODO: Figure out how to make this a property
        fn bytes_le(&self, py: Python) -> PyResult<Py<PyBytes>> {
            // Must clone or an error occurs
            let mut b = self.handle.as_bytes().clone();
            // Convert big endian to little endian
            b[0..4].reverse();
            b[4..6].reverse();
            b[6..8].reverse();
            let b = b.as_ref();
            let b = PyBytes::new(py, b);
            Ok(b)
        }

        #[getter]
        fn hex(&self) -> PyResult<String> {
            Ok(self
                .handle
                .to_simple()
                .encode_lower(&mut Uuid::encode_buffer())
                .to_string())
        }

        #[getter]
        fn urn(&self) -> PyResult<String> {
            Ok(self
                .handle
                .to_urn()
                .encode_lower(&mut Uuid::encode_buffer())
                .to_string())
        }

        #[getter]
        fn version(&self) -> PyResult<usize> {
            Ok(self.handle.get_version_num())
        }

        #[getter]
        fn variant(&self) -> PyResult<Option<&'static str>> {
            Ok(match self.handle.get_variant() {
                Some(Variant::NCS) => Some("reserved for NCS compatibility"),
                Some(Variant::RFC4122) => Some("specified in RFC 4122"),
                Some(Variant::Microsoft) => Some("reserved for Microsoft compatibility"),
                Some(Variant::Future) => Some("reserved for future definition"),
                _ => None,
            })
        }
    }

    impl<'p> FromPyObject<'p> for UUID {
        fn extract(obj: &'p PyAny) -> PyResult<Self> {
            let result: &UUID = obj.downcast_ref()?;
            Ok(UUID {
                handle: result.handle,
            })
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

        fn __richcmp__(&self, other: UUID, op: CompareOp) -> PyResult<bool> {
            match op {
                CompareOp::Eq => Ok(self.handle == other.handle),
                CompareOp::Ne => Ok(self.handle != other.handle),
                CompareOp::Lt => Ok(self.handle < other.handle),
                CompareOp::Gt => Ok(self.handle > other.handle),
                CompareOp::Le => Ok(self.handle <= other.handle),
                CompareOp::Ge => Ok(self.handle >= other.handle),
            }
        }

        fn __hash__(&self) -> PyResult<isize> {
            let mut s = DefaultHasher::new();
            self.handle.hash(&mut s);
            let result = s.finish() as isize;

            Ok(result)
        }
    }

    #[pyfn(m, "uuid3")]
    fn uuid3(namespace: &UUID, name: &PyBytes) -> PyResult<UUID> {
        Ok(UUID {
            handle: Uuid::new_v3(&namespace.handle, name.as_bytes()),
        })
    }

    #[pyfn(m, "uuid5")]
    fn uuid5(namespace: &UUID, name: &PyBytes) -> PyResult<UUID> {
        Ok(UUID {
            handle: Uuid::new_v5(&namespace.handle, name.as_bytes()),
        })
    }

    #[pyfn(m, "uuid4_bulk")]
    fn uuid4_bulk(py: Python, n: usize) -> Vec<UUID> {
        py.allow_threads(|| {
            iter::repeat_with(|| UUID {
                handle: Uuid::new_v4(),
            })
            .take(n)
            .collect()
        })
    }

    #[pyfn(m, "uuid4")]
    fn uuid4() -> UUID {
        UUID {
            handle: Uuid::new_v4(),
        }
    }

    m.add_class::<UUID>()?;

    Ok(())
}
