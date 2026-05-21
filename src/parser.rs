use crate::database::Database;

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





pub(crate) fn execute_command(command:Command,db:&mut Database)->std::io::Result<String>{
    match command {
        Command::Set{key,value} => {
            db.set(key,value);
            Ok("Ok".to_string())
               
            
        },
        Command::Get { key } => {
            match db.get(&key){
                Some(data) => {
                    let response = format!("Value: {}",data);
                    Ok(response)
                    
                },
                None => {
                    Ok("NOT_FOUND".to_string())
                }
            }
        },
        Command::Delete { key } =>{
            if db.delete(&key){
                Ok("Ok".to_string())
            } else{
                Ok("NOT_FOUND".to_string())
            }
        }
    }
    
}



pub(crate) fn process_command(input: &str) -> Result<Command, String> {
    // println!("COMMAND: {}", input);

    
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
                key: key.trim_start().to_string(),
            })
        }

        "DELETE" => {
            let key = split_data
                .next()
                .ok_or("DELETE requires key".to_string())?;

            Ok(Command::Delete {
                key: key.trim_start().to_string(),
            })
        }

        _ => Err("Unknown command".to_string()),
    }
}