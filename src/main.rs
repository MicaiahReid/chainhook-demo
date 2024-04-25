mod server;

use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::sync::{Arc, RwLock};

use chainhook_sdk::chainhooks::types::ChainhookConfig;
use chainhook_sdk::chainhooks::types::{BitcoinChainhookFullSpecification, ChainhookSpecification};
use chainhook_sdk::types::BitcoinNetwork;
use server::gql::Context as GraphContext;
use server::start_server;

#[tokio::main]
async fn main() {
    let json_predicate = std::fs::read_to_string("./predicate.json").expect("Unable to read file");
    // let json_predicate = std::env::var("PREDICATE").expect("Missing required predicate");

    let hook_spec: BitcoinChainhookFullSpecification =
        serde_json::from_str(&json_predicate).expect("unable to parse chainhook spec");

    let bitcoin_network = BitcoinNetwork::Regtest;
    let stacks_network = chainhook_sdk::types::StacksNetwork::Mainnet;

    let mut bitcoin_hook_spec = hook_spec
        .into_selected_network_specification(&bitcoin_network)
        .expect("unable to parse bitcoin spec");
    bitcoin_hook_spec.enabled = true;

    let mut chainhook_config = ChainhookConfig::new();
    chainhook_config
        .register_specification(ChainhookSpecification::Bitcoin(bitcoin_hook_spec))
        .unwrap();

    let config = chainhook_sdk::observer::EventObserverConfig {
        chainhook_config: Some(chainhook_config),
        bitcoin_rpc_proxy_enabled: false,
        // only for stacks, right? can we remove?
        ingestion_port: 0, //chainhook_sdk::observer::DEFAULT_INGESTION_PORT,
        bitcoind_rpc_username: "regtest".to_string(),
        bitcoind_rpc_password: "test1235".to_string(), // 1235 instead of 1234 to keep the hackers on their toes
        bitcoind_rpc_url: "http://0.0.0.0:8332".to_string(),
        // how will the bitcoin node notify chainhook of new blocks?
        bitcoin_block_signaling: chainhook_sdk::types::BitcoinBlockSignaling::ZeroMQ(
            "tcp://0.0.0.0:18543".to_string(),
        ),
        display_logs: true,
        // is this only used for stacks, can we remove?
        cache_path: String::new(),
        bitcoin_network: bitcoin_network,
        // can we remove? only used for stacks
        stacks_network: stacks_network,

        data_handler_tx: None,
        prometheus_monitoring_port: Some(20457),
    };
    let (observer_commands_tx, observer_commands_rx) = channel();

    // set up context to configure how we display logs from the event observer
    let logger = hiro_system_kit::log::setup_logger();
    let _guard = hiro_system_kit::log::setup_global_logger(logger.clone());
    let ctx = chainhook_sdk::utils::Context {
        logger: Some(logger),
        tracer: false,
    };

    let moved_ctx = ctx.clone();
    let _ = hiro_system_kit::thread_named("Chainhook event observer")
        .spawn(move || {
            let future = chainhook_sdk::observer::start_bitcoin_event_observer(
                config,
                observer_commands_tx,
                observer_commands_rx,
                None,
                None,
                moved_ctx,
            );
            match hiro_system_kit::nestable_block_on(future) {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e)
                }
            }
        })
        .expect("unable to spawn thread");

    let gql_context = GraphContext {
        data: Arc::new(RwLock::new(HashMap::new())),
    };
    println!("starting server");
    let _ = start_server(gql_context, 8545, &ctx).await;
    let (tx, rx) = channel();
    match rx.recv() {
        Ok(_) => {}
        Err(_) => {}
    }
    let _ = tx.send(true);
}
