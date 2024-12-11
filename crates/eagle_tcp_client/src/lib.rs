#![allow(unused, non_snake_case)]

extern crate libc;

use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use libc::c_char;
use std::{
    ffi::CStr,
    fs::OpenOptions,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::RwLock;
use tokio::{net::TcpStream, runtime::Runtime};
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};
use tracing::{debug, error, info};
use tracing_subscriber::{fmt, layer::SubscriberExt, Registry};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[repr(i32)]
pub enum SocketTag {
    Login = 1,
    Gateway = 2,
}

pub struct ConnectionKeeper {
    Login: Option<Connection>,
    Gateway: Option<Connection>,
}

pub struct Connection {
    sender: Sender<Message>,
    receiver: Mutex<Receiver<Message>>, // Mutex protects access to the Receiver
    write: Arc<RwLock<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
}

static mut RUNTIME: Option<Arc<Runtime>> = None;
static mut CONNECTIONS: ConnectionKeeper = ConnectionKeeper {
    Login: None,
    Gateway: None,
};

impl SocketTag {
    pub fn connection(&self) -> Option<&Connection> {
        match self {
            SocketTag::Login => unsafe { CONNECTIONS.Login.as_ref() },
            SocketTag::Gateway => unsafe { CONNECTIONS.Gateway.as_ref() },
        }
    }

    pub fn set_connection(&self, conn: Option<Connection>) {
        match self {
            SocketTag::Login => unsafe {
                CONNECTIONS.Login = conn;
            },
            SocketTag::Gateway => unsafe {
                CONNECTIONS.Gateway = conn;
            },
        }
    }
}

fn get_runtime() -> &'static Arc<Runtime> {
    unsafe {
        if RUNTIME.is_none() {
            RUNTIME = Some(Arc::new(Runtime::new().unwrap()));
        }
        RUNTIME.as_ref().unwrap()
    }
}

#[no_mangle]
pub extern "system" fn DllMain(
    _hinst_dll: *mut std::ffi::c_void,
    _fdw_reason: u32,
    _lpreserved: *mut std::ffi::c_void,
) -> bool {
    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("EagleTcp.log")
        .unwrap();

    let subscriber = Registry::default()
        .with(fmt::layer().with_ansi(false).pretty())
        .with(fmt::layer().with_ansi(false).with_writer(log_file));

    if let Ok(_) = tracing::subscriber::set_global_default(subscriber) {
        info!("DLL initialized");
    }

    true
}

#[no_mangle]
pub extern "C" fn contectServer(
    psz_server_ip: *const c_char,
    n_server_port: i32,
    tag: SocketTag,
) -> bool {
    let c_str = unsafe { CStr::from_ptr(psz_server_ip) };
    let mut server_ip = c_str.to_str().unwrap();

    if tag == SocketTag::Login {
        server_ip = "127.0.0.1";
    }

    let server_url = format!("ws://{}:{}", server_ip, n_server_port);
    debug!("Connecting to server: {}", server_url);

    let runtime = get_runtime().clone();
    runtime.spawn(async move {
        match connect_async(&server_url).await {
            Ok((ws_stream, _)) => {
                info!("WebSocket connection established for tag: {:?}", tag);

                let (write, read) = ws_stream.split();

                let write = Arc::new(RwLock::new(write));

                // Setup channel for message passing
                let (tx, rx) = mpsc::channel(100);
                let receiver = Mutex::new(rx);

                // Spawn a task to read messages and queue them
                let tx_clone = tx.clone();
                tokio::spawn(async move {
                    read.for_each(|message| async {
                        if let Ok(msg) = message {
                            tx_clone.send(msg).await.unwrap_or_else(|_| {
                                error!("Failed to enqueue incoming message");
                            });
                        }
                    })
                    .await;
                });

                tag.set_connection(Some(Connection {
                    sender: tx,
                    receiver,
                    write,
                }));
            }
            Err(err) => {
                error!("Failed to connect to server: {}", err);
            }
        }
    });

    true
}

#[no_mangle]
pub extern "C" fn disconnectServer(tag: SocketTag) {
    tag.set_connection(None);
}

#[no_mangle]
pub extern "C" fn isConnected(tag: SocketTag) -> bool {
    tag.connection().is_some()
}

#[no_mangle]
pub extern "C" fn sendCmd(
    tag: SocketTag,
    main_cmd: u32,
    para_cmd: u32,
    msg_content: *const u8,
    size: i32,
) -> i32 {
    if msg_content.is_null() || size <= 0 {
        error!("Invalid message content or size");
        return -1;
    }

    let data: &[u8] = unsafe { std::slice::from_raw_parts(msg_content, size as usize) };
    debug!(
        "Sending command: {:?}, {}, {}, {:?}",
        tag, main_cmd, para_cmd, data
    );

    if let Some(conn) = tag.connection() {
        let msg = Message::Binary(data.to_vec());
        let write = conn.write.clone();
        let runtime = get_runtime().clone();

        runtime.spawn(async move {
            let mut write_guard = write.write().await;
            if let Err(err) = write_guard.send(msg).await {
                error!("Failed to send message: {}", err);
            }
        });

        0
    } else {
        error!("No connection found for tag: {:?}", tag);
        -1
    }
}

#[no_mangle]
pub extern "C" fn parseCmd(
    tag: SocketTag,
    main_cmd: *mut i32,
    para_cmd: *mut i32,
    json_out: *mut u8,
    size: i32,
) -> i32 {
    if let Some(conn) = tag.connection() {
        let mut receiver = conn.receiver.lock().unwrap();
        if let Ok(msg) = receiver.try_recv() {
            match msg {
                Message::Binary(data) => {
                    debug!("Received message: {:?}", data);
                    // Fill out main_cmd, para_cmd, and json_out as needed
                    // For now, return the size of the message
                    return data.len() as i32;
                }
                _ => {
                    error!("Unsupported message type");
                    return -1;
                }
            }
        } else {
            debug!("No message available in queue");
            return 0;
        }
    } else {
        error!("No connection found for tag: {:?}", tag);
        return -1;
    }
}
