// `Node.HTTP.ClientRequest` FFIs.
use std::rc::Rc;

use Purs_Node_HTTP_Types::{outgoing_state, ClientRequest};

fn unbox(value: &crate::UnknownType) -> Rc<ClientRequest> {
    value.unwrap_class::<Rc<ClientRequest>>().clone()
}

pub fn Node_HTTP_ClientRequest_path(request: Rc<ClientRequest>) -> String {
    outgoing_state(&request).lock().unwrap().path.clone()
}

pub fn Node_HTTP_ClientRequest_method(request: Rc<ClientRequest>) -> String {
    outgoing_state(&request).lock().unwrap().method.clone()
}

pub fn Node_HTTP_ClientRequest_host(request: Rc<ClientRequest>) -> String {
    outgoing_state(&request).lock().unwrap().host.clone()
}

pub fn Node_HTTP_ClientRequest_protocol(request: Rc<ClientRequest>) -> String {
    outgoing_state(&request).lock().unwrap().protocol.clone()
}

pub fn Node_HTTP_ClientRequest_reusedSocket(_request: Rc<ClientRequest>) -> bool {
    false
}

pub fn Node_HTTP_ClientRequest_setNoDelayImpl() -> crate::UnknownType {
    crate::Value::Func2(purust_core::Func2::Static(|_, _| crate::Value::Unit))
}

pub fn Node_HTTP_ClientRequest_setSocketKeepAliveImpl() -> crate::UnknownType {
    crate::Value::Func3(purust_core::Func3::Static(|_, _, _| crate::Value::Unit))
}

pub fn Node_HTTP_ClientRequest_setTimeoutImpl() -> crate::UnknownType {
    crate::Value::Func2(purust_core::Func2::Static(|_, _| crate::Value::Unit))
}
