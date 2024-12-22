# Solution

after adding bin as entry point i started to fix the single threaded server and turn it into multi threaded server to handle multible the client’s request, allowing the main thread to accept new connections simultaneously 

First task -> Debug and fix the existing server code.

the server bugs Documentation:
1-The is_running field is wrapped in Arc<AtomicBool> even though there is no multi-threading
2-The handle method for a client is repeatedly called in a loop within the run method so If the handle method encounters non-critical errors or doesn't properly terminate, the server can enter an infinite loop serving the same client
3-The TcpStream object does not have a read/write timeout set in the Client::new method so the client could block the server indefinitely
4-the server sets the TcpListener to non-blocking mode, but it doesn’t properly handle scenarios where no connections are pending (ErrorKind::WouldBlock)
5-problems in shutdown or disconnect the server 
6-some errors are logged and the other ignored so it can cause problems in debugging 
7-The stop method relies on external calls to set is_running to false, but there’s no mechanism to handle signals to stop the server gracefully

second task -> Transition the server from single-threaded to multithreaded

and its done while the midified server can handle multible clients at a time but there is one test case the server fails in .. test_client_add_request

the output of the cargo test is: 
failures:
    test_client_add_request

test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.02s

the problem lies in handeling the message and iam working on fixing it

some enhancements to the client_test.rs:
1-Provides clear insights into the client’s activities (e.g., connection status, message send/receive operations)
2-Validates the buffer after encoding a message:
    if buffer.is_empty() {
    return Err(io::Error::new(io::ErrorKind::InvalidData, "Encoding error"));
    }
3-Using to_socket_addrs to validate IP and port before attempting to connect.
4-Ensures proper shutdown of the TCP stream during disconnection
    stream.shutdown(std::net::Shutdown::Both)?;
5-Adds checks for the active connection before sending or receiving messages:
    warn!("Attempted to send message without an active connection");
    warn!("Attempted to receive message without an active connection");

Here you can document all bugs and design flaws.