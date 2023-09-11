use serde_json;
use crate::runner::rsprinter;
use crate::runner::rstrait::Runner;

pub struct RsJson;

impl Runner for RsJson {
    fn run(&self) {
        rsprinter::print_start();
        let path = "./assets/es_response.json";
        let data = std::fs::read_to_string(path).expect("Unable to read file");
        let body_json: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");

        // TODO: there must be a better way
        for (index, _all) in body_json["hits"]["hits"].as_array().iter().enumerate() {
            if body_json["hits"]["hits"][index]["_source"].is_object() {
                println!("--> {:?}", body_json["hits"]["hits"][index]["_source"]["blog_id"].to_string());
            }
        }
    }
}
