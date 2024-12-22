use crate::message::{AddRequest, EchoMessage}; // Import custom message structures for decoding and encoding client messages.
use log::{error, info, warn}; // Import macros for structured logging.
use prost::Message; // Import Protobuf support for encoding and decoding messages.
use std::{
    io::{self, ErrorKind, Read, Write}, // Import IO traits for stream handling.
    net::{TcpListener, TcpStream}, // Import network primitives for TCP communication.
    sync::{
        atomic::{AtomicBool, Ordering}, // Atomic types for thread-safe shared state.
        Arc, // Atomic Reference Counter for shared ownership.
    },
    thread, // Support for spawning threads.
    time::Duration, // Support for specifying time intervals.
};

// Represents a single connected client.
struct Client {
    stream: TcpStream, // TCP stream for communication with the client.
}

impl Client {
    // Creates a new Client instance, setting a read timeout for the TCP stream.
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        stream.set_read_timeout(Some(Duration::from_secs(10)))?; // Set a 10-second timeout for read operations.
        Ok(Client { stream })
    }

    // Handles communication with the client.
    pub fn handle(&mut self) -> io::Result<()> {
        let mut buffer = [0; 512]; // Buffer to store incoming data.

        match self.stream.read(&mut buffer) { // Read data from the TCP stream.
            Ok(0) => { // Client has disconnected.
                info!("Client disconnected.");
                return Ok(());
            }
            Ok(bytes_read) => { // Successfully read data from the client.
                if let Ok(echo_message) = EchoMessage::decode(&buffer[..bytes_read]) { // Try decoding an EchoMessage.
                    info!("Received EchoMessage: {}", echo_message.content); // Log the message content.
                    let payload = echo_message.encode_to_vec(); // Encode the message to send back.
                    self.stream.write_all(&payload)?; // Send the message back to the client (echo).
                    self.stream.flush()?; // Ensure all data is sent.
                } else if let Ok(add_request) = AddRequest::decode(&buffer[..bytes_read]) { // Try decoding an AddRequest.
                    info!("Received AddRequest: a = {}, b = {}", add_request.a, add_request.b); // Log the numbers to add.
                    let sum = add_request.a + add_request.b; // Compute the sum.

                    let response = AddRequest { a: sum, b: 0 }; // Reuse the AddRequest structure for the response.
                    let payload = response.encode_to_vec(); // Encode the sum as a response.
                    self.stream.write_all(&payload)?; // Send the response back to the client.
                    self.stream.flush()?; // Ensure all data is sent.
                } else {
                    warn!("Received invalid or unknown message format"); // Log an error if the message format is unrecognized.
                }
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => { // Handle non-blocking read timeout.
                thread::sleep(Duration::from_millis(100)); // Sleep briefly before retrying.
            }
            Err(e) => { // Handle other read errors.
                error!("Error reading from client stream: {}", e); // Log the error.
            }
        }

        Ok(())
    }
}

// Represents the server that listens for and manages client connections.
pub struct Server {
    listener: TcpListener, // TCP listener to accept incoming connections.
    is_running: Arc<AtomicBool>, // Shared state indicating if the server is running.
}

impl Server {
    // Creates a new Server instance bound to the specified address.
    pub fn new(addr: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?; // Bind the listener to the address.
        let is_running = Arc::new(AtomicBool::new(false)); // Initialize the server state.
        Ok(Server { listener, is_running })
    }

    // Runs the server, accepting and handling client connections.
    pub fn run(&self) -> io::Result<()> {
        self.is_running.store(true, Ordering::SeqCst); // Set the server state to running.
        info!("Server is running on {}", self.listener.local_addr()?); // Log the server address.

        self.listener.set_nonblocking(true)?; // Set the listener to non-blocking mode.

        while self.is_running.load(Ordering::SeqCst) { // Loop while the server is running.
            match self.listener.accept() { // Accept new client connections.
                Ok((stream, addr)) => {
                    info!("New client connected: {}", addr); // Log the client's address.

                    let is_running = Arc::clone(&self.is_running); // Clone the shared running state.
                    thread::spawn(move || { // Spawn a thread to handle the client.
                        match Client::new(stream) {
                            Ok(mut client) => {
                                while is_running.load(Ordering::SeqCst) { // Handle the client while the server is running.
                                    if let Err(e) = client.handle() { // Process client messages.
                                        error!("Error handling client: {}", e); // Log any errors.
                                        break;
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Failed to initialize client: {}", e); // Log errors during client initialization.
                            }
                        }
                    });
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => { // Handle non-blocking accept timeout.
                    thread::sleep(Duration::from_millis(100)); // Sleep briefly before retrying.
                }
                Err(e) => { // Handle other accept errors.
                    error!("Error accepting connection: {}", e); // Log the error.
                }
            }
        }

        info!("Server stopped."); // Log server shutdown.
        Ok(())
    }

    // Stops the server gracefully.
    pub fn stop(&self) {
        if self.is_running.load(Ordering::SeqCst) { // Check if the server is running.
            self.is_running.store(false, Ordering::SeqCst); // Set the server state to stopped.
            info!("Shutdown signal sent."); // Log the shutdown signal.
            if let Err(e) = self.listener.try_clone() { // Attempt to clone the listener.
                error!("Error while cloning listener during shutdown: {}", e); // Log cloning errors.
            }
        } else {
            warn!("Server was already stopped or not running."); // Warn if the server was already stopped.
        }
    }
}
