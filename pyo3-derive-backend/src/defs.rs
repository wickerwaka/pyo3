// Copyright (c) 2017-present PyO3 Project and Contributors
use crate::func::MethodProto;

pub struct Proto {
    pub name: &'static str,
    pub protocol_trait: &'static str,
    pub impl_trait: &'static str,
    pub methods: &'static [MethodProto],
    pub py_methods: &'static [PyMethod],
    pub default_impls: &'static [DefaultImpl],
}

pub struct PyMethod {
    pub name: &'static str,
    pub proto: &'static str,
}

pub struct DefaultImpl {
    pub name: &'static str,
    pub default: &'static str,
}

const OBJECT: Proto = Proto {
    name: "Object",
    protocol_trait: "pyo3::class::basic::PyObjectProtocol",
    impl_trait: "pyo3::class::basic::PyObjectProtocolImpl",
    methods: &[
        MethodProto::Binary {
            name: "__getattr__",
            arg: "Name",
            pyres: true,
            proto: "pyo3::class::basic::PyObjectGetAttrProtocol",
        },
        MethodProto::Ternary {
            name: "__setattr__",
            arg1: "Name",
            arg2: "Value",
            pyres: false,
            proto: "pyo3::class::basic::PyObjectSetAttrProtocol",
        },
        MethodProto::Binary {
            name: "__delattr__",
            arg: "Name",
            pyres: true,
            proto: "pyo3::class::basic::PyObjectDelAttrProtocol",
        },
        MethodProto::Unary {
            name: "__str__",
            pyres: true,
            proto: "pyo3::class::basic::PyObjectStrProtocol",
        },
        MethodProto::Unary {
            name: "__repr__",
            pyres: true,
            proto: "pyo3::class::basic::PyObjectReprProtocol",
        },
        MethodProto::Binary {
            name: "__format__",
            arg: "Format",
            pyres: true,
            proto: "pyo3::class::basic::PyObjectFormatProtocol",
        },
        MethodProto::Unary {
            name: "__hash__",
            pyres: false,
            proto: "pyo3::class::basic::PyObjectHashProtocol",
        },
        MethodProto::Unary {
            name: "__bytes__",
            pyres: true,
            proto: "pyo3::class::basic::PyObjectBytesProtocol",
        },
        MethodProto::Unary {
            name: "__bool__",
            pyres: false,
            proto: "pyo3::class::basic::PyObjectBoolProtocol",
        },
        MethodProto::Binary {
            name: "__richcmp__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::basic::PyObjectRichcmpProtocol",
        },
    ],
    py_methods: &[
        PyMethod {
            name: "__format__",
            proto: "pyo3::class::basic::FormatProtocolImpl",
        },
        PyMethod {
            name: "__bytes__",
            proto: "pyo3::class::basic::BytesProtocolImpl",
        },
        PyMethod {
            name: "__unicode__",
            proto: "pyo3::class::basic::UnicodeProtocolImpl",
        },
    ],
    default_impls: &[
        DefaultImpl {
            name: "__getattr__",
            default: "pyo3::class::basic::GetAttrProtocolImpl",
        },
        DefaultImpl {
            name: "__setattr__",
            default: "pyo3::class::basic::tp_setattro_impl::SetAttr",
        },
        DefaultImpl {
            name: "__delattr__",
            default: "pyo3::class::basic::tp_setattro_impl::DelAttr",
        },
        DefaultImpl {
            name: "__str__",
            default: "pyo3::class::basic::StrProtocolImpl",
        },
        DefaultImpl {
            name: "__repr__",
            default: "pyo3::class::basic::ReprProtocolImpl",
        },
        DefaultImpl {
            name: "__format__",
            default: "pyo3::class::basic::FormatProtocolImpl",
        },
        DefaultImpl {
            name: "__hash__",
            default: "pyo3::class::basic::HashProtocolImpl",
        },
        DefaultImpl {
            name: "__bytes__",
            default: "pyo3::class::basic::BytesProtocolImpl",
        },
        DefaultImpl {
            name: "__unicode__",
            default: "pyo3::class::basic::UnicodeProtocolImpl",
        },
        DefaultImpl {
            name: "__bool__",
            default: "pyo3::class::basic::BoolProtocolImpl",
        },
        DefaultImpl {
            name: "__richcmp__",
            default: "pyo3::class::basic::RichcmpProtocolImpl",
        },
    ],
};

