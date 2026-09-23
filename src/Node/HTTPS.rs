// `Node.HTTPS` FFIs. TLS is not implemented by the native backend yet, so the
// secure-server factories and secure requests report an error instead of
// silently pretending to be encrypted.
use std::rc::Rc;

fn unsupported_server() -> crate::UnknownType {
    Purs_Node_HTTP_Types::unsupported_server()
}

pub fn Node_HTTPS_createSecureServer() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Static(|_| unsupported_server()))
}

pub fn Node_HTTPS_createSecureServerOptsImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Static(|_| unsupported_server()))
}

fn unsupported_request() -> crate::UnknownType {
    Purs_Node_HTTP_Types::unsupported_client_request()
}

pub fn Node_HTTPS_requestStrImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Static(|_| unsupported_request()))
}

pub fn Node_HTTPS_requestUrlImpl() -> crate::UnknownType {
    Node_HTTPS_requestStrImpl()
}

pub fn Node_HTTPS_requestStrOptsImpl() -> crate::UnknownType {
    crate::Value::Func2(purust_core::Func2::Static(|_, _| unsupported_request()))
}

pub fn Node_HTTPS_requestUrlOptsImpl() -> crate::UnknownType {
    Node_HTTPS_requestStrOptsImpl()
}

pub fn Node_HTTPS_requestOptsImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Static(|_| unsupported_request()))
}

pub fn Node_HTTPS_getStrImpl() -> crate::UnknownType {
    Node_HTTPS_requestStrImpl()
}

pub fn Node_HTTPS_getUrlImpl() -> crate::UnknownType {
    Node_HTTPS_requestStrImpl()
}

pub fn Node_HTTPS_getStrOptsImpl() -> crate::UnknownType {
    Node_HTTPS_requestStrOptsImpl()
}

pub fn Node_HTTPS_getUrlOptsImpl() -> crate::UnknownType {
    Node_HTTPS_requestStrOptsImpl()
}

pub fn Node_HTTPS_getOptsImpl() -> crate::UnknownType {
    Node_HTTPS_requestOptsImpl()
}
