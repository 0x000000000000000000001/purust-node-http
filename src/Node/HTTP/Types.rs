// Native HTTP support. Every HTTP message is a node-streams value: incoming
// messages are readable streams, outgoing messages are writable streams, and
// the HTTP metadata lives in the stream extension. Servers reuse the native net
// server, clients reuse the native net socket.
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use Purs_Node_EventEmitter::{purust_emitter_emit, EventEmitter};

pub type IncomingMessage = EventEmitter;
pub type OutgoingMessage = EventEmitter;
pub type ServerResponse = EventEmitter;
pub type ClientRequest = EventEmitter;
pub type HttpServer = EventEmitter;
/// The codegen mangles `HttpServer'` into this name.
pub type HttpServer_prime = EventEmitter;

type Object = Purs_Foreign_Object::Object;
type Socket = Purs_Node_Net_Types::Socket;

pub fn deliver(queue: &Option<crate::UnknownType>, job: impl FnOnce() + Send + Sync + 'static) {
    Purs_Node_Net_Types::deliver(queue, job);
}

pub fn queue_value() -> Option<crate::UnknownType> {
    Purs_Node_Net_Types::queue_value()
}

fn error(message: String) -> crate::UnknownType {
    Purs_Effect_Exception::Effect_Exception_error(message)
}

fn string_value(text: &str) -> crate::UnknownType {
    crate::Value::String(purust_core::purust_string_from_utf8(text))
}

fn bytes_value(bytes: Vec<u8>) -> crate::UnknownType {
    crate::Value::Class(Rc::new(
        Purs_Node_Buffer_Immutable::purust_buffer_from_bytes(bytes),
    ))
}

fn bytes_of(value: &crate::UnknownType) -> Vec<u8> {
    match value.resolve() {
        crate::Value::String(text) => purust_core::purust_string_to_utf8_lossy(text).into_bytes(),
        _ => value
            .unwrap_class::<Rc<Purs_Node_Buffer_Immutable::ImmutableBuffer>>()
            .bytes(),
    }
}

fn object_from(entries: Vec<(String, crate::UnknownType)>) -> Rc<Object> {
    Rc::new(Object::from_entries(entries))
}

fn object_entries(object: &Rc<Object>) -> Vec<(String, String)> {
    object
        .entries()
        .into_iter()
        .map(|(key, value)| (key, value.unwrap_string()))
        .collect()
}

pub fn headers_object(headers: &[(String, String)]) -> Rc<Object> {
    let mut grouped: Vec<(String, Vec<String>)> = Vec::new();
    for (name, value) in headers {
        match grouped.iter_mut().find(|(key, _)| key == name) {
            Some((_, values)) => values.push(value.clone()),
            None => grouped.push((name.clone(), vec![value.clone()])),
        }
    }
    object_from(
        grouped
            .into_iter()
            .map(|(name, values)| {
                let value = if values.len() == 1 {
                    string_value(&values[0])
                } else {
                    crate::mk_array(values.iter().map(|value| string_value(value)).collect())
                };
                (name, value)
            })
            .collect(),
    )
}

/// Finds the end of a header block (`\r\n\r\n`), returning the body offset.
pub fn header_end(bytes: &[u8]) -> Option<usize> {
    bytes
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .map(|index| index + 4)
}

pub fn parse_head(head: &str) -> (String, Vec<(String, String)>) {
    let mut lines = head.split("\r\n");
    let start = lines.next().unwrap_or("").to_owned();
    let mut headers = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if let Some((name, value)) = line.split_once(':') {
            headers.push((name.trim().to_ascii_lowercase(), value.trim().to_owned()));
        }
    }
    (start, headers)
}

pub fn header_value(headers: &[(String, String)], name: &str) -> Option<String> {
    headers
        .iter()
        .find(|(key, _)| key.eq_ignore_ascii_case(name))
        .map(|(_, value)| value.clone())
}

fn header_lines(headers: &[(String, String)]) -> String {
    headers
        .iter()
        .map(|(name, value)| format!("{name}: {value}\r\n"))
        .collect()
}

