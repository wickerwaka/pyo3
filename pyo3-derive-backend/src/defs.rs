// Copyright (c) 2017-present PyO3 Project and Contributors
use crate::func::MethodProto;

pub struct Proto {
    pub name: &'static str,
    pub protocol_trait: &'static str,
    pub impl_trait: &'static str,
    pub methods: &'static [MethodProto],
    pub py_methods: &'static [PyMethod],
}

pub struct PyMethod {
    pub name: &'static str,
    pub proto: &'static str,
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
            default: "pyo3::class::basic::PyObjectGetAttrProtocol",
        },
        MethodProto::Ternary {
            name: "__setattr__",
            arg1: "Name",
            arg2: "Value",
            pyres: false,
            proto: "pyo3::class::basic::PyObjectSetAttrProtocol",
            default: "pyo3::class::basic::PyObjectSetAttrProtocol",
        },
        MethodProto::Binary {
            name: "__delattr__",
            arg: "Name",
            pyres: true,
            proto: "pyo3::class::basic::PyObjectDelAttrProtocol",
            default: "pyo3::class::basic::PyObjectDelAttrProtocol",
        },
        MethodProto::Unary {
            name: "__str__",
            pyres: true,
            proto: "pyo3::class::basic::PyObjectStrProtocol",
            default: "pyo3::class::basic::PyObjectStrProtocol",
        },
        MethodProto::Unary {
            name: "__repr__",
            pyres: true,
            proto: "pyo3::class::basic::PyObjectReprProtocol",
            default: "pyo3::class::basic::PyObjectReprProtocol",
        },
        MethodProto::Binary {
            name: "__format__",
            arg: "Format",
            pyres: true,
            proto: "pyo3::class::basic::PyObjectFormatProtocol",
            default: "pyo3::class::basic::PyObjectFormatProtocol",
        },
        MethodProto::Unary {
            name: "__hash__",
            pyres: false,
            proto: "pyo3::class::basic::PyObjectHashProtocol",
            default: "pyo3::class::basic::PyObjectHashProtocol",
        },
        MethodProto::Unary {
            name: "__bytes__",
            pyres: true,
            proto: "pyo3::class::basic::PyObjectBytesProtocol",
            default: "pyo3::class::basic::PyObjectBytesProtocol",
        },
        MethodProto::Unary {
            name: "__bool__",
            pyres: false,
            proto: "pyo3::class::basic::PyObjectBoolProtocol",
            default: "pyo3::class::basic::PyObjectBoolProtocol",
        },
        MethodProto::Binary {
            name: "__richcmp__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::basic::PyObjectRichcmpProtocol",
            default: "pyo3::class::basic::PyObjectRichcmpProtocol",
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
            default: "pyo3::class::pyasync::PyAsyncAwaitProtocol",
        },
        MethodProto::Unary {
            name: "__aiter__",
            pyres: true,
            proto: "pyo3::class::pyasync::PyAsyncAiterProtocol",
            default: "pyo3::class::pyasync::PyAsyncAiterProtocol",
        },
        MethodProto::Unary {
            name: "__anext__",
            pyres: true,
            proto: "pyo3::class::pyasync::PyAsyncAnextProtocol",
            default: "pyo3::class::pyasync::PyAsyncAnextProtocol",
        },
        MethodProto::Unary {
            name: "__aenter__",
            pyres: true,
            proto: "pyo3::class::pyasync::PyAsyncAenterProtocol",
            default: "pyo3::class::pyasync::PyAsyncAenterProtocol",
        },
        MethodProto::Quaternary {
            name: "__aexit__",
            arg1: "ExcType",
            arg2: "ExcValue",
            arg3: "Traceback",
            proto: "pyo3::class::pyasync::PyAsyncAexitProtocol",
            default: "pyo3::class::pyasync::PyAsyncAexitProtocol",
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
            default: "pyo3::class::buffer::PyBufferGetBufferProtocolImpl",
        },
        MethodProto::Unary {
            name: "bf_releasebuffer",
            pyres: false,
            proto: "pyo3::class::buffer::PyBufferReleaseBufferProtocol",
            default: "pyo3::class::buffer::PyBufferReleaseBufferProtocolImpl",
        },
    ],
    py_methods: &[],
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
            default: "pyo3::class::context::PyContextEnterProtocol",
        },
        MethodProto::Quaternary {
            name: "__exit__",
            arg1: "ExcType",
            arg2: "ExcValue",
            arg3: "Traceback",
            proto: "pyo3::class::context::PyContextExitProtocol",
            default: "pyo3::class::context::PyContextExitProtocol",
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
};

