use jsonrpsee::server::Server;
use jsonrpsee::RpcModule;
use librime_rpc::methods;
use std::net::SocketAddr;
use std::sync::{Condvar, Mutex};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let port = 8001;
    let server = Server::builder()
        .build(SocketAddr::from(([0, 0, 0, 0], port)))
        .await?;
    let mut module = RpcModule::new(());

    macro_rules! register_method {
        ($name:literal, $f:expr) => {
            module.register_method($name, |p, _| $f(p))?;
        };
    }
    register_method!("version", methods::version);

    let _handle = server.start(module);

    // the RPC server will be shutdown on `handle` dropped; hangs the program
    let mutex = Mutex::new(());
    let _guard = Condvar::new().wait(mutex.lock().unwrap()).unwrap();

    Ok(())
}