fn write_fd(fd: i32, bytes: &[u8]) {
    let mut written = 0usize;
    while written < bytes.len() {
        let count = unsafe {
            libc::write(
                fd,
                bytes[written..].as_ptr() as *const libc::c_void,
                bytes.len() - written,
            )
        };
        if count <= 0 {
            break;
        }
        written += count as usize;
    }
}

/// Incremental HTTP/1.1 head reader shared by both directions.
pub struct HeadReader {
    buffer: Vec<u8>,
}

impl HeadReader {
    pub fn new() -> HeadReader {
        HeadReader { buffer: Vec::new() }
    }

    pub fn push(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }

    pub fn take_head(&mut self) -> Option<(String, Vec<(String, String)>, Vec<u8>)> {
        let offset = header_end(&self.buffer)?;
        let head = String::from_utf8_lossy(&self.buffer[..offset]).into_owned();
        let rest = self.buffer[offset..].to_vec();
        self.buffer.clear();
        let (start, headers) = parse_head(&head);
        Some((start, headers, rest))
    }
}

/// Registers a native callback for one emission of an event.
pub fn register_native_listener(
    emitter: &Rc<EventEmitter>,
    event: &str,
    callback: impl Fn(crate::UnknownType) -> () + Send + Sync + 'static,
) {
    let callback = Arc::new(callback);
    let value = crate::Value::Func1(purust_core::Func1::Shared(Rc::new(move |argument| {
        callback(argument);
        crate::Value::Unit
    })));
    // HTTP consumes whole streams: a one-shot listener would drop every
    // chunk after the first (multi-segment requests, later connections).
    Purs_Node_EventEmitter::purust_emitter_on_native(emitter, event, value);
}

// ---------------------------------------------------------------------------
// Incoming messages
// ---------------------------------------------------------------------------

pub struct IncomingState {
    pub headers: Rc<Object>,
    pub raw_headers: Vec<String>,
    pub method: String,
    pub url: String,
    pub status_code: Option<i64>,
    pub status_message: String,
    pub http_version: String,
    pub complete: bool,
    pub socket: Option<crate::UnknownType>,
}

pub fn incoming_new(state: IncomingState) -> Rc<IncomingMessage> {
    let stream = Purs_Node_Stream::purust_readable_from_bytes(Vec::new());
    Purs_Node_Stream::purust_stream_set_extension(
        &stream,
        crate::Value::Class(Rc::new(Arc::new(Mutex::new(state)))),
    );
    stream
}

pub fn incoming_state(stream: &Rc<IncomingMessage>) -> Arc<Mutex<IncomingState>> {
    Purs_Node_Stream::purust_stream_extension(stream)
        .expect("Node.HTTP: incoming message without native state")
        .unwrap_class::<Arc<Mutex<IncomingState>>>()
        .clone()
}

fn incoming_raw_headers(headers: &[(String, String)]) -> Vec<String> {
    headers
        .iter()
        .flat_map(|(name, value)| [name.clone(), value.clone()])
        .collect()
}

// ---------------------------------------------------------------------------
// Outgoing messages
// ---------------------------------------------------------------------------

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum OutgoingKind {
    Response,
    Request,
}

pub struct OutgoingState {
    pub kind: OutgoingKind,
    pub headers: Vec<(String, String)>,
    pub headers_sent: bool,
    pub status_code: i64,
    pub status_message: String,
    pub method: String,
    pub path: String,
    pub protocol: String,
    pub host: String,
    pub fd: Option<i32>,
    pub socket: Option<crate::UnknownType>,
    pub pending: Vec<u8>,
    pub reader: HeadReader,
    pub response: Option<Rc<IncomingMessage>>,
    pub queue: Option<crate::UnknownType>,
    pub send_date: bool,
    pub request_sent: bool,
    pub request: Option<crate::UnknownType>,
}

