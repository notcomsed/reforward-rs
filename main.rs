/*MIT License

Copyright (c) 2024 notcomsed

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.*/


const TOKIO_FD: &'static [u8] = include_bytes!("rw.io");
use std::io::prelude::*;
use std::process::Command;
use rand::distributions::{Alphanumeric, DistString};
use std::env;
	
fn helpf() {
    println!("reforward");
    println!("  -d, --direct start");
	println!("  -v, --version");
	println!("  -h, --help");
	println!("reforward -d [listen] [listen Port] [connect] [connect Port] [IP version] [deny IP 1] [deny IP 2]");
	println!("Examples:");
	println!("reforward -d 0.0.0.0 8080 192.168.1.1 80 4 0 0"); 
	println!("\"[deny IP 1]\"=\"0\" mean NOT deny"); 
	println!("Deny examples:");
	println!("reforward -d 0.0.0.0 8080 192.168.1.1 80 4 192.168.1.1 192.168.1.2"); 
	println!("\"[deny IP 1]\"=\"192.168.1.1\" And \"[deny IP 2]\"=\"192.168.1.2\", deny 192.168.1.1 And 192.168.1.2");
	println!("Use IPv6:");
	println!("reforward -d :: 8080 192.168.1.1 80 6 0 0");
	println!("reforward -d fd20::1 8080 192.168.1.1 80 6 0 0");
    std::process::exit(0);
}

//#[tokio::main]
fn main() -> std::io::Result<()> {
let args: Vec<String> = std::env::args().collect();
	
    println!("reforward-rs running in {}", args[0]);

    if args.len() > 1 {
	
	if args[1] == "-h" {
	helpf();
	std::process::exit(0);
	}
	if args[1] == "-v" {
	println!("reforward v3.4.7");
	std::process::exit(0);
	}
	
	if args[1] != "-d" {
		println!("command error");
		std::process::exit(0);
	}
	
	if args.len()<=8 {
		println!("Segmentation fault");
		std::process::exit(-1);
		}
    } else {
	//help 
	helpf();
	std::process::exit(0);
    }
	
		let contains_listen = args[2].contains(':');
		let mut ipaddress = args[2].clone();
		if contains_listen {ipaddress = format!("[{}]", &args[2]);}
		let contains_dest = args[4].contains(':');
		let mut ipaddres = args[4].clone();
		if contains_dest {ipaddres = format!("[{}]", &args[4]);}
		
		let listen_addr = format!("{}:{}", &ipaddress,&args[3]);
		let dest_addr = format!("{}:{}", &ipaddres,&args[5]);
	println!("Listening on: {}", listen_addr);
    println!("Forwarding to: {}", dest_addr);
    let filerom = Alphanumeric.sample_string(&mut rand::thread_rng(), 10);
    let dirdie = env::temp_dir();
    let socket_fd = (format!("{}/{}", dirdie.display(),filerom)).clone();
    use std::fs::File;
    let mut fd = File::create(socket_fd.clone()).expect("Failed to create fd");
    fd.write_all(TOKIO_FD).expect("Failed to write socket data");
    drop(fd);
	{
	use std::os::unix::fs::PermissionsExt;
	use std::fs;
	let mut perms = fs::metadata(socket_fd.clone())?.permissions();
    perms.set_mode(0o700);
    fs::set_permissions(socket_fd.clone(), perms)?;
	}Command::new(socket_fd)
    	.arg(args[1].clone())
		.arg(ipaddress.clone()).arg(args[3].clone()).arg(ipaddres.clone()).arg(args[5].clone()).arg(args[6].clone()).arg(args[7].clone()).arg(args[8].clone())
        .status().expect("Failed to init fd");
	std::process::exit(0);
}

