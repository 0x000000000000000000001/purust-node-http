// `Node.HTTP` FFIs: servers and client requests.
use std::rc::Rc;

use Purs_Node_HTTP_Types::{
    client_request_new, http_server_new, parse_request_options, HttpServer,
};

fn url_text(value: &crate::UnknownType) -> Option<String> {
    match value.resolve() {
        crate::Value::String(text) => Some(text.clone()),
        _ => None,
    }
}

fn request_from(options: &crate::UnknownType, url: &crate::UnknownType) -> crate::UnknownType {
    let url = url_text(url);
    let plan = parse_request_options(options, url);
    let request = client_request_new(plan);
    crate::Value::Class(Rc::new(request))
}

pub fn Node_HTTP_createServer() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Static(|_| {
        crate::Value::Class(Rc::new(http_server_new()))
    }))
}

pub fn Node_HTTP_createServerOptsImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Static(|_| {
        crate::Value::Class(Rc::new(http_server_new()))
    }))
}

pub fn Node_HTTP_maxHeaderSize() -> i64 {
    16384
}

pub fn Node_HTTP_requestStrImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(Rc::new(|url| {
        request_from(&crate::Value::Unit, &url)
    })))
}

pub fn Node_HTTP_requestUrlImpl() -> crate::UnknownType {
    Node_HTTP_requestStrImpl()
}

pub fn Node_HTTP_requestStrOptsImpl() -> crate::UnknownType {
    crate::Value::Func2(purust_core::Func2::Shared(Rc::new(|url, options| {
        request_from(&options, &url)
    })))
}

pub fn Node_HTTP_requestUrlOptsImpl() -> crate::UnknownType {
    Node_HTTP_requestStrOptsImpl()
}

pub fn Node_HTTP_requestOptsImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(Rc::new(|options| {
        request_from(&options, &crate::Value::Unit)
    })))
}

pub fn Node_HTTP_getStrImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(Rc::new(|url| {
        request_from(&crate::Value::Unit, &url)
    })))
}

pub fn Node_HTTP_getUrlImpl() -> crate::UnknownType {
    Node_HTTP_getStrImpl()
}

pub fn Node_HTTP_getStrOptsImpl() -> crate::UnknownType {
    Node_HTTP_requestStrOptsImpl()
}

pub fn Node_HTTP_getUrlOptsImpl() -> crate::UnknownType {
    Node_HTTP_requestStrOptsImpl()
}

pub fn Node_HTTP_getOptsImpl() -> crate::UnknownType {
    Node_HTTP_requestOptsImpl()
}

pub fn Node_HTTP_setMaxIdleHttpParsersImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Static(|_| crate::Value::Unit))
}
