use hnf::Client;

fn main() -> anyhow::Result<()> {
    let client = Client::new();
    println!("{}", client);
    Ok(())
}
