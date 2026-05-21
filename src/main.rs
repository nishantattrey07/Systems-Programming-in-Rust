use std::{char, io::Read, net::{TcpListener, TcpStream}};

// enum Command<'a> {
//     Set {
//         key: &'a str,
//         value: &'a str,
//     },

//     Get {
//         key: &'a str,
//     },

//     Delete {
//         key: &'a str,
//     },
// }

#[derive(Debug)]
enum Command {
    Set {
        key: String,
        value: String,
    },

    Get {
        key: String,
    },

    Delete {
        key: String,
    },
}

fn process_command(input: &str) -> Result<Command, String> {
    println!("COMMAND: {}", input);

    
    let input = input.trim_start();

    // split command from rest
    let mut split_data = input.splitn(2, ' ');

    let command = split_data
        .next()
        .ok_or("Command is empty".to_string())?;

    match command.to_uppercase().as_str() {
        "SET" => {
            let rest = split_data
                .next()
                .ok_or("SET requires key and value".to_string())?;

            let mut set_parts = rest.trim_start().splitn(2, ' ');

            let key = set_parts
                .next()
                .ok_or("Missing key".to_string())?;

            let value = set_parts
                .next()
                .ok_or("Missing value".to_string())?;

            Ok(Command::Set {
                key: key.to_string(),
                value: value.to_string(),
            })
        }

        "GET" => {
            let key = split_data
                .next()
                .ok_or("GET requires key".to_string())?;

            Ok(Command::Get {
                key: key.to_string(),
            })
        }

        "DELETE" => {
            let key = split_data
                .next()
                .ok_or("DELETE requires key".to_string())?;

            Ok(Command::Delete {
                key: key.to_string(),
            })
        }

        _ => Err("Unknown command".to_string()),
    }
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
                      println!("command: {:#?}",command);
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
