use clap::Parser;
use clap::ValueEnum;

#[derive(Parser)]
pub struct Configuration {
    #[arg(short, long, required = true, num_args = 1.. )]
    pub candidats: Vec<String>,

    #[arg(short, long, default_value_t = StorageType::Memory, value_enum)]
    pub storage: StorageType,
}

#[derive(Clone,Copy, ValueEnum)]
pub enum StorageType {
    File,
    Memory,
}