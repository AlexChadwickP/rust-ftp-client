# FTP Client in Rust

This project is a simple asynchronous FTP client written in Rust using the Tokio runtime. The client can connect to an FTP server, log in with a username and password, list the contents of the current directory, and gracefully quit the connection.

## Features
- Connect to FTP Server: Establishes a TCP connection to the specified FTP server.
- Login: Sends the username and password to the server for authentication.
- List Directory: Enters passive mode and retrieves the directory listing from the server.
- Quit: Sends the QUIT command to gracefully close the connection with the server.
## Requirements
- Rust: This program requires Rust to be installed on your machine. If you don't have Rust installed, you can get it from rust-lang.org.

- Tokio: The program is built using the Tokio asynchronous runtime. Ensure that your Cargo.toml includes the following dependencies:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```
Usage
1. Clone the repository:
    ```bash
    git clone https://github.com/yourusername/ftp-client-rust.git
    cd ftp-client-rust
    ```
2. Run the application:

    Build and run the application using Cargo:
    
    ```bash
    cargo run
    ```
3. Output:

    - The program will connect to the FTP server at the address 44.241.66.173:21.
    - It will attempt to log in using the username dlpuser and the password rNrKYTX9g7z3RgJRmxWuGHbeu.
   - The current directory's contents will be listed.
   - Finally, the connection will be terminated with the QUIT command.
   
## Customization
- Server Address: To connect to a different FTP server, change the server_addr variable in the main function.
- Credentials: Modify the username and password arguments in the login function call to use your FTP server's credentials.
- FTP Commands: You can extend the functionality by adding more FTP commands using the send_ftp_command function.

## Error Handling
This program includes basic error handling. If any operation fails (e.g., network errors, incorrect FTP command responses), the error will be returned and printed to the console.

## License
This project is licensed under the MIT License. See the LICENSE file for details.

## Contributing
Contributions are welcome! Please feel free to submit a pull request or open an issue if you find a bug or have a feature request.

## Acknowledgements
- Tokio - The asynchronous runtime for Rust that powers this FTP client.