const ASYNC: Proto = Proto {
    name: "Async",
    protocol_trait: "pyo3::class::pyasync::PyAsyncProtocol",
    impl_trait: "pyo3::class::pyasync::PyAsyncProtocolImpl",
    methods: &[
        MethodProto::Unary {
            name: "__await__",
            pyres: true,
            proto: "pyo3::class::pyasync::PyAsyncAwaitProtocol",
        },
        MethodProto::Unary {
            name: "__aiter__",
            pyres: true,
            proto: "pyo3::class::pyasync::PyAsyncAiterProtocol",
        },
        MethodProto::Unary {
            name: "__anext__",
            pyres: true,
            proto: "pyo3::class::pyasync::PyAsyncAnextProtocol",
        },
        MethodProto::Unary {
            name: "__aenter__",
            pyres: true,
            proto: "pyo3::class::pyasync::PyAsyncAenterProtocol",
        },
        MethodProto::Quaternary {
            name: "__aexit__",
            arg1: "ExcType",
            arg2: "ExcValue",
            arg3: "Traceback",
            proto: "pyo3::class::pyasync::PyAsyncAexitProtocol",
        },
    ],
    py_methods: &[
        PyMethod {
            name: "__aenter__",
            proto: "pyo3::class::pyasync::PyAsyncAenterProtocolImpl",
        },
        PyMethod {
            name: "__aexit__",
            proto: "pyo3::class::pyasync::PyAsyncAexitProtocolImpl",
        },
    ],
    default_impls: &[
        DefaultImpl {
            name: "__await__",
            default: "pyo3::class::pyasync::PyAsyncAwaitProtocol",
        },
        DefaultImpl {
            name: "__aiter__",
            default: "pyo3::class::pyasync::PyAsyncAiterProtocol",
        },
        DefaultImpl {
            name: "__anext__",
            default: "pyo3::class::pyasync::PyAsyncAnextProtocol",
        },
        DefaultImpl {
            name: "__aenter__",
            default: "pyo3::class::pyasync::PyAsyncAenterProtocol",
        },
        DefaultImpl {
            name: "__aexit__",
            default: "pyo3::class::pyasync::PyAsyncAexitProtocol",
        },
    ],
};

const BUFFER: Proto = Proto {
    name: "Buffer",
    protocol_trait: "pyo3::class::buffer::PyBufferProtocol",
    impl_trait: "pyo3::class::buffer::PyBufferProtocolImpl",
    methods: &[
        MethodProto::Unary {
            name: "bf_getbuffer",
            pyres: false,
            proto: "pyo3::class::buffer::PyBufferGetBufferProtocol",
        },
        MethodProto::Unary {
            name: "bf_releasebuffer",
            pyres: false,
            proto: "pyo3::class::buffer::PyBufferReleaseBufferProtocol",
        },
    ],
    py_methods: &[],
    default_impls: &[
        DefaultImpl {
            name: "bf_getbuffer",
            default: "pyo3::class::buffer::PyBufferGetBufferProtocolImpl",
        },
        DefaultImpl {
            name: "bf_releasebuffer",
            default: "pyo3::class::buffer::PyBufferReleaseBufferProtocolImpl",
        },
    ],
};

const CONTEXT: Proto = Proto {
    name: "Context",
    protocol_trait: "pyo3::class::context::PyContextProtocol",
    impl_trait: "pyo3::class::context::PyContextProtocolImpl",
    methods: &[
        MethodProto::Unary {
            name: "__enter__",
            pyres: true,
            proto: "pyo3::class::context::PyContextEnterProtocol",
        },
        MethodProto::Quaternary {
            name: "__exit__",
            arg1: "ExcType",
            arg2: "ExcValue",
            arg3: "Traceback",
            proto: "pyo3::class::context::PyContextExitProtocol",
        },
    ],
    py_methods: &[
        PyMethod {
            name: "__enter__",
            proto: "pyo3::class::context::PyContextEnterProtocolImpl",
        },
        PyMethod {
            name: "__exit__",
            proto: "pyo3::class::context::PyContextExitProtocolImpl",
        },
    ],
    default_impls: &[
        DefaultImpl {
            name: "__enter__",
            default: "pyo3::class::context::PyContextEnterProtocol",
        },
        DefaultImpl {
            name: "__exit__",
            default: "pyo3::class::context::PyContextExitProtocol",
        },
    ],
};

