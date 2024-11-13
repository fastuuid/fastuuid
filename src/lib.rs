#![deny(warnings)]
extern crate pyo3;
extern crate rand;
extern crate uuid;

use pyo3::class::basic::CompareOp;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyInt, PyTuple};
use rand::random;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::iter;
use uuid::{Builder, Context, Timestamp, Uuid, Variant, Version};

/// Generate a random node ID.
/// In hope to be compliant with RFC4122, we set the multicast bit to 1.
/// https://www.rfc-editor.org/rfc/rfc4122.html#section-4.5
///
/// The "multicast bit" of a MAC address is defined to be "the least
/// significant bit of the first octet". This works out to be the 41st bit
/// counting from 1 being the least significant bit, or 1<<40.
///
#[inline]
fn random_node_id() -> [u8; 6] {
    let bytes = random::<u64>().to_be_bytes();
    [
        bytes[2] | 0x01, // set multicast bit
        bytes[3],
        bytes[4],
        bytes[5],
        bytes[6],
        bytes[7],
    ]
}

#[pymodule]
mod fastuuid {
    use super::*;

    #[pyclass(subclass, freelist = 1000)]
    #[derive(Clone)]
    #[allow(clippy::upper_case_acronyms)]
    struct UUID {
        handle: Uuid,
    }

    #[pymethods]
    impl UUID {
        #[new]
        #[pyo3(signature = (hex=None, bytes=None, bytes_le=None, fields=None, int=None, version=None))]
        #[allow(clippy::too_many_arguments)]
        fn new(
            hex: Option<&str>,
            bytes: Option<&Bound<PyBytes>>,
            bytes_le: Option<&Bound<PyBytes>>,
            fields: Option<&Bound<PyTuple>>,
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
                _ => Err(PyErr::new::<PyValueError, &str>("illegal version number")),
            }?;

