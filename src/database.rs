use std::collections::HashMap;

pub(crate) struct Database {
    pub(crate) map: HashMap<String, String>,
}

impl Database {
   pub(crate) fn new() -> Self {
            Database {
                map: HashMap::new(),
            }
    }
    
   pub(crate) fn set(&mut self, key: String, value: String) {
             let _ = self.map.insert(key, value);     
    }

   pub(crate) fn get(&self, key: &str) -> Option<&String>{
        self.map.get(key)
    }

   pub(crate) fn delete(&mut self, key: &str) -> bool{
        match self.map.remove(key){
            Some(_) => {return true},
            
            None => {return false}
        }
        
    }
}