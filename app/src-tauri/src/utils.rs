
pub fn stop_app(msg: &str) -> ! {
    println!("{}", msg);
    std::process::exit(1)
  }