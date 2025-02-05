use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
  name = "makepad-analyzer",
  version
)]
struct MakepadAnalyzer {}

#[tokio::main]
async fn main(){
  MakepadAnalyzer::parse();
  makepad_analyzer_server::start().await;
}