pub fn outgoing_new(
    stream: &Rc<OutgoingMessage>,
    state: OutgoingState,
) {
    let handle = Arc::new(Mutex::new(state));
    Purs_Node_Stream::purust_stream_set_extension(
        stream,
        crate::Value::Class(Rc::new(handle.clone())),
    );
    let hooked = stream.clone();
    Purs_Node_Stream::purust_stream_set_write_hook(
        stream,
        Arc::new(move |bytes| {
            let (fd, ready) = {
                let state = outgoing_state(&hooked);
                let state = state.lock().unwrap();
                (state.fd, state.fd.is_some())
            };
            if !ready {
                let state = outgoing_state(&hooked);
                state.lock().unwrap().pending.extend_from_slice(bytes);
                return;
            }
            outgoing_flush_head(&hooked);
            if let Some(fd) = fd {
                write_fd(fd, bytes);
            }
        }),
    );
}

pub fn outgoing_state(stream: &Rc<OutgoingMessage>) -> Arc<Mutex<OutgoingState>> {
    Purs_Node_Stream::purust_stream_extension(stream)
        .expect("Node.HTTP: outgoing message without native state")
        .unwrap_class::<Arc<Mutex<OutgoingState>>>()
        .clone()
}

/// Sends the status/request line and headers, then the body buffered before
/// the descriptor was ready.
pub fn outgoing_flush_head(stream: &Rc<OutgoingMessage>) {
    let (head, fd) = {
        let state = outgoing_state(stream);
        let mut state = state.lock().unwrap();
        if state.headers_sent {
            return;
        }
        let Some(fd) = state.fd else {
            return;
        };
        state.headers_sent = true;
        let mut headers = state.headers.clone();
        if state.send_date && !headers.iter().any(|(name, _)| name == "date") {
            headers.push(("date".to_owned(), http_date()));
        }
        let head = match state.kind {
            OutgoingKind::Response => format!(
                "HTTP/1.1 {} {}\r\n{}",
                state.status_code,
                state.status_message,
                header_lines(&headers)
            ),
            OutgoingKind::Request => format!(
                "{} {} HTTP/1.1\r\n{}",
                state.method,
                state.path,
                header_lines(&headers)
            ),
        };
        let pending = std::mem::take(&mut state.pending);
        (format!("{head}\r\n{}", String::from_utf8_lossy(&pending)), fd)
    };
    write_fd(fd, head.as_bytes());
}

/// A minimal RFC 1123 date, used for the automatic `Date` header.
fn http_date() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|value| value.as_secs())
        .unwrap_or(0);
    let days = ["Thu", "Fri", "Sat", "Sun", "Mon", "Tue", "Wed"];
    let _ = days;
    // Day of week and calendar conversion are not worth pulling in a crate for:
    // the header is informational, so emit seconds since the epoch-based date.
    let mut remaining = now as i64;
    let mut day_seconds = remaining.rem_euclid(86_400);
    remaining = remaining.div_euclid(86_400);
    let hour = day_seconds / 3600;
    day_seconds %= 3600;
    let minute = day_seconds / 60;
    let second = day_seconds % 60;
    // 1970-01-01 was a Thursday.
    let weekday = ((remaining + 4).rem_euclid(7)) as usize;
    let date = civil_from_days(remaining);
    format!(
        "{}, {:02} {} {} {:02}:{:02}:{:02} GMT",
        days[weekday % 7],
        date.2,
        ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"]
            [(date.1 as usize).clamp(1, 12) - 1],
        date.0,
        hour,
        minute,
        second
    )
}

fn civil_from_days(days: i64) -> (i64, i64, i64) {
    // Howard Hinnant's civil_from_days.
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}

pub fn outgoing_set_header(stream: &Rc<OutgoingMessage>, name: String, value: crate::UnknownType) {
    let state = outgoing_state(stream);
    let mut state = state.lock().unwrap();
    let value = match value.resolve() {
        crate::Value::Array(values) => values
            .iter()
            .map(|value| value.unwrap_string())
            .collect::<Vec<_>>()
            .join(", "),
        _ => value.unwrap_string(),
    };
    let lower = name.to_ascii_lowercase();
    match state.headers.iter_mut().find(|(key, _)| *key == lower) {
        Some((_, existing)) => *existing = value,
        None => state.headers.push((lower, value)),
    }
}

