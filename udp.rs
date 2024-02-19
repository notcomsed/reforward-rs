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

use tokio::net::UdpSocket;
use std::error::Error;

fn helpf() {
    println!("reforward udp version");
    println!("  -d, --direct start");
	println!("  -v, --version");
	println!("  -h, --help");
	println!("reforward -d [listen] [listen Port] [connect] [connect Port] [debug=0]");
	println!("Examples:");
	println!("reforward -d 0.0.0.0 5050 192.168.1.1 500"); 
	println!("\"[deny IP 1]\"=\"0\" mean NOT deny"); 
	println!("Deny:");
	println!("udp version don't support deny options in the application layer,because of too wasted performance during ddos attack, please using iptable instead");
	println!("Use IPv6:");
	println!("reforward -d :: 5050 192.168.1.1 500");
	println!("reforward -d fd20::1 5050 192.168.1.1 500");
	println!("Debug:");
	println!("reforward -d 0.0.0.0 5050 192.168.1.1 500 1"); 
    std::process::exit(0);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let args: Vec<String> = std::env::args().collect();
	let mut debuk=false;
	
	println!("reforward-rs running in {}, udp enabled.", args[0]);

    if args.len() > 1 {
	
	if args[1] == "-h" {
	helpf();
	std::process::exit(0);
	}
	if args[1] == "-v" {
	println!("reforward v3.4.7 udp");
	std::process::exit(0);
	}
	
	if args[1] != "-d" {
		println!("command error");
		std::process::exit(0);
	}
	
	if args.len()<=6 {
		println!("Segmentation fault");
		std::process::exit(-1);
		}
	if args.len()>=6 {
	if args[6] == "1" {debuk = true;}
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
		
		let s1 = format!("{}:{}", &ipaddress,&args[3]);
		let d1 = format!("{}:{}", &ipaddres,&args[5]);
	let source_addr = s1.clone();
	let destination_addr = d1.clone();
	println!("UDP forwarder running. Forwarding from {} to {}", s1, d1);
    let source_socket = UdpSocket::bind(&source_addr).await?;
    let destination_socket = UdpSocket::bind("0.0.0.0:0").await?;
    let mut buf = vec![0u8; 2048]; 
	if debuk {println!("Debug mode: ON");}
    loop {
        let (len, src) = source_socket.recv_from(&mut buf).await?;
        let sent = destination_socket.send_to(&buf[..len], &destination_addr).await?;
        if debuk {println!("转发 {} 字节数据到目标地址: {}", sent, destination_addr);}
        let (len, _) = destination_socket.recv_from(&mut buf).await?;
        let sent_back = source_socket.send_to(&buf[..len], &src).await?;
        if debuk {println!("将响应数据 {} 字节转发回源地址: {}", sent_back, src);}
    }
}

