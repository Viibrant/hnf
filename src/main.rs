use hnf::Client;

fn main() -> anyhow::Result<()> {
    let client = Client::new();
    let ids = client.fetch_top_ids()?;
    println!("{:#?}", &ids[0..10]);
    Ok(())
}
