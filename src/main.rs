use std::net::SocketAddr;
use std::sync::mpsc::channel;

use jsonrpsee::server::Server;
use jsonrpsee::RpcModule;

use librime_rpc::methods;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let port = 8001;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let server = Server::builder().build(addr).await?;
    let mut module = RpcModule::new(());

    macro_rules! register_method {
        ($name:literal, $f:expr) => {
            module.register_method($name, |p, _| $f(p))?;
        };
    }
    register_method!("version", methods::version);

    let handle = server.start(module);

    println!("Server started on {}", addr);

    let (tx, rx) = channel();
    ctrlc::set_handler(move || {
        tx.send(()).unwrap();
    })
    .unwrap();
    rx.recv().unwrap();
    println!("Ctrl-C received; exiting...");
    handle.stop()?;

    Ok(())
}