const GC: Proto = Proto {
    name: "GC",
    protocol_trait: "pyo3::class::gc::PyGCProtocol",
    impl_trait: "pyo3::class::gc::PyGCProtocolImpl",

    methods: &[
        MethodProto::Free {
            name: "__traverse__",
            proto: "pyo3::class::gc::PyGCTraverseProtocol",
            default: "pyo3::class::gc::PyGCTraverseProtocolImpl",
        },
        MethodProto::Free {
            name: "__clear__",
            proto: "pyo3::class::gc::PyGCClearProtocol",
            default: "pyo3::class::gc::PyGCClearProtocolImpl",
        },
    ],
    py_methods: &[],
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
            default: "pyo3::class::descr::PyDescrGetProtocol",
        },
        MethodProto::Ternary {
            name: "__set__",
            arg1: "Inst",
            arg2: "Value",
            pyres: true,
            proto: "pyo3::class::descr::PyDescrSetProtocol",
            default: "pyo3::class::descr::PyDescrSetProtocol",
        },
        MethodProto::Binary {
            name: "__det__",
            arg: "Inst",
            pyres: false,
            proto: "pyo3::class::descr::PyDescrDelProtocol",
            default: "pyo3::class::descr::PyDescrDelProtocol",
        },
        MethodProto::Binary {
            name: "__set_name__",
            arg: "Inst",
            pyres: false,
            proto: "pyo3::class::descr::PyDescrSetNameProtocol",
            default: "pyo3::class::descr::PyDescrSetNameProtocol",
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
            default: "pyo3::class::iter::PyIterIterProtocol",
        },
        MethodProto::Unary {
            name: "__next__",
            pyres: true,
            proto: "pyo3::class::iter::PyIterNextProtocol",
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
            default: "pyo3::class::mapping::PyMappingLenProtocol",
        },
        MethodProto::Binary {
            name: "__getitem__",
            arg: "Key",
            pyres: true,
            proto: "pyo3::class::mapping::PyMappingGetItemProtocol",
            default: "pyo3::class::mapping::PyMappingGetItemProtocol",
        },
        MethodProto::Ternary {
            name: "__setitem__",
            arg1: "Key",
            arg2: "Value",
            pyres: false,
            proto: "pyo3::class::mapping::PyMappingSetItemProtocol",
            default: "pyo3::class::mapping::PyMappingSetItemProtocol",
        },
        MethodProto::Binary {
            name: "__delitem__",
            arg: "Key",
            pyres: false,
            proto: "pyo3::class::mapping::PyMappingDelItemProtocol",
            default: "pyo3::class::mapping::PyMappingDelItemProtocol",
        },
        MethodProto::Binary {
            name: "__contains__",
            arg: "Value",
            pyres: false,
            proto: "pyo3::class::mapping::PyMappingContainsProtocol",
            default: "pyo3::class::mapping::PyMappingContainsProtocol",
        },
        MethodProto::Unary {
            name: "__reversed__",
            pyres: true,
            proto: "pyo3::class::mapping::PyMappingReversedProtocol",
            default: "pyo3::class::mapping::PyMappingReversedProtocol",
        },
        MethodProto::Unary {
            name: "__iter__",
            pyres: true,
            proto: "pyo3::class::mapping::PyMappingIterProtocol",
            default: "pyo3::class::mapping::PyMappingIterProtocol",
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
            default: "pyo3::class::sequence::PySequenceLenProtocolImpl",
        },
        MethodProto::Binary {
            name: "__getitem__",
            arg: "Index",
            pyres: true,
            proto: "pyo3::class::sequence::PySequenceGetItemProtocol",
            default: "pyo3::class::sequence::PySequenceGetItemProtocolImpl",
        },
        MethodProto::Ternary {
            name: "__setitem__",
            arg1: "Index",
            arg2: "Value",
            pyres: false,
            proto: "pyo3::class::sequence::PySequenceSetItemProtocol",
            default: "pyo3::class::sequence::PySequenceSetItemProtocolImpl",
        },
        MethodProto::Binary {
            name: "__delitem__",
            arg: "Index",
            pyres: false,
            proto: "pyo3::class::sequence::PySequenceDelItemProtocol",
            default: "pyo3::class::sequence::PySequenceDelItemProtocolImpl",
        },
        MethodProto::Binary {
            name: "__contains__",
            arg: "Item",
            pyres: false,
            proto: "pyo3::class::sequence::PySequenceContainsProtocol",
            default: "pyo3::class::sequence::PySequenceContainsProtocolImpl",
        },
        MethodProto::Binary {
            name: "__concat__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::sequence::PySequenceConcatProtocol",
            default: "pyo3::class::sequence::PySequenceConcatProtocolImpl",
        },
        MethodProto::Binary {
            name: "__repeat__",
            arg: "Index",
            pyres: true,
            proto: "pyo3::class::sequence::PySequenceRepeatProtocol",
            default: "pyo3::class::sequence::PySequenceRepeatProtocolImpl",
        },
        MethodProto::Binary {
            name: "__inplace_concat__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::sequence::PySequenceInplaceConcatProtocol",
            default: "pyo3::class::sequence::PySequenceInplaceConcatProtocolImpl",
        },
        MethodProto::Binary {
            name: "__inplace_repeat__",
            arg: "Index",
            pyres: true,
            proto: "pyo3::class::sequence::PySequenceInplaceRepeatProtocol",
            default: "pyo3::class::sequence::PySequenceInplaceRepeatProtocolImpl",
        },
    ],
    py_methods: &[],
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
            default: "pyo3::class::number::PyNumberAddProtocol",
        },
        MethodProto::BinaryS {
            name: "__sub__",
            arg1: "Left",
            arg2: "Right",
            pyres: true,
            proto: "pyo3::class::number::PyNumberSubProtocol",
            default: "pyo3::class::number::PyNumberSubProtocol",
        },
        MethodProto::BinaryS {
            name: "__mul__",
            arg1: "Left",
            arg2: "Right",
            pyres: true,
            proto: "pyo3::class::number::PyNumberMulProtocol",
            default: "pyo3::class::number::PyNumberMulProtocol",
        },
        MethodProto::BinaryS {
            name: "__matmul__",
            arg1: "Left",
            arg2: "Right",
            pyres: true,
            proto: "pyo3::class::number::PyNumberMatmulProtocol",
            default: "pyo3::class::number::PyNumberMatmulProtocol",
        },
        MethodProto::BinaryS {
            name: "__truediv__",
            arg1: "Left",
            arg2: "Right",
            pyres: true,
            proto: "pyo3::class::number::PyNumberTruedivProtocol",
            default: "pyo3::class::number::PyNumberTruedivProtocol",
        },
        MethodProto::BinaryS {
            name: "__floordiv__",
            arg1: "Left",
            arg2: "Right",
            pyres: true,
            proto: "pyo3::class::number::PyNumberFloordivProtocol",
            default: "pyo3::class::number::PyNumberFloordivProtocol",
        },
        MethodProto::BinaryS {
            name: "__mod__",
            arg1: "Left",
            arg2: "Right",
            pyres: true,
            proto: "pyo3::class::number::PyNumberModProtocol",
            default: "pyo3::class::number::PyNumberModProtocol",
        },
        MethodProto::BinaryS {
            name: "__divmod__",
            arg1: "Left",
            arg2: "Right",
            pyres: true,
            proto: "pyo3::class::number::PyNumberDivmodProtocol",
            default: "pyo3::class::number::PyNumberDivmodProtocol",
        },
        MethodProto::TernaryS {
            name: "__pow__",
            arg1: "Left",
            arg2: "Right",
            arg3: "Modulo",
            pyres: true,
            proto: "pyo3::class::number::PyNumberPowProtocol",
            default: "pyo3::class::number::PyNumberPowProtocol",
        },
        MethodProto::BinaryS {
            name: "__lshift__",
            arg1: "Left",
            arg2: "Right",
            pyres: true,
            proto: "pyo3::class::number::PyNumberLShiftProtocol",
            default: "pyo3::class::number::PyNumberLShiftProtocol",
        },
        MethodProto::BinaryS {
            name: "__rshift__",
            arg1: "Left",
            arg2: "Right",
            pyres: true,
            proto: "pyo3::class::number::PyNumberRShiftProtocol",
            default: "pyo3::class::number::PyNumberRShiftProtocol",
        },
        MethodProto::BinaryS {
            name: "__and__",
            arg1: "Left",
            arg2: "Right",
            pyres: true,
            proto: "pyo3::class::number::PyNumberAndProtocol",
            default: "pyo3::class::number::PyNumberAndProtocol",
        },
        MethodProto::BinaryS {
            name: "__xor__",
            arg1: "Left",
            arg2: "Right",
            pyres: true,
            proto: "pyo3::class::number::PyNumberXorProtocol",
            default: "pyo3::class::number::PyNumberXorProtocol",
        },
        MethodProto::BinaryS {
            name: "__or__",
            arg1: "Left",
            arg2: "Right",
            pyres: true,
            proto: "pyo3::class::number::PyNumberOrProtocol",
            default: "pyo3::class::number::PyNumberOrProtocol",
        },
        MethodProto::Binary {
            name: "__radd__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::number::PyNumberRAddProtocol",
            default: "pyo3::class::number::PyNumberRAddProtocol",
        },
        MethodProto::Binary {
            name: "__rsub__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::number::PyNumberRSubProtocol",
            default: "pyo3::class::number::PyNumberRSubProtocol",
        },
        MethodProto::Binary {
            name: "__rmul__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::number::PyNumberRMulProtocol",
            default: "pyo3::class::number::PyNumberRMulProtocol",
        },
        MethodProto::Binary {
            name: "__rmatmul__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::number::PyNumberRMatmulProtocol",
            default: "pyo3::class::number::PyNumberRMatmulProtocol",
        },
        MethodProto::Binary {
            name: "__rtruediv__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::number::PyNumberRTruedivProtocol",
            default: "pyo3::class::number::PyNumberRTruedivProtocol",
        },
        MethodProto::Binary {
            name: "__rfloordiv__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::number::PyNumberRFloordivProtocol",
            default: "pyo3::class::number::PyNumberRFloordivProtocol",
        },
        MethodProto::Binary {
            name: "__rmod__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::number::PyNumberRModProtocol",
            default: "pyo3::class::number::PyNumberRModProtocol",
        },
        MethodProto::Binary {
            name: "__rdivmod__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::number::PyNumberRDivmodProtocol",
            default: "pyo3::class::number::PyNumberRDivmodProtocol",
        },
        MethodProto::Ternary {
            name: "__rpow__",
            arg1: "Other",
            arg2: "Modulo",
            pyres: true,
            proto: "pyo3::class::number::PyNumberRPowProtocol",
            default: "pyo3::class::number::PyNumberRPowProtocol",
        },
        MethodProto::Binary {
            name: "__rlshift__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::number::PyNumberRLShiftProtocol",
            default: "pyo3::class::number::PyNumberRLShiftProtocol",
        },
        MethodProto::Binary {
            name: "__rrshift__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::number::PyNumberRRShiftProtocol",
            default: "pyo3::class::number::PyNumberRRShiftProtocol",
        },
        MethodProto::Binary {
            name: "__rand__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::number::PyNumberRAndProtocol",
            default: "pyo3::class::number::PyNumberRAndProtocol",
        },
        MethodProto::Binary {
            name: "__rxor__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::number::PyNumberRXorProtocol",
            default: "pyo3::class::number::PyNumberRXorProtocol",
        },
        MethodProto::Binary {
            name: "__ror__",
            arg: "Other",
            pyres: true,
            proto: "pyo3::class::number::PyNumberROrProtocol",
            default: "pyo3::class::number::PyNumberROrProtocol",
        },
        MethodProto::Binary {
            name: "__iadd__",
            arg: "Other",
            pyres: false,
            proto: "pyo3::class::number::PyNumberIAddProtocol",
            default: "pyo3::class::number::PyNumberIAddProtocol",
        },
        MethodProto::Binary {
            name: "__isub__",
            arg: "Other",
            pyres: false,
            proto: "pyo3::class::number::PyNumberISubProtocol",
            default: "pyo3::class::number::PyNumberISubProtocol",
        },
        MethodProto::Binary {
            name: "__imul__",
            arg: "Other",
            pyres: false,
            proto: "pyo3::class::number::PyNumberIMulProtocol",
            default: "pyo3::class::number::PyNumberIMulProtocol",
        },
        MethodProto::Binary {
            name: "__imatmul__",
            arg: "Other",
            pyres: false,
            proto: "pyo3::class::number::PyNumberIMatmulProtocol",
            default: "pyo3::class::number::PyNumberIMatmulProtocol",
        },
        MethodProto::Binary {
            name: "__itruediv__",
            arg: "Other",
            pyres: false,
            proto: "pyo3::class::number::PyNumberITruedivProtocol",
            default: "pyo3::class::number::PyNumberITruedivProtocol",
        },
        MethodProto::Binary {
            name: "__ifloordiv__",
            arg: "Other",
            pyres: false,
            proto: "pyo3::class::number::PyNumberIFloordivProtocol",
            default: "pyo3::class::number::PyNumberIFloordivProtocol",
        },
        MethodProto::Binary {
            name: "__imod__",
            arg: "Other",
            pyres: false,
            proto: "pyo3::class::number::PyNumberIModProtocol",
            default: "pyo3::class::number::PyNumberIModProtocol",
        },
        MethodProto::Ternary {
            name: "__ipow__",
            arg1: "Other",
            arg2: "Modulo",
            pyres: false,
            proto: "pyo3::class::number::PyNumberIPowProtocol",
            default: "pyo3::class::number::PyNumberIPowProtocol",
        },
        MethodProto::Binary {
            name: "__ilshift__",
            arg: "Other",
            pyres: false,
            proto: "pyo3::class::number::PyNumberILShiftProtocol",
            default: "pyo3::class::number::PyNumberILShiftProtocol",
        },
        MethodProto::Binary {
            name: "__irshift__",
            arg: "Other",
            pyres: false,
            proto: "pyo3::class::number::PyNumberIRShiftProtocol",
            default: "pyo3::class::number::PyNumberIRShiftProtocol",
        },
        MethodProto::Binary {
            name: "__iand__",
            arg: "Other",
            pyres: false,
            proto: "pyo3::class::number::PyNumberIAndProtocol",
            default: "pyo3::class::number::PyNumberIAndProtocol",
        },
        MethodProto::Binary {
            name: "__ixor__",
            arg: "Other",
            pyres: false,
            proto: "pyo3::class::number::PyNumberIXorProtocol",
            default: "pyo3::class::number::PyNumberIXorProtocol",
        },
        MethodProto::Binary {
            name: "__ior__",
            arg: "Other",
            pyres: false,
            proto: "pyo3::class::number::PyNumberIOrProtocol",
            default: "pyo3::class::number::PyNumberIOrProtocol",
        },
        MethodProto::Unary {
            name: "__neg__",
            pyres: true,
            proto: "pyo3::class::number::PyNumberNegProtocol",
            default: "pyo3::class::number::PyNumberNegProtocol",
        },
        MethodProto::Unary {
            name: "__pos__",
            pyres: true,
            proto: "pyo3::class::number::PyNumberPosProtocol",
            default: "pyo3::class::number::PyNumberPosProtocol",
        },
        MethodProto::Unary {
            name: "__abs__",
            pyres: true,
            proto: "pyo3::class::number::PyNumberAbsProtocol",
            default: "pyo3::class::number::PyNumberAbsProtocol",
        },
        MethodProto::Unary {
            name: "__invert__",
            pyres: true,
            proto: "pyo3::class::number::PyNumberInvertProtocol",
            default: "pyo3::class::number::PyNumberInvertProtocol",
        },
        MethodProto::Unary {
            name: "__complex__",
            pyres: true,
            proto: "pyo3::class::number::PyNumberComplexProtocol",
            default: "pyo3::class::number::PyNumberComplexProtocol",
        },
        MethodProto::Unary {
            name: "__int__",
            pyres: true,
            proto: "pyo3::class::number::PyNumberIntProtocol",
            default: "pyo3::class::number::PyNumberIntProtocol",
        },
        MethodProto::Unary {
            name: "__float__",
            pyres: true,
            proto: "pyo3::class::number::PyNumberFloatProtocol",
            default: "pyo3::class::number::PyNumberFloatProtocol",
        },
        MethodProto::Unary {
            name: "__round__",
            pyres: true,
            proto: "pyo3::class::number::PyNumberRoundProtocol",
            default: "pyo3::class::number::PyNumberRoundProtocol",
        },
        MethodProto::Unary {
            name: "__index__",
            pyres: true,
            proto: "pyo3::class::number::PyNumberIndexProtocol",
            default: "pyo3::class::number::PyNumberIndexProtocol",
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
