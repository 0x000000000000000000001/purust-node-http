// `Node.HTTP.OutgoingMessage` FFIs.
use std::rc::Rc;

use Purs_Node_HTTP_Types::{
    outgoing_flush_head, outgoing_header, outgoing_header_names, outgoing_headers,
    outgoing_remove_header, outgoing_set_header, outgoing_socket, outgoing_state,
    OutgoingMessage,
};

fn unbox(value: &crate::UnknownType) -> Rc<OutgoingMessage> {
    value.unwrap_class::<Rc<OutgoingMessage>>().clone()
}

fn class_nullable(value: Option<crate::UnknownType>) -> crate::UnknownType {
    let nullable = match value {
        Some(value) => Purs_Data_Nullable::Data_Nullable_notNull(value),
        None => Purs_Data_Nullable::Data_Nullable_null(),
    };
    crate::Value::Class(Rc::new(nullable))
}

pub fn Node_HTTP_OutgoingMessage_addTrailersImpl() -> crate::UnknownType {
    crate::Value::Func2(purust_core::Func2::Static(|_, _| crate::Value::Unit))
}

pub fn Node_HTTP_OutgoingMessage_appendHeaderImpl() -> crate::UnknownType {
    crate::Value::Func3(purust_core::Func3::Shared(Rc::new(|name, value, message| {
        let message = unbox(&message);
        let name = name.unwrap_string();
        let value = value.unwrap_string();
        let existing = outgoing_header(&message, name.clone());
        let combined = match existing {
            Some(existing) => format!("{existing}, {value}"),
            None => value,
        };
        outgoing_set_header(&message, name, crate::Value::String(combined));
        crate::Value::Unit
    })))
}

pub fn Node_HTTP_OutgoingMessage_appendHeadersImpl() -> crate::UnknownType {
    crate::Value::Func3(purust_core::Func3::Shared(Rc::new(|name, values, message| {
        let message = unbox(&message);
        let name = name.unwrap_string();
        let joined = values
            .unwrap_array()
            .iter()
            .map(|value| value.unwrap_string())
            .collect::<Vec<_>>()
            .join(", ");
        let existing = outgoing_header(&message, name.clone());
        let combined = match existing {
            Some(existing) => format!("{existing}, {joined}"),
            None => joined,
        };
        outgoing_set_header(&message, name, crate::Value::String(combined));
        crate::Value::Unit
    })))
}

pub fn Node_HTTP_OutgoingMessage_flushHeadersImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(Rc::new(|value| {
        let message = unbox(&value);
        outgoing_flush_head(&message);
        crate::Value::Unit
    })))
}

pub fn Node_HTTP_OutgoingMessage_getHeaderImpl() -> crate::UnknownType {
    crate::Value::Func2(purust_core::Func2::Shared(Rc::new(|name, message| {
        let message = unbox(&message);
        let value = outgoing_header(&message, name.unwrap_string());
        class_nullable(value.map(crate::Value::String))
    })))
}

pub fn Node_HTTP_OutgoingMessage_getHeaderNamesImpl() -> crate::UnknownType {
    crate::Value::Func2(purust_core::Func2::Shared(Rc::new(|_, message| {
        let message = unbox(&message);
        let names = outgoing_header_names(&message);
        crate::mk_array(names.into_iter().map(crate::Value::String).collect())
    })))
}

pub fn Node_HTTP_OutgoingMessage_getHeadersImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(Rc::new(|value| {
        let message = unbox(&value);
        crate::Value::Class(Rc::new(outgoing_headers(&message)))
    })))
}

pub fn Node_HTTP_OutgoingMessage_hasHeaderImpl() -> crate::UnknownType {
    crate::Value::Func2(purust_core::Func2::Shared(Rc::new(|name, message| {
        let message = unbox(&message);
        crate::mk_bool(outgoing_header(&message, name.unwrap_string()).is_some())
    })))
}

pub fn Node_HTTP_OutgoingMessage_headersSentImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(Rc::new(|value| {
        let message = unbox(&value);
        let sent = outgoing_state(&message).lock().unwrap().headers_sent;
        crate::mk_bool(sent)
    })))
}

pub fn Node_HTTP_OutgoingMessage_removeHeaderImpl() -> crate::UnknownType {
    crate::Value::Func2(purust_core::Func2::Shared(Rc::new(|name, message| {
        let message = unbox(&message);
        outgoing_remove_header(&message, name.unwrap_string());
        crate::Value::Unit
    })))
}

pub fn Node_HTTP_OutgoingMessage_setHeaderImpl() -> crate::UnknownType {
    crate::Value::Func3(purust_core::Func3::Shared(Rc::new(|name, value, message| {
        let message = unbox(&message);
        outgoing_set_header(&message, name.unwrap_string(), value);
        crate::Value::Unit
    })))
}

pub fn Node_HTTP_OutgoingMessage_setHeaderArrImpl() -> crate::UnknownType {
    Node_HTTP_OutgoingMessage_setHeaderImpl()
}

pub fn Node_HTTP_OutgoingMessage_setTimeoutImpl() -> crate::UnknownType {
    crate::Value::Func2(purust_core::Func2::Static(|_, _| crate::Value::Unit))
}

pub fn Node_HTTP_OutgoingMessage_socketImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(Rc::new(|value| {
        let message = unbox(&value);
        class_nullable(outgoing_socket(&message))
    })))
}
