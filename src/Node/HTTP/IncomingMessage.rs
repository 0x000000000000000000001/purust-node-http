// `Node.HTTP.IncomingMessage` FFIs.
use std::rc::Rc;

use Purs_Node_HTTP_Types::{incoming_state, IncomingMessage};

fn unbox(value: &crate::UnknownType) -> Rc<IncomingMessage> {
    value.unwrap_class::<Rc<IncomingMessage>>().clone()
}

fn class_nullable(value: Option<crate::UnknownType>) -> crate::UnknownType {
    let nullable = match value {
        Some(value) => Purs_Data_Nullable::Data_Nullable_notNull(value),
        None => Purs_Data_Nullable::Data_Nullable_null(),
    };
    crate::Value::Class(Rc::new(nullable))
}

pub fn Node_HTTP_IncomingMessage_completeImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(Rc::new(|value| {
        let message = unbox(&value);
        let complete = incoming_state(&message).lock().unwrap().complete;
        crate::mk_bool(complete)
    })))
}

pub fn Node_HTTP_IncomingMessage_headersImpl(message: Rc<IncomingMessage>) -> crate::UnknownType {
    let headers = incoming_state(&message).lock().unwrap().headers.clone();
    crate::Value::Class(Rc::new(headers))
}

pub fn Node_HTTP_IncomingMessage_headersDistinct(message: Rc<IncomingMessage>) -> crate::UnknownType {
    let values = {
        let state = incoming_state(&message);
        let state = state.lock().unwrap();
        state
            .headers
            .entries()
            .into_iter()
            .map(|(name, value)| {
                let values = match value.resolve() {
                    crate::Value::Array(entries) => entries.to_vec(),
                    _ => vec![value.clone()],
                };
                (name, crate::mk_array(values))
            })
            .collect()
    };
    crate::Value::Class(Rc::new(purust_core::SharedRecord::from_entries(values)))
}

pub fn Node_HTTP_IncomingMessage_httpVersion(message: Rc<IncomingMessage>) -> String {
    incoming_state(&message).lock().unwrap().http_version.clone()
}

pub fn Node_HTTP_IncomingMessage_method(message: Rc<IncomingMessage>) -> String {
    incoming_state(&message).lock().unwrap().method.clone()
}

pub fn Node_HTTP_IncomingMessage_rawHeaders(message: Rc<IncomingMessage>) -> crate::UnknownType {
    let raw = incoming_state(&message).lock().unwrap().raw_headers.clone();
    crate::mk_array(raw.iter().map(|value| crate::Value::String(value.clone())).collect())
}

pub fn Node_HTTP_IncomingMessage_rawTrailersImpl(
    _message: Rc<IncomingMessage>,
) -> Rc<Purs_Data_Nullable::Nullable> {
    Purs_Data_Nullable::Data_Nullable_null()
}

pub fn Node_HTTP_IncomingMessage_socketImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(Rc::new(|value| {
        let message = unbox(&value);
        let socket = incoming_state(&message).lock().unwrap().socket.clone();
        class_nullable(socket)
    })))
}

pub fn Node_HTTP_IncomingMessage_statusCode(message: Rc<IncomingMessage>) -> i64 {
    incoming_state(&message)
        .lock()
        .unwrap()
        .status_code
        .unwrap_or(0)
}

pub fn Node_HTTP_IncomingMessage_statusMessage(message: Rc<IncomingMessage>) -> String {
    incoming_state(&message).lock().unwrap().status_message.clone()
}

pub fn Node_HTTP_IncomingMessage_trailersImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Static(|_| class_nullable(None)))
}

pub fn Node_HTTP_IncomingMessage_trailersDistinctImpl() -> crate::UnknownType {
    Node_HTTP_IncomingMessage_trailersImpl()
}

pub fn Node_HTTP_IncomingMessage_url(message: Rc<IncomingMessage>) -> String {
    incoming_state(&message).lock().unwrap().url.clone()
}