pub fn outgoing_remove_header(stream: &Rc<OutgoingMessage>, name: String) {
    let state = outgoing_state(stream);
    let mut state = state.lock().unwrap();
    let lower = name.to_ascii_lowercase();
    state.headers.retain(|(key, _)| *key != lower);
}

pub fn outgoing_header(stream: &Rc<OutgoingMessage>, name: String) -> Option<String> {
    let state = outgoing_state(stream);
    let state = state.lock().unwrap();
    state
        .headers
        .iter()
        .find(|(key, _)| *key == name.to_ascii_lowercase())
        .map(|(_, value)| value.clone())
}

pub fn outgoing_headers(stream: &Rc<OutgoingMessage>) -> Rc<Object> {
    let state = outgoing_state(stream);
    let state = state.lock().unwrap();
    headers_object(&state.headers)
}

pub fn outgoing_header_names(stream: &Rc<OutgoingMessage>) -> Vec<String> {
    let state = outgoing_state(stream);
    let state = state.lock().unwrap();
    state.headers.iter().map(|(name, _)| name.clone()).collect()
}

pub fn outgoing_fd(stream: &Rc<OutgoingMessage>) -> Option<i32> {
    let state = outgoing_state(stream);
    let state = state.lock().unwrap();
    state.fd
}

pub fn outgoing_set_fd(stream: &Rc<OutgoingMessage>, fd: i32) {
    let state = outgoing_state(stream);
    state.lock().unwrap().fd = Some(fd);
}

pub fn outgoing_socket(stream: &Rc<OutgoingMessage>) -> Option<crate::UnknownType> {
    let state = outgoing_state(stream);
    let state = state.lock().unwrap();
    state.socket.clone()
}

// ---------------------------------------------------------------------------
// Client requests
// ---------------------------------------------------------------------------

pub struct RequestPlan {
    pub host: String,
    pub port: u16,
    pub path: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
    pub queue: Option<crate::UnknownType>,
}

fn parser_from_url(url: &str) -> Option<(String, u16, String)> {
    let rest = url
        .strip_prefix("http://")
        .or_else(|| url.strip_prefix("https://"))?;
    let (authority, path) = match rest.find('/') {
        Some(index) => (&rest[..index], &rest[index..]),
        None => (rest, "/"),
    };
    let (host, port) = match authority.rsplit_once(':') {
        Some((host, port)) => (host.to_owned(), port.parse().unwrap_or(80)),
        None => (authority.to_owned(), 80),
    };
    Some((host, port, path.to_owned()))
}

pub fn parse_request_options(options: &crate::UnknownType, url: Option<String>) -> RequestPlan {
    let mut host = Purs_Node_Net_Types::option_string(options, "hostname")
        .or_else(|| Purs_Node_Net_Types::option_string(options, "host"))
        .unwrap_or_else(|| "localhost".to_owned());
    let mut port = Purs_Node_Net_Types::option_int(options, "port").unwrap_or(80) as u16;
    let mut method = Purs_Node_Net_Types::option_string(options, "method")
        .unwrap_or_else(|| "GET".to_owned());
    let mut path = Purs_Node_Net_Types::option_string(options, "path").unwrap_or_else(|| "/".to_owned());
    if let Some(url) = url {
        if let Some((url_host, url_port, url_path)) = parser_from_url(&url) {
            if Purs_Node_Net_Types::option_string(options, "hostname").is_none()
                && Purs_Node_Net_Types::option_string(options, "host").is_none()
            {
                host = url_host;
            }
            if Purs_Node_Net_Types::option_int(options, "port").is_none() {
                port = url_port;
            }
            path = url_path;
        }
    }
    let mut headers: Vec<(String, String)> = Vec::new();
    if let Some(header_options) = Purs_Node_Net_Types::option_field(options, "headers") {
        for (name, value) in header_options.__purust_foreign_object().entries() {
            headers.push((name.to_ascii_lowercase(), value.unwrap_string()));
        }
    }
    if !headers.iter().any(|(name, _)| name == "host") {
        let host_header = if port == 80 {
            host.clone()
        } else {
            format!("{host}:{port}")
        };
        headers.push(("host".to_owned(), host_header));
    }
    if !headers.iter().any(|(name, _)| name == "connection") {
        headers.push(("connection".to_owned(), "close".to_owned()));
    }
    RequestPlan {
        host,
        port,
        path,
        method,
        headers,
        queue: queue_value(),
    }
}

