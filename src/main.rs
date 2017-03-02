extern crate example_importer_search;
extern crate rs_es;

use rs_es::operations::index;
use rs_es::operations::bulk::Action;
use rs_es::Client;

fn make_client(url: &str) -> rs_es::Client {
    let hostname = url.to_owned();
    Client::new(&hostname).unwrap()
}

fn import() -> Result<bool, String>{
    //create elasticsearch client
    let mut client = make_client("http://192.168.100.16:9200");

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

fn bulk_import() {
    let mut client = make_client("http://192.168.100.16:9200");

    let mut actions: Vec<Action<example_importer_search::AddressDocument>> = vec![];
    actions.push(Action::index(example_importer_search::AddressDocument::new()
                .with_address("Kota Malang")
                .with_name("Tumenggung Sudarsono")
                .with_id(2))
                .with_id("2")
                .with_index("address").with_doc_type("default"));

    actions.push(Action::index(example_importer_search::AddressDocument::new()
                .with_address("Kota Yogyakarta")
                .with_name("Slamet Raharjo")
                .with_id(2))
                .with_id("3")
                .with_index("address").with_doc_type("default"));

    client.bulk(&actions).send().unwrap();
}

fn main() {
    bulk_import();
    match import() {
        Ok(_) => {
            println!("imported");
        },
        Err(_) => {
            println!("shit happened");
        } ,
    };
}