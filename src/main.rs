use embedded_recruitment_task::server::Server;
use log::{info, debug, error, warn};

fn main() {
    // Initialize logging
    env_logger::init();

    // Create and run the server
    let addr = "127.0.0.1:8080"; // this is test address 
    match Server::new(addr) {
        Ok(server) => {
            println!("Server running on {}", addr);
            if let Err(e) = server.run() {
                eprintln!("Server encountered an error: {}", e);
            }
        }
        Err(e) => eprintln!("Failed to start server: {}", e),
    }
}