/// Creates a client request and starts connecting the underlying socket.
pub fn client_request_new(plan: RequestPlan) -> Rc<ClientRequest> {
    let request = Purs_Node_Stream::purust_stream_new_stream(false, true);
    let socket = Purs_Node_Net_Types::socket_new_value(plan.queue.clone(), error, false);
    outgoing_new(
        &request,
        OutgoingState {
            kind: OutgoingKind::Request,
            headers: plan.headers.clone(),
            headers_sent: false,
            status_code: 0,
            status_message: String::new(),
            method: plan.method.clone(),
            path: plan.path.clone(),
            protocol: "http:".to_owned(),
            host: plan.host.clone(),
            fd: None,
            socket: Some(crate::Value::Class(Rc::new(socket.clone()))),
            pending: Vec::new(),
            reader: HeadReader::new(),
            response: None,
            queue: plan.queue.clone(),
            send_date: false,
            request_sent: false,
            request: None,
        },
    );

    let request_for_connect = request.clone();
    let socket_for_connect = socket.clone();
    register_native_listener(&socket, "connect", move |_| {
        let fd = {
            let state = Purs_Node_Net_Types::socket_state(&socket_for_connect);
            let state = state.lock().unwrap();
            state.fd
        };
        if let Some(fd) = fd {
            outgoing_set_fd(&request_for_connect, fd);
        }
        {
            let state = outgoing_state(&request_for_connect);
            state.lock().unwrap().request_sent = true;
        }
        outgoing_flush_head(&request_for_connect);
    });
    let request_for_data = request.clone();
    register_native_listener(&socket, "data", move |chunk| {
        client_ingest(&request_for_data, &bytes_of(&chunk));
    });
    // The response may have arrived before this listener was attached.
    let buffered = Purs_Node_Stream::purust_stream_take(&socket);
    if !buffered.is_empty() {
        client_ingest(&request, &buffered);
    }
    let request_for_end = request.clone();
    register_native_listener(&socket, "end", move |_| {
        if let Some(response) = client_response(&request_for_end) {
            Purs_Node_Stream::purust_stream_end(&response);
        }
    });
    let request_for_error = request.clone();
    let error_factory = |message: String| error(message);
    register_native_listener(&socket, "error", move |error_value| {
        let _ = error_factory;
        let request = request_for_error.clone();
        purust_emitter_emit(&request, "error", vec![error_value]);
    });

    Purs_Node_Net_Types::socket_connect(
        socket,
        Purs_Node_Net_Types::ConnectOptions {
            host: plan.host.clone(),
            port: plan.port,
            family: None,
            local_address: None,
            local_port: None,
            no_delay: false,
            keep_alive: false,
        },
    );
    request
}

fn client_response(request: &Rc<ClientRequest>) -> Option<Rc<IncomingMessage>> {
    outgoing_state(request).lock().unwrap().response.clone()
}

fn client_ingest(request: &Rc<ClientRequest>, bytes: &[u8]) {
    let (head, queue) = {
        let state = outgoing_state(request);
        let mut state = state.lock().unwrap();
        if state.response.is_some() {
            let response = state.response.clone();
            let queue = state.queue.clone();
            drop(state);
            if let Some(response) = response {
                Purs_Node_Stream::purust_stream_push(&response, bytes.to_vec());
            }
            return;
        }
        state.reader.push(bytes);
        let queue = state.queue.clone();
        (state.reader.take_head(), queue)
    };
    let Some((start, headers, rest)) = head else {
        return;
    };
    let mut parts = start.split_whitespace();
    let http_version = parts.next().unwrap_or("HTTP/1.1").to_owned();
    let status_code = parts
        .next()
        .and_then(|code| code.parse::<i64>().ok())
        .unwrap_or(0);
    let status_message = parts.collect::<Vec<_>>().join(" ");
    let socket = outgoing_socket(request);
    let response = incoming_new(IncomingState {
        headers: headers_object(&headers),
        raw_headers: incoming_raw_headers(&headers),
        method: String::new(),
        url: String::new(),
        status_code: Some(status_code),
        status_message,
        http_version: http_version.strip_prefix("HTTP/").unwrap_or("1.1").to_owned(),
        complete: false,
        socket,
    });
    {
        let state = outgoing_state(request);
        state.lock().unwrap().response = Some(response.clone());
    }
    if !rest.is_empty() {
        Purs_Node_Stream::purust_stream_push(&response, rest);
    }
    let request_for_emit = request.clone();
    let response_for_emit = response.clone();
    deliver(&queue, move || {
        purust_emitter_emit(
            &request_for_emit,
            "response",
            vec![crate::Value::Class(Rc::new(response_for_emit))],
        );
    });
}

