use embedded_recruitment_task::{
    message::{client_message, server_message, AddRequest, EchoMessage}, // Importing message types for client-server communication
    server::Server, // Importing server functionalities
};
use log::{debug, error, info, warn}; // Logging macros for debug, error, info, and warning levels
use std::{
    env, // Provides access to environment variables
    net::TcpListener, // Used to create and manage a TCP listener
    sync::Arc, // For shared ownership of server instances between threads
    thread::{self, JoinHandle}, // For thread creation and management
};

mod client; // Declares a client module for client-related operations

/// Utility function to set up a server in a separate thread.
///
/// # Arguments
/// - `server`: An `Arc`-wrapped server instance to allow shared ownership across threads.
///
/// # Returns
/// - A `JoinHandle` for the spawned thread that runs the server.
fn setup_server_thread(server: Arc<Server>) -> JoinHandle<()> {
    thread::spawn(move || {
        if let Err(e) = server.run() {
            error!("Server encountered an error: {}", e); // Log server errors
        }
    })
}

/// Utility function to create a new server instance and bind it to a unique port.
///
/// # Returns
/// - A tuple containing the `Arc`-wrapped server instance and the port number it is bound to.
fn create_server() -> (Arc<Server>, u16) {
    let listener = TcpListener::bind("localhost:0").expect("Failed to bind to an available port"); // Bind to an available port
    let port = listener.local_addr().unwrap().port(); // Retrieve the assigned port
    drop(listener); // Release the port for use by the server
    let server = Arc::new(Server::new(&format!("localhost:{}", port)).expect("Failed to start server")); // Initialize the server
    (server, port)
}

/// Test to validate basic client connection and disconnection behavior.
#[test]
fn test_client_connection() {
    // Set up environment for logging
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();

    // Create a server instance and start it in a separate thread
    let (server, port) = create_server();
    let handle = setup_server_thread(server.clone());

    // Initialize the client
    let mut client = client::Client::new("localhost", port.into(), 1000);

    // Test client connection
    match client.connect() {
        Ok(_) => debug!("Client connected successfully"),
        Err(e) => panic!("Failed to connect to the server: {}", e),
    }

    // Test client disconnection
    match client.disconnect() {
        Ok(_) => debug!("Client disconnected successfully"),
        Err(e) => panic!("Failed to disconnect from the server: {}", e),
    }

    // Stop the server and ensure the thread exits cleanly
    server.stop();
    assert!(handle.join().is_ok(), "Server thread panicked or failed to join");
}

/// Test to validate server handling of `EchoMessage`.
#[test]
fn test_client_echo_message() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();

    let (server, port) = create_server();
    let handle = setup_server_thread(server.clone());

    let mut client = client::Client::new("localhost", port.into(), 1000);
    assert!(client.connect().is_ok(), "Failed to connect to the server");

    // Create an EchoMessage
    let mut echo_message = EchoMessage::default();
    echo_message.content = "Hello, World!".to_string();
    let message = client_message::Message::EchoMessage(echo_message.clone());

    // Send the message and check for response
    assert!(client.send(message).is_ok(), "Failed to send message");

    let response = client.receive();
    assert!(response.is_ok(), "Failed to receive response for EchoMessage");

    if let Some(server_message::Message::EchoMessage(echo)) = response.unwrap().message {
        assert_eq!(
            echo.content, echo_message.content,
            "Expected echoed content: '{}', but got: '{}'",
            echo_message.content, echo.content
        );
    } else {
        panic!("Expected EchoMessage, but received a different message");
    }

    assert!(client.disconnect().is_ok(), "Failed to disconnect from the server");
    server.stop();
    assert!(handle.join().is_ok(), "Server thread panicked or failed to join");
}

/// Test to validate server handling of multiple `EchoMessage` instances sequentially.
#[test]
fn test_multiple_echo_messages() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();

    let (server, port) = create_server();
    let handle = setup_server_thread(server.clone());

    let mut client = client::Client::new("localhost", port.into(), 1000);
    assert!(client.connect().is_ok(), "Failed to connect to the server");

    let messages = vec!["Hello, World!", "How are you?", "Goodbye!"];

    for message_content in messages {
        let mut echo_message = EchoMessage::default();
        echo_message.content = message_content.to_string();
        let message = client_message::Message::EchoMessage(echo_message.clone());

        assert!(client.send(message).is_ok(), "Failed to send message");

        let response = client.receive();
        assert!(response.is_ok(), "Failed to receive response for EchoMessage");

        if let Some(server_message::Message::EchoMessage(echo)) = response.unwrap().message {
            assert_eq!(
                echo.content, echo_message.content,
                "Expected echoed content: '{}', but got: '{}'",
                echo_message.content, echo.content
            );
        } else {
            panic!("Expected EchoMessage, but received a different message");
        }
    }

    assert!(client.disconnect().is_ok(), "Failed to disconnect from the server");
    server.stop();
    assert!(handle.join().is_ok(), "Server thread panicked or failed to join");
}

/// Test to validate server handling of multiple clients simultaneously.
#[test]
fn test_multiple_clients() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();

    let (server, port) = create_server();
    let handle = setup_server_thread(server.clone());

    let client_count = 3;
    let mut clients: Vec<client::Client> = (0..client_count)
        .map(|_| client::Client::new("localhost", port.into(), 1000))
        .collect();

    for client in clients.iter_mut() {
        assert!(client.connect().is_ok(), "Failed to connect to the server");
    }

    let messages = vec!["Hello, World!", "How are you?", "Goodbye!"];

    for message_content in &messages {
        let mut echo_message = EchoMessage::default();
        echo_message.content = message_content.to_string();
        let message = client_message::Message::EchoMessage(echo_message.clone());

        for client in clients.iter_mut() {
            assert!(client.send(message.clone()).is_ok(), "Failed to send message");

            let response = client.receive();
            assert!(response.is_ok(), "Failed to receive response for EchoMessage");

            if let Some(server_message::Message::EchoMessage(echo)) = response.unwrap().message {
                assert_eq!(
                    echo.content, echo_message.content,
                    "Expected echoed content: '{}', but got: '{}'",
                    echo_message.content, echo.content
                );
            } else {
                panic!("Expected EchoMessage, but received a different message");
            }
        }
    }

    for client in clients.iter_mut() {
        assert!(client.disconnect().is_ok(), "Failed to disconnect from the server");
    }

    server.stop();
    assert!(handle.join().is_ok(), "Server thread panicked or failed to join");
}

/// Test to validate server handling of `AddRequest` messages.
#[test]
fn test_client_add_request() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();

    let (server, port) = create_server();
    let handle = setup_server_thread(server.clone());

    let mut client = client::Client::new("localhost", port.into(), 1000);
    assert!(client.connect().is_ok(), "Failed to connect to the server");

    // Create an AddRequest
    let mut add_request = AddRequest::default();
    add_request.a = 10;
    add_request.b = 20;
    let message = client_message::Message::AddRequest(add_request.clone());

    // Send the AddRequest and verify the response
    assert!(client.send(message).is_ok(), "Failed to send message");

    let response = client.receive();
    assert!(response.is_ok(), "Failed to receive response for AddRequest");

    if let Some(server_message::Message::AddResponse(add_response)) = response.unwrap().message {
        assert_eq!(
            add_response.result,
            add_request.a + add_request.b,
            "AddResponse result does not match"
        );
    } else {
        panic!("Expected AddResponse, but received a different message");
    }

    assert!(client.disconnect().is_ok(), "Failed to disconnect from the server");
    server.stop();
    assert!(handle.join().is_ok(), "Server thread panicked or failed to join");
}
