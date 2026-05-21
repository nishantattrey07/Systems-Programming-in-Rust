use std::net::{TcpListener,TcpStream};
use crate::database::Database; 
use std::io::{Read,Write};
use crate::parser::{process_command,execute_command};


fn handle_client(stream: &mut TcpStream, db:&mut Database) -> std::io::Result<()> {
    let mut buf = [0u8; 1024];

    
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
            // println!("{:?}",bytes);

            // newline delimiter
            if *byte as char == '\n' {

                // remove newline
                bytes.pop();

                // bytes -> String
                let command = String::from_utf8_lossy(&bytes);

              let response = process_command(&command);

              match response {
                  Ok(command) => {
                      // println!("command: {:#?}",command);
                      match execute_command(command, db){
                          Ok(data)=> {
                              stream.write_all(data.as_bytes()).expect("Failed to write to stream");
                              stream.flush().expect("Failed to flush stream");
                          },
                          Err(err) => {
                              eprintln!("{}",err);
                              
                          }
                      }
                  },
                  Err(err) => {
                      eprintln!("{}",err);
                  }
              }
                // ready for next command
                bytes.clear();
            }
        }
    }

    Ok(())
}

pub fn tcp_listener() -> std::io::Result<()>{

    let mut db =  Database::new();


    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        println!("new connection");
        let mut stream = stream?;
        handle_client(&mut stream,&mut db)?;
    }

    Ok(())
}