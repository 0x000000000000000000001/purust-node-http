// `Node.HTTP.Server` FFIs.
use std::rc::Rc;

use Purs_Node_HTTP_Types::{close_all_connections, HttpServer};

fn unbox(value: &crate::UnknownType) -> Rc<HttpServer> {
    value.unwrap_class::<Rc<HttpServer>>().clone()
}

pub fn Node_HTTP_Server_bytesParsed(_value: crate::UnknownType) -> i64 {
    0
}

pub fn Node_HTTP_Server_rawPacket(_value: crate::UnknownType) -> crate::UnknownType {
    crate::mk_array(Vec::new())
}

pub fn Node_HTTP_Server_closeAllConnectionsImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(Rc::new(|value| {
        let server = unbox(&value);
        close_all_connections(&server);
        crate::Value::Unit
    })))
}

pub fn Node_HTTP_Server_closeIdleConnectionsImpl() -> crate::UnknownType {
    Node_HTTP_Server_closeAllConnectionsImpl()
}

fn server_int(default: i64) -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(Rc::new(move |_| crate::mk_int(default))))
}

fn server_set_int() -> crate::UnknownType {
    crate::Value::Func2(purust_core::Func2::Static(|_, _| crate::Value::Unit))
}

fn server_milliseconds(default: i64) -> crate::UnknownType {
    // `Milliseconds` is a newtype over Number, erased by the backend.
    crate::Value::Func1(purust_core::Func1::Shared(Rc::new(move |_| {
        crate::mk_number(default as f64)
    })))
}

pub fn Node_HTTP_Server_headersTimeoutImpl() -> crate::UnknownType {
    server_int(60000)
}

pub fn Node_HTTP_Server_setHeadersTimeoutImpl() -> crate::UnknownType {
    server_set_int()
}

pub fn Node_HTTP_Server_maxHeadersCountImpl() -> crate::UnknownType {
    server_int(2000)
}

pub fn Node_HTTP_Server_setMaxHeadersCountImpl() -> crate::UnknownType {
    server_set_int()
}

pub fn Node_HTTP_Server_requestTimeoutImpl() -> crate::UnknownType {
    server_milliseconds(300000)
}

pub fn Node_HTTP_Server_setRequestTimeoutImpl() -> crate::UnknownType {
    server_set_int()
}

pub fn Node_HTTP_Server_maxRequestsPerSocketImpl() -> crate::UnknownType {
    server_int(0)
}

pub fn Node_HTTP_Server_setMaxRequestsPerSocketImpl() -> crate::UnknownType {
    server_set_int()
}

pub fn Node_HTTP_Server_timeoutImpl() -> crate::UnknownType {
    server_milliseconds(0)
}

pub fn Node_HTTP_Server_setTimeoutImpl() -> crate::UnknownType {
    server_set_int()
}

pub fn Node_HTTP_Server_keepAliveTimeoutImpl() -> crate::UnknownType {
    server_milliseconds(5000)
}

pub fn Node_HTTP_Server_setKeepAliveTimeoutImpl() -> crate::UnknownType {
    server_set_int()
}