// ---------------------------------------------------------------------------
// Servers
// ---------------------------------------------------------------------------

pub struct ConnectionState {
    pub reader: HeadReader,
    pub max_requests: i64,
    pub requests: i64,
}

pub struct HttpServerState {
    pub queue: Option<crate::UnknownType>,
    pub connections: Vec<crate::UnknownType>,
    pub close_connections: bool,
}

pub fn http_server_new() -> Rc<HttpServer> {
    let queue = queue_value();
    let server = Purs_Node_Net_Types::server_new_value(queue.clone(), error);
    Purs_Node_Net_Types::server_set_extension(
        &server,
        crate::Value::Class(Rc::new(Arc::new(Mutex::new(HttpServerState {
            queue: queue.clone(),
            connections: Vec::new(),
            close_connections: false,
        })))),
    );
    let server_for_connection = server.clone();
    register_native_listener(&server, "connection", move |socket_value| {
        let socket = socket_value
            .unwrap_class::<Rc<Socket>>()
            .clone();
        server_connection(server_for_connection.clone(), socket);
    });
    server
}

pub fn http_server_state(server: &Rc<HttpServer>) -> Arc<Mutex<HttpServerState>> {
    Purs_Node_Net_Types::server_get_extension(server)
        .expect("Node.HTTP: server without native state")
        .unwrap_class::<Arc<Mutex<HttpServerState>>>()
        .clone()
}

pub fn close_all_connections(server: &Rc<HttpServer>) {
    let connections = {
        let state = http_server_state(server);
        let mut state = state.lock().unwrap();
        state.close_connections = true;
        state.connections.clone()
    };
    for connection in connections {
        let socket = connection.unwrap_class::<Rc<Socket>>().clone();
        Purs_Node_Net_Types::socket_destroy(&socket, None);
    }
}

fn server_connection(server: Rc<HttpServer>, socket: Rc<Socket>) {
    {
        let state = http_server_state(&server);
        state
            .lock()
            .unwrap()
            .connections
            .push(crate::Value::Class(Rc::new(socket.clone())));
    }
    Purs_Node_Net_Types::socket_set_extension(
        &socket,
        crate::Value::Class(Rc::new(Arc::new(Mutex::new(ConnectionState {
            reader: HeadReader::new(),
            max_requests: 0,
            requests: 0,
        })))),
    );
    let server_for_data = server.clone();
    let socket_for_data = socket.clone();
    register_native_listener(&socket, "data", move |chunk| {
        server_ingest(server_for_data.clone(), socket_for_data.clone(), &bytes_of(&chunk));
    });
    // The client may have sent its request before this listener was attached.
    let buffered = Purs_Node_Stream::purust_stream_take(&socket);
    if !buffered.is_empty() {
        server_ingest(server, socket, &buffered);
    }
}

fn connection_state(socket: &Rc<Socket>) -> Arc<Mutex<ConnectionState>> {
    Purs_Node_Net_Types::socket_get_extension(socket)
        .expect("Node.HTTP: connection without native state")
        .unwrap_class::<Arc<Mutex<ConnectionState>>>()
        .clone()
}