const GC: Proto = Proto {
    name: "GC",
    protocol_trait: "pyo3::class::gc::PyGCProtocol",
    impl_trait: "pyo3::class::gc::PyGCProtocolImpl",

    methods: &[
        MethodProto::Free {
            name: "__traverse__",
            proto: "pyo3::class::gc::PyGCTraverseProtocol",
        },
        MethodProto::Free {
            name: "__clear__",
            proto: "pyo3::class::gc::PyGCClearProtocol",
        },
    ],
    py_methods: &[],
    default_impls: &[
        DefaultImpl {
            name: "__traverse__",
            default: "pyo3::class::gc::PyGCTraverseProtocolImpl",
        },
        DefaultImpl {
            name: "__clear__",
            default: "pyo3::class::gc::PyGCClearProtocolImpl",
        },
    ],
};

const DESCR: Proto = Proto {
    name: "Descriptor",
    protocol_trait: "pyo3::class::descr::PyDescrProtocol",
    impl_trait: "pyo3::class::descr::PyDescrProtocolImpl",

    methods: &[
        MethodProto::Ternary {
            name: "__get__",
            arg1: "Inst",
            arg2: "Owner",
            pyres: true,
            proto: "pyo3::class::descr::PyDescrGetProtocol",
        },
        MethodProto::Ternary {
            name: "__set__",
            arg1: "Inst",
            arg2: "Value",
            pyres: true,
            proto: "pyo3::class::descr::PyDescrSetProtocol",
        },
        MethodProto::Binary {
            name: "__det__",
            arg: "Inst",
            pyres: false,
            proto: "pyo3::class::descr::PyDescrDelProtocol",
        },
        MethodProto::Binary {
            name: "__set_name__",
            arg: "Inst",
            pyres: false,
            proto: "pyo3::class::descr::PyDescrSetNameProtocol",
        },
    ],
    py_methods: &[
        PyMethod {
            name: "__del__",
            proto: "pyo3::class::context::PyDescrDelProtocolImpl",
        },
        PyMethod {
            name: "__set_name__",
            proto: "pyo3::class::context::PyDescrNameProtocolImpl",
        },
    ],
    default_impls: &[
        DefaultImpl {
            name: "__get__",
            default: "pyo3::class::descr::PyDescrGetProtocol",
        },
        DefaultImpl {
            name: "__set__",
            default: "pyo3::class::descr::PyDescrSetProtocol",
        },
        DefaultImpl {
            name: "__det__",
            default: "pyo3::class::descr::PyDescrDelProtocol",
        },
        DefaultImpl {
            name: "__set_name__",
            default: "pyo3::class::descr::PyDescrSetNameProtocol",
        },
    ],
};

const ITER: Proto = Proto {
    name: "Iter",
    protocol_trait: "pyo3::class::iter::PyIterProtocol",
    impl_trait: "pyo3::class::iter::PyIterProtocolImpl",

    py_methods: &[],
    methods: &[
        MethodProto::Unary {
            name: "__iter__",
            pyres: true,
            proto: "pyo3::class::iter::PyIterIterProtocol",
        },
        MethodProto::Unary {
            name: "__next__",
            pyres: true,
            proto: "pyo3::class::iter::PyIterNextProtocol",
        },
    ],
    default_impls: &[
        DefaultImpl {
            name: "__iter__",
            default: "pyo3::class::iter::PyIterIterProtocol",
        },
        DefaultImpl {
            name: "__next__",
            default: "pyo3::class::iter::PyIterNextProtocol",
        },
    ],
};

