use rust_net::discovery::{
    DiscoveryInfo,
    DiscoveryService,
};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Example: Discovery");

    let action = match args.get(1) {
        Some(arg) => arg.as_str(),
        None => {
            print_usage();
            return;
        }
    };

    match action {
        "register" => {
            register(&args);
        },
        "query" => {
            query(&args);
        },
        _ => {
            print_usage();
        }
    } 
}

fn register(args: &[String]) { 
    let service = DiscoveryService::new().expect("failed to create service");

    let service_name = match args.get(2) {
        Some(arg) => format!("{}.local.", arg),
        None => {
            print_register_usage();
            return;
        }
    };

    let instance_name = match args.get(3) {
        Some(arg) => arg,
        None => {
            print_register_usage();
            return;
        }
    };

    let hostname = match args.get(4) {
        Some(arg) => format!("{}.local.", arg),
        None => {
            print_register_usage();
            return;
        }
    };
    
    let port = 3456;

    let info = DiscoveryInfo::new(
        &service_name,
        instance_name,
        &hostname,
        port,
    )
    .expect("failed to create service info");
}

fn query(args: &[String]) {
    let service = DiscoveryService::new().expect("failed to create service");

    let service_name = match args.get(2) {
        Some(arg) => format!("{}.local.", arg),
        None => {
            print_query_usage();
            return;
        }
    };

    let receiver = service.browse(&service_name).expect("failed to browse");

    let now = std::time::Instant::now();

    while let Ok(event) = receiver.recv() {
        match event {
            other_event => {
                println!("At {:?} {:?}", now.elapsed(), other_event);
            }
        }
    }
}

fn print_usage() {
    const HEADLINE: &str = "\x1b[1;32m";
    const TITLE: &str = "\x1b[1;36m";
    const BODY: &str = "\x1b[36m";
    const RESET: &str = "\x1b[0m";

    println!("\
        {HEADLINE}Usage:{RESET} \
        {TITLE}discovery{RESET} \
        {BODY}<register|query>{RESET} \
    ");
}

fn print_register_usage() {
    const HEADLINE: &str = "\x1b[1;32m";
    const TITLE: &str = "\x1b[1;36m";
    const BODY: &str = "\x1b[36m";
    const RESET: &str = "\x1b[0m";

    println!("\
        {HEADLINE}Usage:{RESET} \
        {TITLE}discovery{RESET} \
        {BODY}register{RESET} \
        {BODY}<SERVICE_TYPE>{RESET} \
        {BODY}<INSTANCE_NAME>{RESET} \
        {BODY}<HOSTNAME>{RESET} \
    ");
    println!();
    println!("  {BODY}<SERVICE_TYPE>{RESET}     mDNS service type (e.g., _http._tcp)");
    println!("  {BODY}<INSTANCE_NAME>{RESET}    Human-readable service name");
    println!("  {BODY}<HOSTNAME>{RESET}         Host providing the service");
}

fn print_query_usage() {
    const HEADLINE: &str = "\x1b[1;32m";
    const TITLE: &str = "\x1b[1;36m";
    const BODY: &str = "\x1b[36m";
    const RESET: &str = "\x1b[0m";

    println!("\
        {HEADLINE}Usage:{RESET} \
        {TITLE}discovery{RESET} \
        {BODY}query{RESET} \
        {BODY}<SERVICE_TYPE>{RESET} \
    ");
    println!();
    println!("  {BODY}<SERVICE_TYPE>{RESET}     mDNS service type (e.g., _http._tcp)");
}

