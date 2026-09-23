// `Node.HTTP.ServerResponse` FFIs.
use std::rc::Rc;

use Purs_Node_HTTP_Types::{outgoing_flush_head, outgoing_set_header, outgoing_state, ServerResponse};

fn unbox(value: &crate::UnknownType) -> Rc<ServerResponse> {
    value.unwrap_class::<Rc<ServerResponse>>().clone()
}

pub fn Node_HTTP_ServerResponse_req(value: crate::UnknownType) -> crate::UnknownType {
    let response = unbox(&value);
    let request = outgoing_state(&response).lock().unwrap().request.clone();
    request.unwrap_or(crate::Value::Unit)
}

pub fn Node_HTTP_ServerResponse_sendDateImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(Rc::new(|value| {
        let response = unbox(&value);
        let send_date = outgoing_state(&response).lock().unwrap().send_date;
        crate::mk_bool(send_date)
    })))
}

pub fn Node_HTTP_ServerResponse_setSendDateImpl() -> crate::UnknownType {
    crate::Value::Func2(purust_core::Func2::Shared(Rc::new(|flag, response| {
        let response = unbox(&response);
        outgoing_state(&response).lock().unwrap().send_date = flag.unwrap_bool();
        crate::Value::Unit
    })))
}

pub fn Node_HTTP_ServerResponse_statusCodeImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(Rc::new(|value| {
        let response = unbox(&value);
        let status = outgoing_state(&response).lock().unwrap().status_code;
        crate::mk_int(status)
    })))
}

pub fn Node_HTTP_ServerResponse_setStatusCodeImpl() -> crate::UnknownType {
    crate::Value::Func2(purust_core::Func2::Shared(Rc::new(|status, response| {
        let response = unbox(&response);
        outgoing_state(&response).lock().unwrap().status_code = status.unwrap_int();
        crate::Value::Unit
    })))
}

pub fn Node_HTTP_ServerResponse_statusMessageImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(Rc::new(|value| {
        let response = unbox(&value);
        let message = outgoing_state(&response).lock().unwrap().status_message.clone();
        crate::Value::String(message)
    })))
}

pub fn Node_HTTP_ServerResponse_setStatusMessageImpl() -> crate::UnknownType {
    crate::Value::Func2(purust_core::Func2::Shared(Rc::new(|message, response| {
        let response = unbox(&response);
        outgoing_state(&response).lock().unwrap().status_message = message.unwrap_string();
        crate::Value::Unit
    })))
}

pub fn Node_HTTP_ServerResponse_strictContentLengthImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Static(|_| crate::mk_bool(false)))
}

pub fn Node_HTTP_ServerResponse_setStrictContentLengthImpl() -> crate::UnknownType {
    crate::Value::Func2(purust_core::Func2::Static(|_, _| crate::Value::Unit))
}

pub fn Node_HTTP_ServerResponse_writeEarlyHintsImpl() -> crate::UnknownType {
    crate::Value::Func2(purust_core::Func2::Static(|_, _| crate::Value::Unit))
}

pub fn Node_HTTP_ServerResponse_writeEarlyHintsCbImpl() -> crate::UnknownType {
    crate::Value::Func3(purust_core::Func3::Shared(Rc::new(|_, callback, _| {
        callback.unwrap_func1()(crate::Value::Unit);
        crate::Value::Unit
    })))
}

fn write_head(status: i64, message: Option<String>, headers: Option<crate::UnknownType>, response: Rc<ServerResponse>) {
    {
        let state = outgoing_state(&response);
        let mut state = state.lock().unwrap();
        state.status_code = status;
        if let Some(message) = message {
            state.status_message = message;
        }
        if let Some(headers) = headers {
            for (name, value) in headers.__purust_foreign_object().entries() {
                state.headers.push((name.to_ascii_lowercase(), value.unwrap_string()));
            }
        }
    }
    outgoing_flush_head(&response);
}

pub fn Node_HTTP_ServerResponse_writeHeadImpl() -> crate::UnknownType {
    crate::Value::Func2(purust_core::Func2::Shared(Rc::new(|status, response| {
        write_head(status.unwrap_int(), None, None, unbox(&response));
        crate::Value::Unit
    })))
}

pub fn Node_HTTP_ServerResponse_writeHeadMsgImpl() -> crate::UnknownType {
    crate::Value::Func3(purust_core::Func3::Shared(Rc::new(|status, message, response| {
        write_head(status.unwrap_int(), Some(message.unwrap_string()), None, unbox(&response));
        crate::Value::Unit
    })))
}

pub fn Node_HTTP_ServerResponse_writeHeadHeadersImpl() -> crate::UnknownType {
    crate::Value::Func3(purust_core::Func3::Shared(Rc::new(|status, headers, response| {
        write_head(status.unwrap_int(), None, Some(headers), unbox(&response));
        crate::Value::Unit
    })))
}

pub fn Node_HTTP_ServerResponse_writeHeadMsgHeadersImpl() -> crate::UnknownType {
    crate::Value::Func4(purust_core::Func4::Shared(Rc::new(
        |status, message, headers, response| {
            write_head(
                status.unwrap_int(),
                Some(message.unwrap_string()),
                Some(headers),
                unbox(&response),
            );
            crate::Value::Unit
        },
    )))
}

pub fn Node_HTTP_ServerResponse_writeProcessingImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Static(|_| crate::Value::Unit))
}
