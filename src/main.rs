use named_pipe::{PipeOptions, PipeClient};
use std::io::{BufReader, BufWriter, BufRead, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("pipes <server|client>");
        std::process::exit(1);
    }

    match args[1].as_str() {
       "server" => {
           run_server();
       },
       "client" => {
           run_client();
       },
       _ => {},
    }
}

fn run_client() {
    let client = PipeClient::connect("\\\\.\\pipe\\MyPipe")
        .unwrap();
    println!("Connected to pipe");
    let mut s = String::new(); 
    let mut writer = BufWriter::new(client);
    loop {
        print!("|> ");
        std::io::stdin().read_line(&mut s).unwrap();
        writer.write_all(s.as_ref()).unwrap();
        writer.flush().unwrap();
        s.clear();
    }
}

fn run_server() {
    let server = PipeOptions::new("\\\\.\\pipe\\MyPipe")
        .single().unwrap();
    println!("Pipe open");

    let pipe = server.wait().unwrap();
    println!("Client connected");
    let mut reader = BufReader::new(pipe);
    let mut s = String::new();
    loop {
        reader.read_line(&mut s).unwrap();
        print!("Client says: {}", s);
        s.clear();
    }
}