            let result: PyResult<Uuid> = match (hex, bytes, bytes_le, fields, int) {
                (Some(hex), None, None, None, None) => {
                    if let Ok(uuid) = Uuid::parse_str(hex) {
                        Ok(uuid)
                    } else {
                        // TODO: Provide more context to why the string wasn't parsed correctly.
                        Err(PyErr::new::<PyValueError, &str>(
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
                            Ok(builder.into_uuid())
                        }
                        Err(_) => Err(PyErr::new::<PyValueError, &str>(
                            "bytes is not a 16-char string",
                        )),
                    }
                }
                (None, None, Some(bytes_le), None, None) => {
                    if bytes_le.len()? != 16 {
                        Err(PyErr::new::<PyValueError, &str>(
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
                        Ok(builder.into_uuid())
                    }
                }
                (None, None, None, Some(fields), None) => {
                    let f = fields;
                    if f.len() != 6 {
                        Err(PyErr::new::<PyValueError, &str>("fields is not a 6-tuple"))
                    } else {
                        let time_low = match f.get_item(0)?.downcast::<PyInt>()?.extract::<u32>() {
                            Ok(time_low) => Ok(u128::from(time_low)),
                            Err(_) => Err(PyErr::new::<PyValueError, &str>(
                                "field 1 out of range (need a 32-bit value)",
                            )),
                        };

                        let time_low = time_low?;

                        let time_mid = match f.get_item(1)?.downcast::<PyInt>()?.extract::<u16>() {
                            Ok(time_mid) => Ok(u128::from(time_mid)),
                            Err(_) => Err(PyErr::new::<PyValueError, &str>(
                                "field 2 out of range (need a 16-bit value)",
                            )),
                        };

                        let time_mid = time_mid?;

                        let time_high_version =
                            match f.get_item(2)?.downcast::<PyInt>()?.extract::<u16>() {
                                Ok(time_high_version) => Ok(u128::from(time_high_version)),
                                Err(_) => Err(PyErr::new::<PyValueError, &str>(
                                    "field 3 out of range (need a 16-bit value)",
                                )),
                            };

                        let time_high_version = time_high_version?;

                        let clock_seq_hi_variant =
                            match f.get_item(3)?.downcast::<PyInt>()?.extract::<u8>() {
                                Ok(clock_seq_hi_variant) => Ok(u128::from(clock_seq_hi_variant)),
                                Err(_) => Err(PyErr::new::<PyValueError, &str>(
                                    "field 4 out of range (need a 8-bit value)",
                                )),
                            };

                        let clock_seq_hi_variant = clock_seq_hi_variant?;

                        let clock_seq_low =
                            match f.get_item(4)?.downcast::<PyInt>()?.extract::<u8>() {
                                Ok(clock_seq_low) => Ok(u128::from(clock_seq_low)),
                                Err(_) => Err(PyErr::new::<PyValueError, &str>(
                                    "field 5 out of range (need a 8-bit value)",
                                )),
                            };

                        let clock_seq_low = clock_seq_low?;

                        let node = f.get_item(5)?.downcast::<PyInt>()?.extract::<u128>()?;
                        if node >= (1 << 48) {
                            return Err(PyErr::new::<PyValueError, &str>(
                                "field 6 out of range (need a 48-bit value)",
                            ));
                        }

                        let clock_seq = clock_seq_hi_variant.wrapping_shl(8) | clock_seq_low;
                        let time_low = time_low.wrapping_shl(96);
                        let time_mid = time_mid.wrapping_shl(80);
                        let time_high_version = time_high_version.wrapping_shl(64);
                        let clock_seq = clock_seq.wrapping_shl(48);
                        let int = time_low | time_mid | time_high_version | clock_seq | node;
                        Ok(Uuid::from_u128(int))
                    }
                }
                (None, None, None, None, Some(int)) => Ok(Uuid::from_u128(int)),
                _ => Err(PyErr::new::<PyTypeError, &str>(
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
        fn bytes_le<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
            let mut b = *self.handle.as_bytes();
            // Convert big endian to little endian
            b[0..4].reverse();
            b[4..6].reverse();
            b[6..8].reverse();
            PyBytes::new_bound(py, &b)
        }

        #[getter]
        fn hex(&self) -> String {
            self.handle
                .simple()
                .encode_lower(&mut Uuid::encode_buffer())
                .to_string()
        }

        #[getter]
        fn urn(&self) -> String {
            self.handle
                .urn()
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
                Variant::NCS => Some("reserved for NCS compatibility"),
                Variant::RFC4122 => Some("specified in RFC 4122"),
                Variant::Microsoft => Some("reserved for Microsoft compatibility"),
                Variant::Future => Some("reserved for future definition"),
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
            let time_hi_version =
                time_hi_version.call_method_bound(py, "__and__", (0x0fff,), None)?;
            let time_hi_version =
                time_hi_version.call_method_bound(py, "__lshift__", (48,), None)?;
            let time_mid = self.time_mid().to_object(py);
            let time_mid = time_mid.call_method_bound(py, "__lshift__", (32,), None)?;
            let time_low = self.time_low().to_object(py);
            let time = time_hi_version;
            let time = time.call_method_bound(py, "__or__", (time_mid,), None)?;
            let time = time.call_method_bound(py, "__or__", (time_low,), None)?;
            Ok(time)
        }

        #[getter]
        fn node(&self) -> u64 {
            (self.int() & 0xffffffffffff) as u64
        }

        fn __str__(&self) -> PyResult<String> {
            Ok(self
                .handle
                .hyphenated()
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

        fn __int__(&self) -> PyResult<u128> {
            Ok(self.int())
        }

        pub fn __getstate__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
            Ok(PyBytes::new_bound(py, self.bytes()))
        }

        pub fn __setstate__(&mut self, py: Python, state: PyObject) -> PyResult<()> {
            let bytes_state = state.extract::<Bound<'_, PyBytes>>(py)?;
            let uuid_builder = Builder::from_slice(bytes_state.as_bytes());

            match uuid_builder {
                Ok(builder) => {
                    self.handle = builder.into_uuid();
                    Ok(())
                }
                Err(_) => Err(PyErr::new::<PyValueError, &str>(
                    "bytes is not a 16-char string",
                )),
            }
        }

        #[allow(clippy::type_complexity)]
        pub fn __getnewargs__(
            &self,
        ) -> PyResult<(
            Option<&str>,
            Option<&PyBytes>,
            Option<&PyBytes>,
            Option<&PyTuple>,
            Option<u128>,
            Option<u8>,
        )> {
            // hex, bytes, bytes_le, fields, int, version. The cheapest to compute valid call to new. The actual value
            // will be set by __setstate__ as part of unpickling.
            Ok((None, None, None, None, Some(0u128), None))
        }

        pub fn __copy__(&self) -> Self {
            self.clone()
        }

        pub fn __deepcopy__(&self, _memo: &Bound<PyDict>) -> Self {
            // fast bitwise copy instead of python's pickling process
            self.clone()
        }
    }

    #[pyfunction]
    fn uuid3(namespace: &UUID, name: &Bound<PyBytes>) -> UUID {
        UUID {
            handle: Uuid::new_v3(&namespace.handle, name.as_bytes()),
        }
    }

    #[pyfunction]
    fn uuid5(namespace: &UUID, name: &Bound<PyBytes>) -> UUID {
        UUID {
            handle: Uuid::new_v5(&namespace.handle, name.as_bytes()),
        }
    }

    #[pyfunction]
    fn uuid4_bulk(py: Python, n: usize) -> Vec<UUID> {
        py.allow_threads(|| {
            iter::repeat_with(|| UUID {
                handle: Uuid::new_v4(),
            })
            .take(n)
            .collect()
        })
    }

    #[pyfunction]
    fn uuid4_as_strings_bulk(py: Python, n: usize) -> Vec<String> {
        py.allow_threads(|| {
            iter::repeat_with(|| {
                (*Uuid::new_v4()
                    .simple()
                    .encode_lower(&mut Uuid::encode_buffer()))
                .to_string()
            })
            .take(n)
            .collect()
        })
    }

    #[pyfunction]
    fn uuid4() -> UUID {
        UUID {
            handle: Uuid::new_v4(),
        }
    }

    #[pyfunction]
    #[pyo3(signature = (node=None, clock_seq=None))]
    fn uuid1(py: Python, node: Option<u64>, clock_seq: Option<u16>) -> PyResult<UUID> {
        let node = match node {
            Some(node) => {
                let bytes = node.to_be_bytes();
                [bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]
            }
            _ => {
                // TODO: use a native implementation of getnode() instead of calling Python.
                // This is already quite fast, since the value is cached in Python.
                let py_uuid = PyModule::import_bound(py, "uuid")?;
                let node = py_uuid.getattr("getnode")?.call0()?.extract::<u64>()?;
                let bytes = node.to_be_bytes();
                [bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]
            }
        };
        Ok(match clock_seq {
            Some(clock_seq) => {
                // Timestamp::now(Context::new(clock_seq)) acquires an Atomic<u16>.
                // If we can avoid it, we can probably get another performance boost.
                let ts = Timestamp::now(Context::new(clock_seq));
                UUID {
                    handle: Uuid::new_v1(ts, &node),
                }
            }
            _ => UUID {
                handle: Uuid::now_v1(&node),
            },
        })
    }

    /// Fast path for uuid1 with a randomly generated MAC address.
    /// à la postgres' uuid extension.
    /// Further Reading:
    ///   - https://www.postgresql.org/docs/current/uuid-ossp.html
    ///   - https://www.edgedb.com/docs/stdlib/uuid#function::std::uuid_generate_v1mc
    ///   - https://supabase.com/blog/choosing-a-postgres-primary-key#uuidv1
    ///   -
    #[pyfunction]
    fn uuid_v1mc() -> UUID {
        UUID {
            handle: Uuid::now_v1(&random_node_id()),
        }
    }

    #[pyfunction]
    fn uuid7_bulk(py: Python, n: usize) -> Vec<UUID> {
        py.allow_threads(|| {
            iter::repeat_with(|| UUID {
                handle: Uuid::now_v7(),
            })
            .take(n)
            .collect()
        })
    }

    #[pyfunction]
    fn uuid7_as_strings_bulk(py: Python, n: usize) -> Vec<String> {
        py.allow_threads(|| {
            iter::repeat_with(|| {
                (*Uuid::now_v7()
                    .simple()
                    .encode_lower(&mut Uuid::encode_buffer()))
                .to_string()
            })
            .take(n)
            .collect()
        })
    }

    #[pyfunction]
    fn uuid7() -> UUID {
        UUID {
            handle: Uuid::now_v7(),
        }
    }
}