const MAPPING: Proto = Proto {
    name: "Mapping",
    protocol_trait: "pyo3::class::mapping::PyMappingProtocol",
    impl_trait: "pyo3::class::mapping::PyMappingProtocolImpl",

    methods: &[
        MethodProto::Unary {
            name: "__len__",
            pyres: false,
            proto: "pyo3::class::mapping::PyMappingLenProtocol",
        },
        MethodProto::Binary {
            name: "__getitem__",
            arg: "Key",
            pyres: true,
            proto: "pyo3::class::mapping::PyMappingGetItemProtocol",
        },
        MethodProto::Ternary {
            name: "__setitem__",
            arg1: "Key",
            arg2: "Value",
            pyres: false,
            proto: "pyo3::class::mapping::PyMappingSetItemProtocol",
        },
        MethodProto::Binary {
            name: "__delitem__",
            arg: "Key",
            pyres: false,
            proto: "pyo3::class::mapping::PyMappingDelItemProtocol",
        },
        MethodProto::Binary {
            name: "__contains__",
            arg: "Value",
            pyres: false,
            proto: "pyo3::class::mapping::PyMappingContainsProtocol",
        },
        MethodProto::Unary {
            name: "__reversed__",
            pyres: true,
            proto: "pyo3::class::mapping::PyMappingReversedProtocol",
        },
        MethodProto::Unary {
            name: "__iter__",
            pyres: true,
            proto: "pyo3::class::mapping::PyMappingIterProtocol",
        },
    ],
    py_methods: &[
        PyMethod {
            name: "__iter__",
            proto: "pyo3::class::mapping::PyMappingIterProtocolImpl",
        },
        PyMethod {
            name: "__contains__",
            proto: "pyo3::class::mapping::PyMappingContainsProtocolImpl",
        },
        PyMethod {
            name: "__reversed__",
            proto: "pyo3::class::mapping::PyMappingReversedProtocolImpl",
        },
    ],
    default_impls: &[
        DefaultImpl {
            name: "__len__",
            default: "pyo3::class::mapping::PyMappingLenProtocolImpl",
        },
        DefaultImpl {
            name: "__getitem__",
            default: "pyo3::class::mapping::PyMappingGetItemProtocolImpl",
        },
        DefaultImpl {
            name: "__setitem__",
            default: "pyo3::class::mapping::PyMappingSetItemProtocolImpl",
        },
        DefaultImpl {
            name: "__delitem__",
            default: "pyo3::class::mapping::PyMappingDelItemProtocolNotImpl",
        },
        DefaultImpl {
            name: "__contains__",
            default: "pyo3::class::mapping::PyMappingContainsProtocolImpl",
        },
        DefaultImpl {
            name: "__reversed__",
            default: "pyo3::class::mapping::PyMappingReversedProtocolImpl",
        },
        DefaultImpl {
            name: "__iter__",
            default: "pyo3::class::mapping::PyMappingIterProtocolImpl",
        },
    ],
};

const SEQ: Proto = Proto {
    name: "Sequence",
    protocol_trait: "pyo3::class::sequence::PySequenceProtocol",
    impl_trait: "pyo3::class::sequence::PySequenceProtocolImpl",

    methods: &[
        MethodProto::Unary {
            name: "__len__",
            pyres: false,
            proto: "pyo3::class::sequence::PySequenceLenProtocol",
        },
        MethodProto::Binary {
            name: "__getitem__",
            arg: "Index",
            pyres: true,
            proto: "pyo3::class::sequence::PySequenceGetItemProtocol",
        },
        MethodProto::Ternary {
            name: "__setitem__",
            arg1: "Index",
            arg2: "Value",
            pyres: false,
            proto: "pyo3::class::sequence::PySequenceSetItemProtocol",
        },
        MethodProto::Binary {
            name: "__delitem__",
            arg: "Index",
            pyres: false,
            proto: "pyo3::class::sequence::PySequenceDelItemProtocol",
        },
        MethodProto::Binary {
            name: "__contains__",
            arg: "Item",
            pyres: false,
            proto: "pyo3::class::sequence::PySequenceContainsProtocol",
        },
        MethodProto::Binary {
            name: "__concat__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::sequence::PySequenceConcatProtocol",
        },
        MethodProto::Binary {
            name: "__repeat__",
            arg: "Index",
            pyres: true,
            proto: "pyo3::class::sequence::PySequenceRepeatProtocol",
        },
        MethodProto::Binary {
            name: "__inplace_concat__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::sequence::PySequenceInplaceConcatProtocol",
        },
        MethodProto::Binary {
            name: "__inplace_repeat__",
            arg: "Index",
            pyres: true,
            proto: "pyo3::class::sequence::PySequenceInplaceRepeatProtocol",
        },
    ],
    py_methods: &[],
    default_impls: &[
        DefaultImpl {
            name: "__len__",
            default: "pyo3::class::sequence::PySequenceLenProtocolImpl",
        },
        DefaultImpl {
            name: "__getitem__",
            default: "pyo3::class::sequence::PySequenceGetItemProtocolImpl",
        },
        DefaultImpl {
            name: "__setitem__",
            default: "pyo3::class::sequence::PySequenceSetItemProtocolImpl",
        },
        DefaultImpl {
            name: "__delitem__",
            default: "pyo3::class::sequence::PySequenceDelItemProtocolImpl",
        },
        DefaultImpl {
            name: "__contains__",
            default: "pyo3::class::sequence::PySequenceContainsProtocolImpl",
        },
        DefaultImpl {
            name: "__concat__",
            default: "pyo3::class::sequence::PySequenceConcatProtocolImpl",
        },
        DefaultImpl {
            name: "__repeat__",
            default: "pyo3::class::sequence::PySequenceRepeatProtocolImpl",
        },
        DefaultImpl {
            name: "__inplace_concat__",
            default: "pyo3::class::sequence::PySequenceInplaceConcatProtocolImpl",
        },
        DefaultImpl {
            name: "__inplace_repeat__",
            default: "pyo3::class::sequence::PySequenceInplaceRepeatProtocolImpl",
        },
    ],
};