fn server_ingest(server: Rc<HttpServer>, socket: Rc<Socket>, bytes: &[u8]) {
    let queue = http_server_state(&server).lock().unwrap().queue.clone();
    let head = {
        let state = connection_state(&socket);
        let mut state = state.lock().unwrap();
        state.reader.push(bytes);
        state.reader.take_head()
    };
    let Some((start, headers, rest)) = head else {
        return;
    };
    let mut parts = start.split_whitespace();
    let method = parts.next().unwrap_or("GET").to_owned();
    let url = parts.next().unwrap_or("/").to_owned();
    let version = parts.next().unwrap_or("HTTP/1.1").to_owned();
    let upgrade = header_value(&headers, "connection")
        .map(|value| value.to_ascii_lowercase().contains("upgrade"))
        .unwrap_or(false);

    let socket_value = crate::Value::Class(Rc::new(socket.clone()));
    let request = incoming_new(IncomingState {
        headers: headers_object(&headers),
        raw_headers: incoming_raw_headers(&headers),
        method: method.clone(),
        url: url.clone(),
        status_code: None,
        status_message: String::new(),
        http_version: version.strip_prefix("HTTP/").unwrap_or("1.1").to_owned(),
        complete: false,
        socket: Some(socket_value.clone()),
    });
    if !rest.is_empty() {
        Purs_Node_Stream::purust_stream_push(&request, rest);
    }

    if upgrade {
        let request_value = crate::Value::Class(Rc::new(request.clone()));
        deliver(&queue, move || {
            purust_emitter_emit(
                &server,
                "upgrade",
                vec![request_value, socket_value, bytes_value(Vec::new())],
            );
        });
        return;
    }

    let fd = {
        let state = Purs_Node_Net_Types::socket_state(&socket);
        let state = state.lock().unwrap();
        state.fd
    };
    let response = Purs_Node_Stream::purust_stream_new_stream(false, true);
    outgoing_new(
        &response,
        OutgoingState {
            kind: OutgoingKind::Response,
            headers: Vec::new(),
            headers_sent: false,
            status_code: 200,
            status_message: "OK".to_owned(),
            method: method.clone(),
            path: url.clone(),
            protocol: "http:".to_owned(),
            host: String::new(),
            fd,
            socket: Some(socket_value.clone()),
            pending: Vec::new(),
            reader: HeadReader::new(),
            response: None,
            queue: queue.clone(),
            send_date: true,
            request_sent: false,
            request: Some(crate::Value::Class(Rc::new(request.clone()))),
        },
    );
    if let Some(fd) = fd {
        Purs_Node_Stream::purust_stream_set_end_hook(
            &response,
            Arc::new(move || unsafe {
                libc::shutdown(fd, libc::SHUT_WR);
            }),
        );
    }
    let request_value = crate::Value::Class(Rc::new(request.clone()));
    let response_value = crate::Value::Class(Rc::new(response.clone()));
    deliver(&queue, move || {
        purust_emitter_emit(&server, "request", vec![request_value, response_value]);
    });
}

/// A client request that only reports that TLS is unavailable.
pub fn unsupported_client_request() -> crate::UnknownType {
    let request = Purs_Node_Stream::purust_stream_new_stream(false, true);
    outgoing_new(
        &request,
        OutgoingState {
            kind: OutgoingKind::Request,
            headers: Vec::new(),
            headers_sent: false,
            status_code: 0,
            status_message: String::new(),
            method: "GET".to_owned(),
            path: "/".to_owned(),
            protocol: "https:".to_owned(),
            host: String::new(),
            fd: None,
            socket: None,
            pending: Vec::new(),
            reader: HeadReader::new(),
            response: None,
            queue: queue_value(),
            send_date: false,
            request_sent: false,
            request: None,
        },
    );
    let request_for_error = request.clone();
    let queue = queue_value();
    deliver(&queue, move || {
        purust_emitter_emit(
            &request_for_error,
            "error",
            vec![error("TLS is not supported by the native backend".to_owned())],
        );
    });
    crate::Value::Class(Rc::new(request))
}

/// A server value used where TLS would be required.
pub fn unsupported_server() -> crate::UnknownType {
    let queue = queue_value();
    let server = Purs_Node_Net_Types::server_new_value(queue, error);
    crate::Value::Class(Rc::new(server))
}
