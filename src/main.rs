use std::{io::Read, net::{TcpListener, TcpStream}};


fn process_command(command: &str) {
    println!("COMMAND: {}", command);
}

fn handle_client(stream: &mut TcpStream) -> std::io::Result<()> {
    let mut buf = [0u8; 1024];

    // persistent accumulator
    let mut bytes: Vec<u8> = vec![];

    'maintain_connection: loop {
        let bytes_read = stream.read(&mut buf)?;

        // client disconnected
        if bytes_read == 0 {
            println!("client disconnected");
            break 'maintain_connection;
        }

        for byte in buf[..bytes_read].iter() {
            bytes.push(*byte);
            println!("{:?}",bytes);

            // newline delimiter
            if *byte as char == '\n' {

                // remove newline
                bytes.pop();

                // bytes -> String
                let command = String::from_utf8_lossy(&bytes);

                process_command(&command);

                // ready for next command
                bytes.clear();
            }
        }
    }

    Ok(())
}

fn tcp_listener() -> std::io::Result<()>{

    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        println!("new connection");
        let mut stream = stream?;
        handle_client(&mut stream)?;
    }

    Ok(())
}

fn main(){
match tcp_listener() {
    Ok(_) => {},
    Err(err) => {
        eprintln!("This is the error {}",err);
    }
}
    
}
