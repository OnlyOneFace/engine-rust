mod logger;

use log::info;
use std::net::{TcpListener, TcpStream};
use std::{thread, time, str};
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    logger::init();

    let listener = TcpListener::bind("localhost:8080")?;

    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    for stream in listener.incoming() {
        let stream = stream.expect("failed");
        let handle = thread::spawn(move || {
            handle_client(stream).unwrap_or_else(|error| eprint!("{:?}", error));
        });
        thread_vec.push(handle);
    }
    for handle in thread_vec {
        handle.join().unwrap();
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0; 512];
    for _ in 0..1000 {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        info!("{}", str::from_utf8(&buf[..bytes_read]).unwrap());
        stream.write(&buf[..bytes_read])?;
        thread::sleep(time::Duration::from_secs(1));
    }
    Ok(())
}