const NUM: Proto = Proto {
    name: "Number",
    protocol_trait: "pyo3::class::number::PyNumberProtocol",
    impl_trait: "pyo3::class::number::PyNumberProtocolImpl",

    methods: &[
        MethodProto::BinaryS {
            name: "__add__",
            arg1: "Left",
            arg2: "Right",
            pyres: true,
            proto: "pyo3::class::number::PyNumberAddProtocol",
        },
        MethodProto::BinaryS {
            name: "__sub__",
            arg1: "Left",
            arg2: "Right",
            pyres: true,
            proto: "pyo3::class::number::PyNumberSubProtocol",
        },
        MethodProto::BinaryS {
            name: "__mul__",
            arg1: "Left",
            arg2: "Right",
            pyres: true,
            proto: "pyo3::class::number::PyNumberMulProtocol",
        },
        MethodProto::BinaryS {
            name: "__matmul__",
            arg1: "Left",
            arg2: "Right",
            pyres: true,
            proto: "pyo3::class::number::PyNumberMatmulProtocol",
        },
        MethodProto::BinaryS {
            name: "__truediv__",
            arg1: "Left",
            arg2: "Right",
            pyres: true,
            proto: "pyo3::class::number::PyNumberTruedivProtocol",
        },
        MethodProto::BinaryS {
            name: "__floordiv__",
            arg1: "Left",
            arg2: "Right",
            pyres: true,
            proto: "pyo3::class::number::PyNumberFloordivProtocol",
        },
        MethodProto::BinaryS {
            name: "__mod__",
            arg1: "Left",
            arg2: "Right",
            pyres: true,
            proto: "pyo3::class::number::PyNumberModProtocol",
        },
        MethodProto::BinaryS {
            name: "__divmod__",
            arg1: "Left",
            arg2: "Right",
            pyres: true,
            proto: "pyo3::class::number::PyNumberDivmodProtocol",
        },
        MethodProto::TernaryS {
            name: "__pow__",
            arg1: "Left",
            arg2: "Right",
            arg3: "Modulo",
            pyres: true,
            proto: "pyo3::class::number::PyNumberPowProtocol",
        },
        MethodProto::BinaryS {
            name: "__lshift__",
            arg1: "Left",
            arg2: "Right",
            pyres: true,
            proto: "pyo3::class::number::PyNumberLShiftProtocol",
        },
        MethodProto::BinaryS {
            name: "__rshift__",
            arg1: "Left",
            arg2: "Right",
            pyres: true,
            proto: "pyo3::class::number::PyNumberRShiftProtocol",
        },
        MethodProto::BinaryS {
            name: "__and__",
            arg1: "Left",
            arg2: "Right",
            pyres: true,
            proto: "pyo3::class::number::PyNumberAndProtocol",
        },
        MethodProto::BinaryS {
            name: "__xor__",
            arg1: "Left",
            arg2: "Right",
            pyres: true,
            proto: "pyo3::class::number::PyNumberXorProtocol",
        },
        MethodProto::BinaryS {
            name: "__or__",
            arg1: "Left",
            arg2: "Right",
            pyres: true,
            proto: "pyo3::class::number::PyNumberOrProtocol",
        },
        MethodProto::Binary {
            name: "__radd__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::number::PyNumberRAddProtocol",
        },
        MethodProto::Binary {
            name: "__rsub__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::number::PyNumberRSubProtocol",
        },
        MethodProto::Binary {
            name: "__rmul__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::number::PyNumberRMulProtocol",
        },
        MethodProto::Binary {
            name: "__rmatmul__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::number::PyNumberRMatmulProtocol",
        },
        MethodProto::Binary {
            name: "__rtruediv__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::number::PyNumberRTruedivProtocol",
        },
        MethodProto::Binary {
            name: "__rfloordiv__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::number::PyNumberRFloordivProtocol",
        },
        MethodProto::Binary {
            name: "__rmod__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::number::PyNumberRModProtocol",
        },
        MethodProto::Binary {
            name: "__rdivmod__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::number::PyNumberRDivmodProtocol",
        },
        MethodProto::Ternary {
            name: "__rpow__",
            arg1: "Other",
            arg2: "Modulo",
            pyres: true,
            proto: "pyo3::class::number::PyNumberRPowProtocol",
        },
        MethodProto::Binary {
            name: "__rlshift__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::number::PyNumberRLShiftProtocol",
        },
        MethodProto::Binary {
            name: "__rrshift__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::number::PyNumberRRShiftProtocol",
        },
        MethodProto::Binary {
            name: "__rand__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::number::PyNumberRAndProtocol",
        },
        MethodProto::Binary {
            name: "__rxor__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::number::PyNumberRXorProtocol",
        },
        MethodProto::Binary {
            name: "__ror__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::number::PyNumberROrProtocol",
        },
        MethodProto::Binary {
            name: "__iadd__",
            arg: "Other",
            pyres: false,
            proto: "pyo3::class::number::PyNumberIAddProtocol",
        },
        MethodProto::Binary {
            name: "__isub__",
            arg: "Other",
            pyres: false,
            proto: "pyo3::class::number::PyNumberISubProtocol",
        },
        MethodProto::Binary {
            name: "__imul__",
            arg: "Other",
            pyres: false,
            proto: "pyo3::class::number::PyNumberIMulProtocol",
        },
        MethodProto::Binary {
            name: "__imatmul__",
            arg: "Other",
            pyres: false,
            proto: "pyo3::class::number::PyNumberIMatmulProtocol",
        },
        MethodProto::Binary {
            name: "__itruediv__",
            arg: "Other",
            pyres: false,
            proto: "pyo3::class::number::PyNumberITruedivProtocol",
        },
        MethodProto::Binary {
            name: "__ifloordiv__",
            arg: "Other",
            pyres: false,
            proto: "pyo3::class::number::PyNumberIFloordivProtocol",
        },
        MethodProto::Binary {
            name: "__imod__",
            arg: "Other",
            pyres: false,
            proto: "pyo3::class::number::PyNumberIModProtocol",
        },
        MethodProto::Ternary {
            name: "__ipow__",
            arg1: "Other",
            arg2: "Modulo",
            pyres: false,
            proto: "pyo3::class::number::PyNumberIPowProtocol",
        },
        MethodProto::Binary {
            name: "__ilshift__",
            arg: "Other",
            pyres: false,
            proto: "pyo3::class::number::PyNumberILShiftProtocol",
        },
        MethodProto::Binary {
            name: "__irshift__",
            arg: "Other",
            pyres: false,
            proto: "pyo3::class::number::PyNumberIRShiftProtocol",
        },
        MethodProto::Binary {
            name: "__iand__",
            arg: "Other",
            pyres: false,
            proto: "pyo3::class::number::PyNumberIAndProtocol",
        },
        MethodProto::Binary {
            name: "__ixor__",
            arg: "Other",
            pyres: false,
            proto: "pyo3::class::number::PyNumberIXorProtocol",
        },
        MethodProto::Binary {
            name: "__ior__",
            arg: "Other",
            pyres: false,
            proto: "pyo3::class::number::PyNumberIOrProtocol",
        },
        MethodProto::Unary {
            name: "__neg__",
            pyres: true,
            proto: "pyo3::class::number::PyNumberNegProtocol",
        },
        MethodProto::Unary {
            name: "__pos__",
            pyres: true,
            proto: "pyo3::class::number::PyNumberPosProtocol",
        },
        MethodProto::Unary {
            name: "__abs__",
            pyres: true,
            proto: "pyo3::class::number::PyNumberAbsProtocol",
        },
        MethodProto::Unary {
            name: "__invert__",
            pyres: true,
            proto: "pyo3::class::number::PyNumberInvertProtocol",
        },
        MethodProto::Unary {
            name: "__complex__",
            pyres: true,
            proto: "pyo3::class::number::PyNumberComplexProtocol",
        },
        MethodProto::Unary {
            name: "__int__",
            pyres: true,
            proto: "pyo3::class::number::PyNumberIntProtocol",
        },
        MethodProto::Unary {
            name: "__float__",
            pyres: true,
            proto: "pyo3::class::number::PyNumberFloatProtocol",
        },
        MethodProto::Unary {
            name: "__round__",
            pyres: true,
            proto: "pyo3::class::number::PyNumberRoundProtocol",
        },
        MethodProto::Unary {
            name: "__index__",
            pyres: true,
            proto: "pyo3::class::number::PyNumberIndexProtocol",
        },
    ],
    py_methods: &[
        PyMethod {
            name: "__radd__",
            proto: "pyo3::class::number::PyNumberRAddProtocolImpl",
        },
        PyMethod {
            name: "__rsub__",
            proto: "pyo3::class::number::PyNumberRSubProtocolImpl",
        },
        PyMethod {
            name: "__rmul__",
            proto: "pyo3::class::number::PyNumberRMulProtocolImpl",
        },
        PyMethod {
            name: "__rmatmul__",
            proto: "pyo3::class::number::PyNumberRMatmulProtocolImpl",
        },
        PyMethod {
            name: "__rtruediv__",
            proto: "pyo3::class::number::PyNumberRTruedivProtocolImpl",
        },
        PyMethod {
            name: "__rfloordiv__",
            proto: "pyo3::class::number::PyNumberRFloordivProtocolImpl",
        },
        PyMethod {
            name: "__rmod__",
            proto: "pyo3::class::number::PyNumberRModProtocolImpl",
        },
        PyMethod {
            name: "__rdivmod__",
            proto: "pyo3::class::number::PyNumberRDivmodProtocolImpl",
        },
        PyMethod {
            name: "__rpow__",
            proto: "pyo3::class::number::PyNumberRPowProtocolImpl",
        },
        PyMethod {
            name: "__rlshift__",
            proto: "pyo3::class::number::PyNumberRLShiftProtocolImpl",
        },
        PyMethod {
            name: "__rrshift__",
            proto: "pyo3::class::number::PyNumberRRShiftProtocolImpl",
        },
        PyMethod {
            name: "__rand__",
            proto: "pyo3::class::number::PyNumberRAndProtocolImpl",
        },
        PyMethod {
            name: "__rxor__",
            proto: "pyo3::class::number::PyNumberRXorProtocolImpl",
        },
        PyMethod {
            name: "__ror__",
            proto: "pyo3::class::number::PyNumberROrProtocolImpl",
        },
        PyMethod {
            name: "__complex__",
            proto: "pyo3::class::number::PyNumberComplexProtocolImpl",
        },
        PyMethod {
            name: "__round__",
            proto: "pyo3::class::number::PyNumberRoundProtocolImpl",
        },
    ],
    default_impls: &[
        DefaultImpl {
            name: "__add__",
            default: "pyo3::class::number::PyNumberAddProtocolImpl",
        },
        DefaultImpl {
            name: "__sub__",
            default: "pyo3::class::number::PyNumberSubProtocolImpl",
        },
        DefaultImpl {
            name: "__mul__",
            default: "pyo3::class::number::PyNumberMulProtocolImpl",
        },
        DefaultImpl {
            name: "__matmul__",
            default: "pyo3::class::number::PyNumberMatmulProtocolImpl",
        },
        DefaultImpl {
            name: "__truediv__",
            default: "pyo3::class::number::PyNumberTruedivProtocolImpl",
        },
        DefaultImpl {
            name: "__floordiv__",
            default: "pyo3::class::number::PyNumberFloordivProtocolImpl",
        },
        DefaultImpl {
            name: "__mod__",
            default: "pyo3::class::number::PyNumberModProtocolImpl",
        },
        DefaultImpl {
            name: "__divmod__",
            default: "pyo3::class::number::PyNumberDivmodProtocolImpl",
        },
        DefaultImpl {
            name: "__pow__",
            default: "pyo3::class::number::PyNumberPowProtocolImpl",
        },
        DefaultImpl {
            name: "__lshift__",
            default: "pyo3::class::number::PyNumberLShiftProtocolImpl",
        },
        DefaultImpl {
            name: "__rshift__",
            default: "pyo3::class::number::PyNumberRShiftProtocolImpl",
        },
        DefaultImpl {
            name: "__and__",
            default: "pyo3::class::number::PyNumberAndProtocolImpl",
        },
        DefaultImpl {
            name: "__xor__",
            default: "pyo3::class::number::PyNumberXorProtocolImpl",
        },
        DefaultImpl {
            name: "__or__",
            default: "pyo3::class::number::PyNumberOrProtocolImpl",
        },
        DefaultImpl {
            name: "__radd__",
            default: "pyo3::class::number::PyNumberRAddProtocolImpl",
        },
        DefaultImpl {
            name: "__rsub__",
            default: "pyo3::class::number::PyNumberRSubProtocolImpl",
        },
        DefaultImpl {
            name: "__rmul__",
            default: "pyo3::class::number::PyNumberRMulProtocolImpl",
        },
        DefaultImpl {
            name: "__rmatmul__",
            default: "pyo3::class::number::PyNumberRMatmulProtocolImpl",
        },
        DefaultImpl {
            name: "__rtruediv__",
            default: "pyo3::class::number::PyNumberRTruedivProtocolImpl",
        },
        DefaultImpl {
            name: "__rfloordiv__",
            default: "pyo3::class::number::PyNumberRFloordivProtocolImpl",
        },
        DefaultImpl {
            name: "__rmod__",
            default: "pyo3::class::number::PyNumberRModProtocolImpl",
        },
        DefaultImpl {
            name: "__rdivmod__",
            default: "pyo3::class::number::PyNumberRDivmodProtocolImpl",
        },
        DefaultImpl {
            name: "__rpow__",
            default: "pyo3::class::number::PyNumberRPowProtocolImpl",
        },
        DefaultImpl {
            name: "__rlshift__",
            default: "pyo3::class::number::PyNumberRLShiftProtocolImpl",
        },
        DefaultImpl {
            name: "__rrshift__",
            default: "pyo3::class::number::PyNumberRRShiftProtocolImpl",
        },
        DefaultImpl {
            name: "__rand__",
            default: "pyo3::class::number::PyNumberRAndProtocolImpl",
        },
        DefaultImpl {
            name: "__rxor__",
            default: "pyo3::class::number::PyNumberRXorProtocolImpl",
        },
        DefaultImpl {
            name: "__ror__",
            default: "pyo3::class::number::PyNumberROrProtocolImpl",
        },
        DefaultImpl {
            name: "__iadd__",
            default: "pyo3::class::number::PyNumberIAddProtocolImpl",
        },
        DefaultImpl {
            name: "__isub__",
            default: "pyo3::class::number::PyNumberISubProtocolImpl",
        },
        DefaultImpl {
            name: "__imul__",
            default: "pyo3::class::number::PyNumberIMulProtocolImpl",
        },
        DefaultImpl {
            name: "__imatmul__",
            default: "pyo3::class::number::PyNumberIMatmulProtocolImpl",
        },
        DefaultImpl {
            name: "__itruediv__",
            default: "pyo3::class::number::PyNumberITruedivProtocolImpl",
        },
        DefaultImpl {
            name: "__ifloordiv__",
            default: "pyo3::class::number::PyNumberIFloordivProtocolImpl",
        },
        DefaultImpl {
            name: "__imod__",
            default: "pyo3::class::number::PyNumberIModProtocolImpl",
        },
        DefaultImpl {
            name: "__ipow__",
            default: "pyo3::class::number::PyNumberIPowProtocolImpl",
        },
        DefaultImpl {
            name: "__ilshift__",
            default: "pyo3::class::number::PyNumberILShiftProtocolImpl",
        },
        DefaultImpl {
            name: "__irshift__",
            default: "pyo3::class::number::PyNumberIRShiftProtocolImpl",
        },
        DefaultImpl {
            name: "__iand__",
            default: "pyo3::class::number::PyNumberIAndProtocolImpl",
        },
        DefaultImpl {
            name: "__ixor__",
            default: "pyo3::class::number::PyNumberIXorProtocolImpl",
        },
        DefaultImpl {
            name: "__ior__",
            default: "pyo3::class::number::PyNumberIOrProtocolImpl",
        },
        DefaultImpl {
            name: "__neg__",
            default: "pyo3::class::number::PyNumberNegProtocolImpl",
        },
        DefaultImpl {
            name: "__pos__",
            default: "pyo3::class::number::PyNumberPosProtocolImpl",
        },
        DefaultImpl {
            name: "__abs__",
            default: "pyo3::class::number::PyNumberAbsProtocolImpl",
        },
        DefaultImpl {
            name: "__invert__",
            default: "pyo3::class::number::PyNumberInvertProtocolImpl",
        },
        DefaultImpl {
            name: "__complex__",
            default: "pyo3::class::number::PyNumberComplexProtocolImpl",
        },
        DefaultImpl {
            name: "__int__",
            default: "pyo3::class::number::PyNumberIntProtocolImpl",
        },
        DefaultImpl {
            name: "__float__",
            default: "pyo3::class::number::PyNumberFloatProtocolImpl",
        },
        DefaultImpl {
            name: "__round__",
            default: "pyo3::class::number::PyNumberRoundProtocolImpl",
        },
        DefaultImpl {
            name: "__index__",
            default: "pyo3::class::number::PyNumberIndexProtocolImpl",
        },
    ],
};

const PROTOCOLS: &[Proto] = &[
    OBJECT, ASYNC, MAPPING, ITER, CONTEXT, SEQ, NUM, DESCR, BUFFER, GC,
];

pub fn all_protocols() -> &'static [Proto] {
    PROTOCOLS
}

pub fn find_protocol(protocol_trait: &str) -> Option<&'static Proto> {
    PROTOCOLS
        .iter()
        .find(|p| p.protocol_trait.ends_with(protocol_trait))
}
