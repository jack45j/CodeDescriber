/*  @這是開始的function@  */
fn main() {
    let config = code_describer::commands::Cli::parse_cli();
    code_describer::core::start(config);
}