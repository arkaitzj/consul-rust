use std::collections::HashMap;
use agent::Agent;
use catalog::Catalog;
use health::Health;
use structs::{Service, HealthService};
use rustc_serialize::json;


#[test]
pub fn test_agent() {
    let agent1 = Agent::new("127.0.0.1:8500");
    let map: HashMap<String, Service> = agent1.services();
    assert!(map.contains_key("consul"));
}

#[test]
pub fn test_catalog(){
    let catalog1 = Catalog::new("127.0.0.1:8500");
    let map: HashMap<String, Vec<String>> = catalog1.services();
    assert!(map.contains_key("gsearch"));
}


#[test]
pub fn test_health(){
    let health = Health::new("127.0.0.1:8500");
    let list1: Vec<HealthService> = health.service("gsearch", "release");
    assert_eq!(list1.len(), 1);
    let list2: Vec<HealthService> = health.service("redis", "release2");
    assert_eq!(list2.len(), 0);
    println!("{:?}", json::encode(&list1));
}
