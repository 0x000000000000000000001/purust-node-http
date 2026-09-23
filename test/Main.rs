// Test helpers: a timer and a writable stdout.
use std::rc::Rc;

pub fn Test_Main_setTimeoutImpl() -> crate::UnknownType {
    crate::Value::Func2(purust_core::Func2::Shared(Rc::new(|milliseconds, action| {
        let milliseconds = match milliseconds.resolve() {
            crate::Value::Int(value) => *value,
            crate::Value::Number(value) => *value as i64,
            _ => 0,
        };
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(milliseconds.max(0) as u64));
            Purs_Node_Net_Types::deliver(&Purs_Node_Net_Types::queue_value(), move || {
                action.unwrap_func1()(crate::Value::Unit);
            });
        });
        crate::Value::Unit
    })))
}

pub fn Test_Main_stdout() -> crate::UnknownType {
    let stream = Purs_Node_Stream::purust_writable_new();
    Purs_Node_Stream::purust_stream_set_write_fd(&stream, 1);
    crate::Value::Class(Rc::new(stream))
}
