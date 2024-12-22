use embedded_recruitment_task::message::{client_message, ServerMessage};
use log::{error, info, warn};
use prost::Message;
use std::{
    io::{self, Read, Write},
    net::{SocketAddr, TcpStream, ToSocketAddrs},
    time::Duration,
};

pub struct Client {
    ip: String,
    port: u32,
    timeout: Duration,
    stream: Option<TcpStream>,
}

impl Client {
    /// Creates a new client instance with the given IP, port, and timeout in milliseconds.
    pub fn new(ip: &str, port: u32, timeout_ms: u64) -> Self {
        Client {
            ip: ip.to_string(),
            port,
            timeout: Duration::from_millis(timeout_ms),
            stream: None,
        }
    }

    /// Connects the client to the server.
    pub fn connect(&mut self) -> io::Result<()> {
        info!("Connecting to {}:{}", self.ip, self.port);

        let address = format!("{}:{}", self.ip, self.port);
        let socket_addrs: Vec<SocketAddr> = address.to_socket_addrs()?.collect();

        if socket_addrs.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid IP or port",
            ));
        }

        let stream = TcpStream::connect_timeout(&socket_addrs[0], self.timeout)?;
        stream.set_read_timeout(Some(self.timeout))?;
        stream.set_write_timeout(Some(self.timeout))?;

        self.stream = Some(stream);
        info!("Connected to the server!");
        Ok(())
    }

    /// Disconnects the client from the server.
    pub fn disconnect(&mut self) -> io::Result<()> {
        if let Some(stream) = self.stream.take() {
            stream.shutdown(std::net::Shutdown::Both)?;
        }
        info!("Disconnected from the server!");
        Ok(())
    }

    /// Sends a message to the server.
    pub fn send(&mut self, message: client_message::Message) -> io::Result<()> {
        if let Some(ref mut stream) = self.stream {
            let mut buffer = Vec::new();
            // Assuming `encode` does not return a Result
            message.encode(&mut buffer);
            
            // If you need to handle errors related to the encoding, you can check it manually
            if buffer.is_empty() {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Encoding error"));
            }
    
            stream.write_all(&buffer)?;
            stream.flush()?;
            info!("Sent message: {:?}", message);
            Ok(())
        } else {
            warn!("Attempted to send message without an active connection");
            Err(io::Error::new(
                io::ErrorKind::NotConnected,
                "No active connection",
            ))
        }
    }

    /// Receives a message from the server.
    pub fn receive(&mut self) -> io::Result<ServerMessage> {
        if let Some(ref mut stream) = self.stream {
            info!("Receiving message from the server");
            let mut buffer = vec![0u8; 1024];
            let bytes_read = stream.read(&mut buffer)?;

            if bytes_read == 0 {
                warn!("Server disconnected or no data received.");
                return Err(io::Error::new(
                    io::ErrorKind::ConnectionAborted,
                    "Server disconnected",
                ));
            }

            ServerMessage::decode(&buffer[..bytes_read]).map_err(|e| {
                error!("Failed to decode ServerMessage: {}", e);
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Failed to decode ServerMessage: {}", e),
                )
            })
        } else {
            warn!("Attempted to receive message without an active connection");
            Err(io::Error::new(
                io::ErrorKind::NotConnected,
                "No active connection",
            ))
        }
    }
}
