use colored::Colorize;

pub fn demo_token() -> Result<(), Box<dyn std::error::Error>> {
  // call https://api.differential.dev/demo and return the result

  let resp = reqwest::blocking::get("https://api.differential.dev/demo/token")?
    .text()?;

  println!("Created demo token: {}", resp.green());
  println!("Note: this token is only valid for 1 hour. For a permanent token, use `differential auth get-token`");

  Ok(())
}