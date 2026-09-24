// `Node.HTTPS` FFIs. The secure server reuses the HTTP server on top of the net
// server with a TLS acceptor; secure requests delegate to the HTTP client,
// whose connection plan enables TLS for `https:` URLs and options.
use std::rc::Rc;

use Purs_Node_HTTP_Types::{http_server_new, HttpServer};

/// `key`/`cert` accept the upstream shapes: a `Buffer`, or an array whose
/// first `Buffer` is used (like Node's `tls.createSecureContext`).
fn buffer_bytes(value: &crate::UnknownType) -> Option<Vec<u8>> {
    if let crate::Value::Class(payload) = value.resolve() {
        if let Some(buffer) = payload
            .downcast_ref::<Rc<Purs_Node_Buffer_Immutable::ImmutableBuffer>>()
        {
            return Some(buffer.bytes());
        }
    }
    None
}

fn option_bytes(options: &crate::UnknownType, key: &str) -> Option<Vec<u8>> {
    let value = Purs_Node_Net_Types::option_field(options, key)?;
    match value.resolve() {
        crate::Value::Array(items) => items.iter().find_map(buffer_bytes),
        _ => buffer_bytes(&value),
    }
}

fn tls_acceptor(options: &crate::UnknownType) -> Option<Rc<native_tls::TlsAcceptor>> {
    let key = option_bytes(options, "key")?;
    let cert = option_bytes(options, "cert")?;
    let identity = native_tls::Identity::from_pkcs8(&cert, &key).ok()?;
    let acceptor = native_tls::TlsAcceptor::new(identity).ok()?;
    Some(Rc::new(acceptor))
}

pub fn Node_HTTPS_createSecureServer() -> crate::UnknownType {
    // Node allows creating a server without options and supplying them per
    // connection; the native port requires the certificate up front.
    crate::Value::Func1(purust_core::Func1::Static(|_| {
        Purs_Node_HTTP_Types::unsupported_server()
    }))
}

pub fn Node_HTTPS_createSecureServerOptsImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Static(|options| {
        let acceptor = tls_acceptor(&options)
            .expect("https.createServer: a PKCS#8 `key` and `cert` are required");
        let server: Rc<HttpServer> = http_server_new();
        Purs_Node_Net_Types::server_set_tls_acceptor(&server, acceptor);
        crate::Value::Class(Rc::new(server))
    }))
}

pub fn Node_HTTPS_requestStrImpl() -> crate::UnknownType {
    Purs_Node_HTTP::Node_HTTP_requestStrImpl()
}

pub fn Node_HTTPS_requestUrlImpl() -> crate::UnknownType {
    Purs_Node_HTTP::Node_HTTP_requestUrlImpl()
}

pub fn Node_HTTPS_requestStrOptsImpl() -> crate::UnknownType {
    Purs_Node_HTTP::Node_HTTP_requestStrOptsImpl()
}

pub fn Node_HTTPS_requestUrlOptsImpl() -> crate::UnknownType {
    Purs_Node_HTTP::Node_HTTP_requestUrlOptsImpl()
}

pub fn Node_HTTPS_requestOptsImpl() -> crate::UnknownType {
    Purs_Node_HTTP::Node_HTTP_requestOptsImpl()
}

pub fn Node_HTTPS_getStrImpl() -> crate::UnknownType {
    Purs_Node_HTTP::Node_HTTP_getStrImpl()
}

pub fn Node_HTTPS_getUrlImpl() -> crate::UnknownType {
    Purs_Node_HTTP::Node_HTTP_getUrlImpl()
}

pub fn Node_HTTPS_getStrOptsImpl() -> crate::UnknownType {
    Purs_Node_HTTP::Node_HTTP_getStrOptsImpl()
}

pub fn Node_HTTPS_getUrlOptsImpl() -> crate::UnknownType {
    Purs_Node_HTTP::Node_HTTP_getUrlOptsImpl()
}

pub fn Node_HTTPS_getOptsImpl() -> crate::UnknownType {
    Purs_Node_HTTP::Node_HTTP_getOptsImpl()
}
