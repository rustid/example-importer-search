extern crate rs_es;
extern crate serde;

#[macro_use]
extern crate serde_json;
#[macro_use] 
extern crate serde_derive;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressDocument {
    pub name:  String,
    pub address: String,
    pub id: i64
}

impl AddressDocument {
    pub fn new() -> AddressDocument {
        AddressDocument {
            name: "".to_owned(),
            address: "".to_owned(),
            id: 1
        }
    }
    pub fn with_name(mut self, doc_name: &str) -> AddressDocument {
        self.name = doc_name.to_owned();
        self
    }
    pub fn with_address(mut self, doc_address: &str) -> AddressDocument {
        self.address = doc_address.to_owned();
        self
    }
    pub fn with_id(mut self, doc_id: i64) -> AddressDocument {
        self.id = doc_id;
        self
    }
}