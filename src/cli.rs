#[derive(Debug, clap::Parser)]
#[command(
    author = "Zhai Can <bczhc0@126.com>",
    about = "JSON-RPC server for librime"
)]
pub struct Args {
    #[arg(
        short,
        long,
        help = "Port the server listens on",
        default_value = "8000"
    )]
    pub port: u16,
    #[arg(help = "librime user data directory")]
    pub user_data_dir: String,
    #[arg(help = "librime shared data directory")]
    pub shared_data_dir: String,
}
