use std::{
    io::{self, prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    fs
};

// define the socket ADDRess
const ADDR: &str = "localhost";
const PORT: u16 = 6783; // CS in ascii

fn main() {
    // HTTP request format:
    // METHOD (GET, POST, PUT, DELETE) Request-URI HTTP-Version CRLF(\r\n)
    // Headers CRLF
    // Message-body
    
    // HTTP response format:
    // HTTP-Version Status-Code Reason-Phrase CRLF
    // headers CRLF
    // message-body

    run();
}

fn run() {
    // Create a new connection
    let listener = TcpListener::bind(format!("{}:{}", ADDR, PORT)).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_conn(stream);
    }
}

fn handle_conn(mut stream: TcpStream) {
    // Implement buffering rather than reading the whole stream data
    // into one string
    let buf_reader = BufReader::new(&stream);
    
    // Read the request line by line until an empty line is read
    // Place the results in a vector
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    //let _ = 
    match request_line.as_str() {
        "GET / HTTP/1.1" => handle_home(stream),
        "GET /css/style.css HTTP/1.1" => handle_styles(stream),
        _ => handle_error(stream),
    };
}

fn handle_error(mut stream: TcpStream) -> io::Result<()> {
    println!("[ERROR] not found");
    stream.write_all("HTTP/1.1 404 NOT FOUND".as_bytes());
    Ok(())
}

fn handle_styles(mut stream: TcpStream) -> io::Result<()> {
    println!("[INFO] Loading styles...");
    // read in html file
    let path = "public/css/style.css";
    
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string(path).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
    Ok(())
}

fn handle_home(mut stream: TcpStream) -> io::Result<()> {
    println!("[INFO] Loading home page...");
    // read in html file
    let path = "public/index.html";
    
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string(path).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
    Ok(())
}

