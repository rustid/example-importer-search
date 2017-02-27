extern crate example_importer_search;
extern crate rs_es;

use rs_es::operations::index;
use rs_es::Client;

fn make_client(url: &str) -> rs_es::Client {
    let hostname = url.to_owned();
    Client::new(&hostname).unwrap()
}

fn import() -> Result<bool, String>{
    //create elasticsearch client
    let mut client = make_client("http://localhost:9200");

    //prepare data
    let data = example_importer_search::AddressDocument::new().with_address("Kota kediri").with_name("Gian Giovani");

    //import to elasticsearch
    let result_wrapped = client.index("address","default")
        .with_doc(&data)
        .with_id("1")
        .with_op_type(index::OpType::Create)
        .send();
    println!("TEST RESULT: {:?}", result_wrapped);
    match result_wrapped {
        Ok(_) => {
            Ok(true)
        },
        Err(_) => {
            println!("Failed to export");
            Err(std::string::String::from("failed to export"))
        },
    }
}

fn bulk_import() ->Result<bool,String> {
    Ok(true)
}

fn main() {
    match import() {
        Ok(_) => {
            println!("imported");
        },
        Err(_) => {
            println!("shit happened");
        } ,
    };
}