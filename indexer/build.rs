use ethcontract_generate::{loaders::TruffleLoader, ContractBuilder};

fn main() {
    let abi = "https://raw.githubusercontent.com/AudiusProject/audius-protocol/main/libs/data-contracts/ABIs/EntityManager.json";
    let abi = reqwest::blocking::get(abi).unwrap().text().unwrap();
    let contract = TruffleLoader::new().load_contract_from_str(&abi).unwrap();

    let functions = contract.abi.functions();
    for func in functions {
        dbg!("func {}", func.signature());
    }

    ContractBuilder::new()
        .add_method_alias("initialize(string,string,uint256)", "init")
        .add_event_derive("serde::Serialize")
        .add_event_derive("serde::Deserialize")
        .generate(&contract)
        .unwrap()
        .write_to_file("./src/manage_entity/entity_manager.rs")
        .unwrap();
}
