# Solution  

This project involves debugging and enhancing a single-threaded server by transitioning it to a multi-threaded architecture. Below is the documentation of the process, challenges, and improvements.  

## Overview  

Initially, the server was a single-threaded implementation, but it needed to handle multiple client requests concurrently. After addressing the bugs in the original implementation, the server was transitioned to a multi-threaded design.  

## Tasks  

### 1. Debug and Fix the Existing Server Code  

#### Bugs and Design Flaws Documented:  

1. **Improper Use of `is_running`:** The `is_running` field is wrapped in `Arc<AtomicBool>` even though there is no multi-threading.  
2. **Infinite Loop on Client Handling:** The `handle` method for a client is repeatedly called in a loop within the `run` method. If the `handle` method encounters non-critical errors or doesn't terminate properly, the server can enter an infinite loop serving the same client.  
3. **Lack of Timeout in `TcpStream`:** The `TcpStream` object does not have a read/write timeout set in the `Client::new` method, leading to potential indefinite blocking.  
4. **Improper Handling of `ErrorKind::WouldBlock`:** The server sets the `TcpListener` to non-blocking mode but doesn’t handle scenarios where no connections are pending.  
5. **Shutdown and Disconnection Issues:** The server lacks proper mechanisms for shutting down or disconnecting gracefully.  
6. **Inconsistent Error Logging:** Some errors are logged while others are ignored, making debugging difficult.  
7. **Signal Handling for Graceful Stop:** The `stop` method relies on external calls to set `is_running` to `false` without any mechanism to handle signals for graceful shutdown.  

### 2. Transition to a Multi-Threaded Server  

The server was successfully transitioned to handle multiple clients concurrently using multi-threading. This modification allows the main thread to accept new connections while worker threads process client requests.  

**Status:**  
- The modified server handles multiple clients concurrently.  
- **Issue:** One test case, `test_client_add_request`, fails with the following output:  
    failures: test_client_add_request
    test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.02s
  The failure appears to be related to message handling, and work is ongoing to resolve this issue.  

### Enhancements to `client_test.rs`  

1. **Improved Client Activity Insights:** Provides clear insights into the client’s activities, such as connection status and message send/receive operations.  
2. **Validation of Buffer After Encoding:** Ensures that the buffer is not empty after encoding a message:  
 ```rust
 if buffer.is_empty() {
     return Err(io::Error::new(io::ErrorKind::InvalidData, "Encoding error"));
 }
```
3. Validation of IP and Port: Uses to_socket_addrs to validate the IP and port before attempting to connect.
4. Proper TCP Stream Shutdown: Ensures a proper shutdown during disconnection
```rust
stream.shutdown(std::net::Shutdown::Both)?;
```
5. **Active Connection Checks: Adds warnings for operations without an active connection:
```rust
warn!("Attempted to send message without an active connection");
warn!("Attempted to receive message without an active connection");
```
# Next Steps  

- Fix the `test_client_add_request` issue related to message handling.  
- Conduct further testing to ensure stability and performance in high-concurrency scenarios.  
- Document additional enhancements and their impact.  

# Contributions  

Feel free to contribute by reporting issues, suggesting improvements, or submitting pull requests!  

# License  

This project is licensed under the [MIT License](LICENSE).  
