use gumdrop::Options;

#[derive(Options)]
pub struct Args {
    #[options(help = "number of events")]
    pub number: Option<usize>,

    #[options(help = "output format")]
    pub format: Option<String>,

    #[options(help = "output file path")]
    pub output: Option<String>,

    #[options(help = "print help message")]
    pub help: bool,

    #[options(help = "print version")]
    pub version: bool,
}
