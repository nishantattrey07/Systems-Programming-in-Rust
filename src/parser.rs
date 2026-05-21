use crate::database::Database;
use crate::types::ProtocolError;

#[derive(Debug)]
pub(crate) enum Command {
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





pub(crate) fn execute_command(command:Command,db:&mut Database)->String{
    match command {
        Command::Set{key,value} => {
            db.set(key,value);
            "OK".to_string()
               
            
        },
        Command::Get { key } => {
            match db.get(&key){
                Some(data) => {
                    let response = format!("VALUE {}",data);
                    response
                    
                },
                None => {
                    "NOT_FOUND".to_string()
                    
                }
            }
        },
        Command::Delete { key } =>{
            if db.delete(&key){
                "OK".to_string()
            } else{
                "NOT_FOUND".to_string()
            }
        }
    }
    
}



pub(crate) fn process_command(input: &str) -> Result<Command, ProtocolError> {
    // println!("COMMAND: {}", input);
    let input = input.trim_start();

    // split command from rest
    let mut split_data = input.splitn(2, ' ');

    let command = split_data
        .next()
        .ok_or(ProtocolError::EmptyCommand)?;

    match command.to_uppercase().as_str() {
        "SET" => {
            let rest = split_data
                .next()
                .ok_or(ProtocolError::MissingKey)?;

            

            let mut set_parts = rest.trim_start().splitn(2, ' ');

            let key = set_parts
                .next()
                .ok_or(ProtocolError::MissingKey)?;

            let value = set_parts
                .next()
                .ok_or(ProtocolError::MissingValue)?;

            Ok(Command::Set {
                key: key.to_string(),
                value: value.to_string(),
            })
        }

        "GET" => {

            // 1. Get the raw string segment containing the key
            let raw_key = split_data
                .next()
                .ok_or(ProtocolError::MissingKey)?;
            
            // 2. Parse it to ensure it contains exactly one word
            let mut t = raw_key.split_whitespace();
            
            let key = match (t.next(), t.next()) {
                (Some(val), None) => val.trim_start().to_string(), // Exactly one word found
                _ => return Err(ProtocolError::InvalidFormat(input.to_string())), 
            };
            
            // 3. Return the command successfully
            Ok(Command::Get { key })
        }

        "DELETE" => {
            let raw_key = split_data
                .next()
                .ok_or(ProtocolError::MissingKey)?;

            let mut t = raw_key.split_whitespace();

            let key = match (t.next(),t.next()) {
                (Some(val),None) => val.trim_start().to_string(),

                _ => return Err(ProtocolError::InvalidFormat(input.to_string())), 
            };

            Ok(Command::Delete {
                key: key.trim_start().to_string(),
            })
        }

        _ => Err(ProtocolError::UnknownCommand(command.to_string())),
    }
}