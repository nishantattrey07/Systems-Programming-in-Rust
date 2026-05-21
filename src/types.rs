#[derive(Debug)]
pub enum ProtocolError {
    EmptyCommand,
    UnknownCommand(String),
    MissingKey,
    MissingValue,
    InvalidFormat(String),
}

impl ProtocolError {
   pub fn to_response(&self) -> String {
        match self {
            ProtocolError::MissingKey =>
                "ERR missing key".to_string(),

            ProtocolError::MissingValue =>
                "ERR missing value".to_string(),

            ProtocolError::UnknownCommand(data) =>{
                let data = format!("ERR unknown command <{}>",data);
                data 
            },

            ProtocolError::InvalidFormat(data) => {
                let data = format!("ERR invalid command format <{}>",data);
                data 
            },

            ProtocolError::EmptyCommand =>
                "ERR empty command".to_string()
       
    }
  }
}