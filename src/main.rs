use anyhow::anyhow;
use clap::Parser;
use std::net::SocketAddr;
use std::sync::mpsc::channel;

use jsonrpsee::server::{Server, ServerHandle};
use jsonrpsee::RpcModule;
use rime_api::{create_session, full_deploy_and_wait, initialize, setup, DeployResult, Traits};

use librime_rpc::cli::Args;
use librime_rpc::{cli, methods, mutex_lock, RIME_SESSION};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();

    println!("Initializing librime...");
    init_rime(&args)?;

    let server = start_server(args.port).await?;

    let (tx, rx) = channel();
    ctrlc::set_handler(move || {
        tx.send(()).unwrap();
    })
    .unwrap();
    rx.recv().unwrap();
    println!("Ctrl-C received; exiting...");
    server.stop()?;

    Ok(())
}

fn init_rime(args: &Args) -> anyhow::Result<()> {
    let mut traits = Traits::new();
    traits.set_shared_data_dir(&args.shared_data_dir);
    traits.set_user_data_dir(&args.user_data_dir);
    traits.set_distribution_name("Rime-RPC");
    traits.set_distribution_code_name("Rime-RPC");
    traits.set_distribution_version("1.0.0");
    traits.set_app_name("Rime-RPC");

    setup(&mut traits);
    initialize(&mut traits);

    let deploy_result = full_deploy_and_wait();
    match deploy_result {
        DeployResult::Success => {
            println!("Deployment done");
        }
        DeployResult::Failure => {
            return Err(anyhow!("Deployment failed"));
        }
    }

    let session = create_session()?;

    mutex_lock!(RIME_SESSION).replace(session);

    Ok(())
}

async fn start_server(port: u16) -> anyhow::Result<ServerHandle> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let server = Server::builder().build(addr).await?;
    let mut module = RpcModule::new(());

    macro_rules! register_method {
        ($name:literal, $f:expr) => {
            module.register_method($name, |p, _| $f(p))?;
        };
    }
    register_method!("version", methods::version);
    register_method!("simulate_key_sequence", methods::simulate_key_sequence);

    let handle = server.start(module);

    println!("Server started on {}", addr);

    Ok(handle)
}
