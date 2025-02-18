// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]
use aptos_config::config::NodeConfig;
use hex::FromHex;
use rand::{rngs::StdRng, SeedableRng};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Aptos Node")]
struct Args {
    #[structopt(
        short = "f",
        long,
        required_unless = "test",
        help = "Path to NodeConfig"
    )]
    config: Option<PathBuf>,
    #[structopt(long, help = "Enable a single validator testnet")]
    test: bool,

    #[structopt(
        long,
        help = "RNG Seed to use when starting single validator testnet",
        parse(try_from_str = FromHex::from_hex),
        requires("test")
    )]
    seed: Option<[u8; 32]>,

    #[structopt(long, help = "Enabling random ports for testnet", requires("test"))]
    random_ports: bool,

    #[structopt(
        long,
        help = "Paths to module blobs to be included in genesis. Can include both files and directories",
        requires("test")
    )]
    genesis_modules: Option<Vec<PathBuf>>,

    #[structopt(
        long,
        help = "Lazy mode, set this flag will set `consensus#mempool_poll_count` config to `u64::MAX` and only commit a block when there is user transaction in mempool",
        requires("test")
    )]
    lazy: bool,
}

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() {
    let args = Args::from_args();

    if args.test {
        println!("Entering test mode, this should never be used in production!");
        let rng = args
            .seed
            .map(StdRng::from_seed)
            .unwrap_or_else(StdRng::from_entropy);
        let genesis_modules = if let Some(module_paths) = args.genesis_modules {
            framework::load_modules_from_paths(&module_paths)
        } else {
            cached_framework_packages::module_blobs().to_vec()
        };
        aptos_node::load_test_environment(
            args.config,
            args.random_ports,
            args.lazy,
            genesis_modules,
            rng,
        );
    } else {
        let config = NodeConfig::load(args.config.unwrap()).expect("Failed to load node config");
        println!("Using node config {:?}", &config);
        aptos_node::start(config, None);
    };
}
