
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rt  = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(std::cmp::max(2, num_cpus::get_physical())) // Use all host cores, unless single-cored in which case pretend to have 2
    .thread_stack_size(8 * 1024 * 1024)
    .enable_time()
    .enable_io()
    .build()?;

  rt.block_on(async {
    if let Err(e) = main_async().await {
      eprintln!("[ main_async ] {}", e);
      std::process::exit(1);
    }
  });

  Ok(())
}

async fn main_async() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let default_server_url = "http://localhost:9050".to_string();
    let server_url = args.get(1).unwrap_or(&default_server_url);
    println!("server_url = {:?}", server_url);


    Ok(())
}