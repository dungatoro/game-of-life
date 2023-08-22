use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct ConwaysArgs {
    /// Dead cell representation
    #[arg(short, long, default_value_t=' ')]
    pub dead: char,

    /// Live cell representation
    #[arg(short, long, default_value_t='@')]
    pub live: char,

    /// Time in ms between each tick
    #[arg(short, long, default_value_t=50)]
    pub tick_rate: i32,
}
