// Copyright (c) 2017-present PyO3 Project and Contributors

//! Represent Python Buffer protocol implementation
//!
//! For more information check [buffer protocol](https://docs.python.org/3/c-api/buffer.html)
//! c-api
use crate::callback::UnitCallbackConverter;
use crate::err::PyResult;
use crate::ffi;
use crate::type_object::PyTypeInfo;
use std::os::raw::c_int;

/// Buffer protocol interface
///
/// For more information check [buffer protocol](https://docs.python.org/3/c-api/buffer.html)
/// c-api
#[allow(unused_variables)]
pub trait PyBufferProtocol<'p>: PyTypeInfo {
    fn bf_getbuffer(&'p self, view: *mut ffi::Py_buffer, flags: c_int) -> Self::Result
    where
        Self: PyBufferGetBufferProtocol<'p>,
    {
        unimplemented!()
    }

    fn bf_releasebuffer(&'p self, view: *mut ffi::Py_buffer) -> Self::Result
    where
        Self: PyBufferReleaseBufferProtocol<'p>,
    {
        unimplemented!()
    }
}

pub trait PyBufferGetBufferProtocol<'p>: PyBufferProtocol<'p> {
    type Result: Into<PyResult<()>>;
}

pub trait PyBufferReleaseBufferProtocol<'p>: PyBufferProtocol<'p> {
    type Result: Into<PyResult<()>>;
}

#[doc(hidden)]
pub trait PyBufferProtocolImpl {
    fn tp_as_buffer() -> Option<ffi::PyBufferProcs> {
        None
    }
}

impl<'p, T> PyBufferProtocolImpl for T
where
    T: PyBufferProtocol<'p> + PyBufferGetBufferProtocolImpl,
{
    #[inline]
    #[allow(clippy::needless_update)] // For python 2 it's not useless
    fn tp_as_buffer() -> Option<ffi::PyBufferProcs> {
        Some(ffi::PyBufferProcs {
            bf_getbuffer: Self::cb_bf_getbuffer(),
            bf_releasebuffer: None,
            ..ffi::PyBufferProcs_INIT
        })
    }
}

pub trait PyBufferGetBufferProtocolImpl {
    fn cb_bf_getbuffer() -> Option<ffi::getbufferproc> {
        None
    }
}

impl<T> PyBufferGetBufferProtocolImpl for T
where
    T: for<'p> PyBufferGetBufferProtocol<'p>,
{
    #[inline]
    fn cb_bf_getbuffer() -> Option<ffi::getbufferproc> {
        unsafe extern "C" fn wrap<T>(
            slf: *mut ffi::PyObject,
            arg1: *mut ffi::Py_buffer,
            arg2: c_int,
        ) -> c_int
        where
            T: for<'p> PyBufferGetBufferProtocol<'p>,
        {
            let py = crate::Python::assume_gil_acquired();
            let _pool = crate::GILPool::new(py);
            let slf = py.mut_from_borrowed_ptr::<T>(slf);

            let result = slf.bf_getbuffer(arg1, arg2).into();
            crate::callback::cb_convert(UnitCallbackConverter, py, result)
        }
        Some(wrap::<T>)
    }
}
