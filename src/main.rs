use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::error::Error;

async fn connect_to_ftp_server(server_addr: &str) -> Result<TcpStream, Box<dyn Error>> {
    let mut stream = TcpStream::connect(server_addr).await?;
    println!("Connected to FTP server at {}", server_addr);

    let mut buffer = [0u8; 1024];
    let n = stream.read(&mut buffer).await?;

    println!("Server: {}", String::from_utf8_lossy(&buffer[..n]));

    Ok(stream)
}

async fn send_ftp_command(stream: &mut TcpStream, command: &str) -> Result<String, Box<dyn Error>> {
    stream.write_all(command.as_bytes()).await?;
    stream.write_all(b"\r\n").await?;

    let mut buffer = [0u8; 1024];
    let n = stream.read(&mut buffer).await?;

    Ok(String::from_utf8_lossy(&buffer[..n]).to_string())
}

async fn login(stream: &mut TcpStream, username: &str, password: &str) -> Result<(), Box<dyn Error>> {
    let response = send_ftp_command(stream, &format!("USER {}", username)).await?;
    println!("Server: {}", response);

    let response = send_ftp_command(stream, &format!("PASS {}", password)).await?;
    println!("Server: {}", response);

    Ok(())
}

async fn list_directory(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let (ip_addr, port) = enter_passive_mode(stream).await?;

    let mut data_stream = TcpStream::connect((ip_addr.as_str(), port)).await?;

    let response = send_ftp_command(stream, "LIST").await?;
    println!("Server: {}", response);

    let mut buffer = [0u8; 1024];
    let n = data_stream.read(&mut buffer).await?;
    println!("Directory listing:\n{}", String::from_utf8_lossy(&buffer[..n]));

    drop(data_stream);

    let response = send_ftp_command(stream, "").await?;
    println!("Server: {}", response);

    Ok(())
}


async fn quit(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let response = send_ftp_command(stream, "QUIT").await?;
    println!("Server: {}", response);

    Ok(())
}

async fn enter_passive_mode(stream: &mut TcpStream) -> Result<(String, u16), Box<dyn Error>> {
    let response = send_ftp_command(stream, "PASV").await?;
    println!("Server: {}", response);

    let start = response.find('(').ok_or("Failed to parse PASV response")? + 1;
    let end = response.find(')').ok_or("Failed to parse PASV response")?;

    let numbers: Vec<u8> = response[start..end]
        .split(',')
        .map(|s| s.parse::<u8>().unwrap_or(0))
        .collect();

    if numbers.len() != 6 {
        return Err("Unexpected PASV response format".into());
    }

    let ip_addr = format!("{}.{}.{}.{}", numbers[0], numbers[1], numbers[2], numbers[3]);
    let port = (numbers[4] as u16) << 8 | (numbers[5] as u16);

    Ok((ip_addr, port))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server_addr = "44.241.66.173:21";
    let mut stream = connect_to_ftp_server(server_addr).await?;

    login(&mut stream, "dlpuser", "rNrKYTX9g7z3RgJRmxWuGHbeu").await?;
    list_directory(&mut stream).await?;
    quit(&mut stream).await?;

    Ok(())
}
