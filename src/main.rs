use KV::server::tcp_listener;


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






fn main(){

match tcp_listener() {
    Ok(_) => {},
    Err(err) => {
        eprintln!("This is the error {}",err);
    }
}
    
}
