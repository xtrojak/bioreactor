use crate::config::DeviceMetadata;
use crate::device::{Bioreactor, DeviceStatus};
use std::collections::HashSet;
use std::net::TcpListener;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{sleep, JoinHandle};
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct NetworkReactorConfig {
    pub meta: DeviceMetadata,
    pub address: Ipv4Addr,
    pub port: u16,
}

/// A bioreactor device that listens to a low-level connection over a TCP socket.
pub struct NetworkReactor {
    metadata: DeviceMetadata,
    listener_thread: JoinHandle<()>,
    state: Arc<DeviceInfo>,
}

/// Enumeration of values that have been updated through the API
/// but the change hasn't been transmitted to the actual device.
#[derive(Clone, PartialEq, Eq, Hash)]
enum DeviceDataItem {
    TargetStatus,
}

struct DeviceInfo {
    abort_flag: AtomicBool,
    status: Mutex<DeviceStatus>,
    target_status: Mutex<DeviceStatus>,
    status_extra: Mutex<Option<String>>,
    to_update: Mutex<HashSet<DeviceDataItem>>,
}

impl DeviceInfo {
    pub fn new() -> DeviceInfo {
        DeviceInfo {
            abort_flag: AtomicBool::new(false),
            status: Mutex::new(DeviceStatus::Offline),
            target_status: Mutex::new(DeviceStatus::Inactive),
            status_extra: Mutex::new(None),
            to_update: Mutex::new(HashSet::new()),
        }
    }

    pub fn set_device_status(&self, status: DeviceStatus) {
        let mut guard = self.status.lock().unwrap();
        *guard = status;
    }

    pub fn get_device_status(&self) -> DeviceStatus {
        let guard = self.status.lock().unwrap();
        *guard
    }

    pub fn get_device_target_status(&self) -> DeviceStatus {
        let guard = self.target_status.lock().unwrap();
        *guard
    }

    pub fn set_device_target_status(&self, status: DeviceStatus) {
        let mut guard = self.target_status.lock().unwrap();
        let old_status = *guard;
        *guard = status;
        if old_status != status {
            let mut guard = self.to_update.lock().unwrap();
            guard.insert(DeviceDataItem::TargetStatus);
        }
    }

    pub fn set_device_status_extra(&self, message: &str) {
        let mut guard = self.status_extra.lock().unwrap();
        *guard = Some(message.to_string());
    }

    pub fn clear_device_status_extra(&self) {
        let mut guard = self.status_extra.lock().unwrap();
        *guard = None;
    }

    pub fn get_device_status_extra(&self) -> Option<String> {
        let guard = self.status_extra.lock().unwrap();
        (*guard).clone()
    }

    pub fn is_aborted(&self) -> bool {
        self.abort_flag.load(Ordering::SeqCst)
    }

    pub fn set_aborted(&self) {
        self.abort_flag.store(true, Ordering::SeqCst);
    }
}

impl NetworkReactor {
    /// Create a new reactor based on the provided config. If the network
    /// configuration cannot be realised, then a valid reactor object is still
    /// returned, but it will remain forever offline.
    pub fn from_config(config: NetworkReactorConfig) -> NetworkReactor {
        let device_info = Arc::new(DeviceInfo::new());
        device_info.set_device_status_extra("Device not initialized.");

        // Initialise the network connection.
        let socket = SocketAddrV4::new(config.address, config.port);
        let thread = match TcpListener::bind(socket) {
            Err(e) => {
                device_info.set_device_status(DeviceStatus::Offline);
                device_info.set_device_status_extra(
                    format!(
                        "Cannot open socket on {}:{} ({:?}).",
                        config.address, config.port, e
                    )
                    .as_str(),
                );
                thread::spawn(|| {
                    return ();
                })
            }
            Ok(listener) => start_listener(device_info.clone(), listener),
        };

        NetworkReactor {
            metadata: config.meta,
            listener_thread: thread,
            state: device_info.clone(),
        }
    }

    /// Block until the background thread of this reactor is stopped.
    pub fn abort(self) {
        self.state.set_aborted();
        if let Err(e) = self.listener_thread.join() {
            eprintln!("Error aborting bioreactor: {:?}", e);
        }
    }
}

fn start_listener(device_info: Arc<DeviceInfo>, listener: TcpListener) -> JoinHandle<()> {
    thread::spawn(move || {
        // The outer loop ensures we are either always listening, or having an active
        // connection.
        'connection: loop {
            device_info.set_device_status(DeviceStatus::Offline);
            device_info.set_device_status_extra("Waiting for a connection.");

            let (mut _stream, _address) = match listener.accept() {
                Ok(connection) => connection,
                Err(e) => {
                    let message = format!("Socket error `{:?}`. Waiting for a new connection.", e);
                    device_info.set_device_status_extra(message.as_str());
                    sleep(Duration::from_secs(1));
                    continue 'connection;
                }
            };

            // Now we have a valid data stream, but we still need to establish
            // a proper device protocol connection.
            device_info.set_device_status_extra("Connecting...");

            // For now, we don't do that. We just wait for the abort flag.
            loop {
                sleep(Duration::from_secs(1));

                // At some point, this should detect a closed connection and break
                // the loop.

                if device_info.is_aborted() {
                    return ();
                }
            }
        }
    })
}

impl Bioreactor for NetworkReactor {
    fn get_id(&self) -> String {
        self.metadata.id.clone()
    }

    fn get_name(&self) -> String {
        self.metadata.name.clone()
    }

    fn get_description(&self) -> Option<String> {
        self.metadata.description.clone()
    }

    fn get_current_status(&self) -> DeviceStatus {
        self.state.get_device_status()
    }

    fn get_current_status_extra(&self) -> Option<String> {
        self.state.get_device_status_extra()
    }

    fn get_target_status(&self) -> DeviceStatus {
        self.state.get_device_target_status()
    }

    fn set_target_status(&mut self, status: DeviceStatus) -> Result<(), String> {
        if status == DeviceStatus::Offline {
            Err("Setting device status to `Offline` is not permitted.".to_string())
        } else if self.state.get_device_status() == DeviceStatus::Offline {
            Err("Cannot set status of a device which is `Offline`.".to_string())
        } else {
            self.state.set_device_target_status(status);
            Ok(())
        }
    }
}
