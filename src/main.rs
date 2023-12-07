use clap::Parser;

mod auth;

#[derive(Parser)]
struct Cli {
    domain: String,
    action: String,
    args: Vec<String>,
}

fn main() {
    let pattern = std::env::args().nth(1).expect("no domain given. expected: domain action [args]");
    let path = std::env::args().nth(2).expect("no action given. expected: domain action [args]");

    let cli = Cli {
        domain: pattern,
        action: path,
        args: std::env::args().skip(3).collect(),
    };

    // println!("domain: {}", cli.domain);
    // println!("action: {}", cli.action);
    // println!("args: {:?}", cli.args);

    match cli.domain.as_str() {
        "auth" => match cli.action.as_str() {
            "get-demo-token" => auth::demo_token().unwrap(),
            _ => println!("unknown action: {}", cli.action),
        },
        _ => println!("unknown domain: {}", cli.domain),
    }
}
