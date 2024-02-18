use tokio::io;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use std::sync::Mutex;
use once_cell::sync::Lazy;
// 使用 Lazy 包装器来初始化 reforward
static REFORWARD: Lazy<Mutex<String>> = Lazy::new(|| {
    Mutex::new(String::from("reforward-rs"))
});
async fn 处理转发函数(mut myaddr: TcpStream) {
    let remote_addr = {
        let lock = REFORWARD.lock().unwrap();
        // Clone the string to break the lifetime dependency on the lock
        let addr = (*lock).clone();
        drop(lock); // 手动drop掉锁
        addr
    };
   let mut dest = match TcpStream::connect(remote_addr).await {
                Ok(socket) => socket,
                Err(e) => {
                    eprintln!("Error occurred: {}", e);
                    return;
                }
            };	
            let (mut ri, mut wi) = myaddr.split();
            let (mut ro, mut wo) = dest.split();

            let client_to_server = io::copy(&mut ri, &mut wo);
            let server_to_client = io::copy(&mut ro, &mut wi);

            match tokio::try_join!(client_to_server, server_to_client) {
                Ok(_) => {
                    println!("Connection closed");
					return;
                }
                Err(e) => {
                    eprintln!("Error occurred: {}", e);
					return;
                }
            }
}

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

#[tokio::main]
async fn main() {

let args: Vec<String> = std::env::args().collect();

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
		let deny_ip1 = &args[7];
	//println!("Listening on: {}", listen_addr);
    //println!("Forwarding to: {}", dest_addr);
    let listener = TcpListener::bind(listen_addr).await.unwrap();
	
    {
    let mut data = REFORWARD.lock().unwrap();
    *data = String::from(dest_addr);
    }
    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
			 if *deny_ip1 == addr.ip().to_string() {
                    eprintln!("deny connect ip: {:?}", addr);
                    continue;
                }
				
                tokio::spawn(处理转发函数(socket));
            }
            Err(e) => {
                println!("Accept error = {:?}", e);
            }
        }
    }
}
