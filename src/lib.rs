#![deny(warnings)]
extern crate pyo3;
extern crate uuid;

use pyo3::class::basic::CompareOp;
use pyo3::class::{PyNumberProtocol, PyObjectProtocol};
use pyo3::exceptions::{TypeError, ValueError};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyInt, PyTuple};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::iter;
use uuid::{Builder, Uuid, Variant, Version};

#[pymodule]
fn fastuuid(_py: Python, m: &PyModule) -> PyResult<()> {

    #[pyclass(freelist = 1000)]
    #[derive(Clone)]
    struct UUID {
        handle: Uuid,
    }

    #[pymethods]
    impl UUID {
        #[new]
        #[allow(clippy::too_many_arguments)]
        fn new(
            hex: Option<&str>,
            bytes: Option<&PyBytes>,
            bytes_le: Option<&PyBytes>,
            fields: Option<&PyTuple>,
            int: Option<u128>,
            version: Option<u8>,
        ) -> PyResult<Self> {
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
                    let builder = Builder::from_slice(bytes.as_bytes());

                    match builder {
                        Ok(mut builder) => {
                            if let Some(v) = version {
                                builder.set_version(v);
                            }
                            Ok(builder.build())
                        }
                        Err(_) => Err(PyErr::new::<ValueError, &str>(
                            "bytes is not a 16-char string",
                        )),
                    }
                }
                (None, None, Some(bytes_le), None, None) => {
                    if bytes_le.len()? != 16 {
                        Err(PyErr::new::<ValueError, &str>(
                            "bytes_le is not a 16-char string",
                        ))
                    } else {
                        let b = bytes_le.as_bytes();
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
                }
                (None, None, None, Some(fields), None) => {
                    let f = fields;
                    if f.len() != 6 {
                        Err(PyErr::new::<ValueError, &str>("fields is not a 6-tuple"))
                    } else {
                        let time_low = match f.get_item(0).downcast::<PyInt>()?.extract::<u32>() {
                            Ok(time_low) => Ok(u128::from(time_low)),
                            Err(_) => Err(PyErr::new::<ValueError, &str>(
                                "field 1 out of range (need a 32-bit value)",
                            )),
                        };

                        if let Err(e) = time_low {
                            return Err(e);
                        }
                        let time_low = time_low.unwrap();

                        let time_mid = match f.get_item(1).downcast::<PyInt>()?.extract::<u16>() {
                            Ok(time_mid) => Ok(u128::from(time_mid)),
                            Err(_) => Err(PyErr::new::<ValueError, &str>(
                                "field 2 out of range (need a 16-bit value)",
                            )),
                        };

                        if let Err(e) = time_mid {
                            return Err(e);
                        }
                        let time_mid = time_mid.unwrap();

                        let time_high_version =
                            match f.get_item(2).downcast::<PyInt>()?.extract::<u16>() {
                                Ok(time_high_version) => Ok(u128::from(time_high_version)),
                                Err(_) => Err(PyErr::new::<ValueError, &str>(
                                    "field 3 out of range (need a 16-bit value)",
                                )),
                            };

                        if let Err(e) = time_high_version {
                            return Err(e);
                        }
                        let time_high_version = time_high_version.unwrap();

                        let clock_seq_hi_variant =
                            match f.get_item(3).downcast::<PyInt>()?.extract::<u8>() {
                                Ok(clock_seq_hi_variant) => Ok(u128::from(clock_seq_hi_variant)),
                                Err(_) => Err(PyErr::new::<ValueError, &str>(
                                    "field 4 out of range (need a 8-bit value)",
                                )),
                            };

                        if let Err(e) = clock_seq_hi_variant {
                            return Err(e);
                        };
                        let clock_seq_hi_variant = clock_seq_hi_variant.unwrap();

                        let clock_seq_low = match f.get_item(4).downcast::<PyInt>()?.extract::<u8>()
                        {
                            Ok(clock_seq_low) => Ok(u128::from(clock_seq_low)),
                            Err(_) => Err(PyErr::new::<ValueError, &str>(
                                "field 5 out of range (need a 8-bit value)",
                            )),
                        };

                        if let Err(e) = clock_seq_low {
                            return Err(e);
                        };
                        let clock_seq_low = clock_seq_low.unwrap();

                        let node = f.get_item(5).downcast::<PyInt>()?.extract::<u128>()?;
                        if node >= (1 << 48) {
                            return Err(PyErr::new::<ValueError, &str>(
                                "field 6 out of range (need a 48-bit value)",
                            ));
                        }

                        let clock_seq = clock_seq_hi_variant.wrapping_shl(8) | clock_seq_low;
                        let time_low = time_low.wrapping_shl(96);
                        let time_mid = time_mid.wrapping_shl(80);
                        let time_high_version = time_high_version.wrapping_shl(64);
                        let clock_seq = clock_seq.wrapping_shl(48);
                        let node = node;
                        let int = time_low | time_mid | time_high_version | clock_seq | node;
                        Ok(Uuid::from_u128(int))
                    }
                }
                (None, None, None, None, Some(int)) => Ok(Uuid::from_u128(int)),
                _ => Err(PyErr::new::<TypeError, &str>(
                    "one of the hex, bytes, bytes_le, fields, or int arguments must be given",
                )),
            };

            match result {
                Ok(handle) => Ok(UUID { handle }),
                Err(e) => Err(e),
            }
        }

        #[getter]
        fn int(&self) -> u128 {
            self.handle.as_u128()
        }

        #[getter]
        fn bytes(&self) -> &[u8] {
            self.handle.as_bytes()
        }

        #[getter]
        fn bytes_le<'py>(&self, py: Python<'py>) -> &'py PyBytes {
            let mut b = *self.handle.as_bytes();
            // Convert big endian to little endian
            b[0..4].reverse();
            b[4..6].reverse();
            b[6..8].reverse();
            PyBytes::new(py, &b)
        }

        #[getter]
        fn hex(&self) -> String {
            self.handle
                .to_simple()
                .encode_lower(&mut Uuid::encode_buffer())
                .to_string()
        }

        #[getter]
        fn urn(&self) -> String {
            self.handle
                .to_urn()
                .encode_lower(&mut Uuid::encode_buffer())
                .to_string()
        }

        #[getter]
        fn version(&self) -> usize {
            self.handle.get_version_num()
        }

        #[getter]
        fn variant(&self) -> Option<&'static str> {
            match self.handle.get_variant() {
                Some(Variant::NCS) => Some("reserved for NCS compatibility"),
                Some(Variant::RFC4122) => Some("specified in RFC 4122"),
                Some(Variant::Microsoft) => Some("reserved for Microsoft compatibility"),
                Some(Variant::Future) => Some("reserved for future definition"),
                _ => None,
            }
        }

        #[getter]
        fn fields(&self) -> (u32, u16, u16, u8, u8, u64) {
            (
                self.time_low(),
                self.time_mid(),
                self.time_hi_version(),
                self.clock_seq_hi_variant(),
                self.clock_seq_low(),
                self.node(),
            )
        }

        #[getter]
        fn time_low(&self) -> u32 {
            let int = self.int();
            int.wrapping_shr(96) as u32
        }

        #[getter]
        fn time_mid(&self) -> u16 {
            let int = self.int();
            (int.wrapping_shr(80) & 0xffff) as u16
        }

        #[getter]
        fn time_hi_version(&self) -> u16 {
            let int = self.int();
            (int.wrapping_shr(64) & 0xffff) as u16
        }

        #[getter]
        fn clock_seq_hi_variant(&self) -> u8 {
            let int = self.int();
            (int.wrapping_shr(56) & 0xff) as u8
        }

        #[getter]
        fn clock_seq_low(&self) -> u8 {
            let int = self.int();
            (int.wrapping_shr(48) & 0xff) as u8
        }

        #[getter]
        fn time(&self, py: Python) -> PyResult<PyObject> {
            // We use Python's API since the result is much larger than u128.
            let time_hi_version = self.time_hi_version().to_object(py);
            let time_hi_version = time_hi_version.call_method(py, "__and__", (0x0fff,), None)?;
            let time_hi_version = time_hi_version.call_method(py, "__lshift__", (48,), None)?;
            let time_mid = self.time_mid().to_object(py);
            let time_mid = time_mid.call_method(py, "__lshift__", (32,), None)?;
            let time_low = self.time_low().to_object(py);
            let time = time_hi_version;
            let time = time.call_method(py, "__or__", (time_mid,), None)?;
            let time = time.call_method(py, "__or__", (time_low,), None)?;
            Ok(time)
        }

        #[getter]
        fn node(&self) -> u64 {
            (self.int() & 0xffffffffffff) as u64
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

    #[pyproto]
    impl<'p> PyNumberProtocol<'p> for UUID {
        fn __int__(&self) -> PyResult<u128> {
            Ok(self.int())
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

    #[pyfn(m, "uuid4_as_strings_bulk")]
    fn uuid4_as_strings_bulk(py: Python, n: usize) -> Vec<String> {
        py.allow_threads(|| {
            iter::repeat_with(|| {
                (*Uuid::new_v4()
                    .to_simple()
                    .encode_lower(&mut Uuid::encode_buffer()))
                .to_string()
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
