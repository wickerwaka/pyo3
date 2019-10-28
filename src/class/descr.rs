// Copyright (c) 2017-present PyO3 Project and Contributors

//! Python Description Interface
//!
//! [Python information](
//! https://docs.python.org/3/reference/datamodel.html#implementing-descriptors)

use crate::callback::{PyObjectCallbackConverter, UnitCallbackConverter};
use crate::class::methods::PyMethodDef;
use crate::err::PyResult;
use crate::type_object::PyTypeInfo;
use crate::types::{PyAny, PyType};
use crate::FromPyObject;
use crate::{ffi, IntoPy, PyObject};
use std::os::raw::c_int;

/// Descriptor interface
#[allow(unused_variables)]
pub trait PyDescrProtocol<'p>: PyTypeInfo {
    fn __get__(&'p self, instance: &'p PyAny, owner: Option<&'p PyType>) -> Self::Result
    where
        Self: PyDescrGetProtocol<'p>,
    {
        unimplemented!()
    }

    fn __set__(&'p self, instance: &'p PyAny, value: &'p PyAny) -> Self::Result
    where
        Self: PyDescrSetProtocol<'p>,
    {
        unimplemented!()
    }

    fn __delete__(&'p self, instance: &'p PyAny) -> Self::Result
    where
        Self: PyDescrDeleteProtocol<'p>,
    {
        unimplemented!()
    }

    fn __set_name__(&'p self, instance: &'p PyAny) -> Self::Result
    where
        Self: PyDescrSetNameProtocol<'p>,
    {
        unimplemented!()
    }
}

pub trait PyDescrGetProtocol<'p>: PyDescrProtocol<'p> {
    type Inst: FromPyObject<'p>;
    type Owner: FromPyObject<'p>;
    type Success: IntoPy<PyObject>;
    type Result: Into<PyResult<Self::Success>>;
}

pub trait PyDescrSetProtocol<'p>: PyDescrProtocol<'p> {
    type Inst: FromPyObject<'p>;
    type Value: FromPyObject<'p>;
    type Result: Into<PyResult<()>>;
}

pub trait PyDescrDeleteProtocol<'p>: PyDescrProtocol<'p> {
    type Inst: FromPyObject<'p>;
    type Result: Into<PyResult<()>>;
}

pub trait PyDescrSetNameProtocol<'p>: PyDescrProtocol<'p> {
    type Inst: FromPyObject<'p>;
    type Result: Into<PyResult<()>>;
}

pub trait PyDescrGetProtocolImpl {
    fn tp_descr_get() -> Option<ffi::descrgetfunc> {
        None
    }
}

impl<T> PyDescrGetProtocolImpl for T
where
    T: for<'p> PyDescrGetProtocol<'p>,
{
    fn tp_descr_get() -> Option<ffi::descrgetfunc> {
        py_ternary_func!(
            PyDescrGetProtocol,
            T::__get__,
            T::Success,
            PyObjectCallbackConverter
        )
    }
}

pub trait PyDescrSetProtocolImpl {
    fn tp_descr_set() -> Option<ffi::descrsetfunc> {
        None
    }
}

impl<T> PyDescrSetProtocolImpl for T
where
    T: for<'p> PyDescrSetProtocol<'p>,
{
    fn tp_descr_set() -> Option<ffi::descrsetfunc> {
        py_ternary_func!(
            PyDescrSetProtocol,
            T::__set__,
            (),
            UnitCallbackConverter,
            c_int
        )
    }
}

trait PyDescrDelProtocolImpl {
    fn __del__() -> Option<PyMethodDef> {
        None
    }
}
impl<'p, T> PyDescrDelProtocolImpl for T where T: PyDescrProtocol<'p> {}

trait PyDescrSetNameProtocolImpl {
    fn __set_name__() -> Option<PyMethodDef> {
        None
    }
}
impl<'p, T> PyDescrSetNameProtocolImpl for T where T: PyDescrProtocol<'p> {}

#[doc(hidden)]
pub trait PyDescrProtocolImpl {
    fn methods() -> Vec<PyMethodDef> {
        Vec::new()
    }
    fn tp_as_descr(_type_object: &mut ffi::PyTypeObject) {}
}

impl<'p, T> PyDescrProtocolImpl for T
where
    T: PyDescrProtocol<'p> + PyDescrGetProtocolImpl + PyDescrSetProtocolImpl,
{
    fn methods() -> Vec<PyMethodDef> {
        Vec::new()
    }
    fn tp_as_descr(type_object: &mut ffi::PyTypeObject) {
        type_object.tp_descr_get = Self::tp_descr_get();
        type_object.tp_descr_set = Self::tp_descr_set();
    }
}
