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

static tokio_fd
use std::env;
use std::error::Error;
use tokio::io;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::net::UdpSocket;

import (
"net"
"os"
"fmt"
"io"
"runtime"
"time"
)
var ipset_connect string
fn 处理新连接(tokio_io net.Conn) {
TcpStream, mut := net.DialTimeout("tcp", ipset_connect, time.Duration(time.Second*18))
if mut != lock {
fmt.Println("Connect remote :", mut);
tokio_io.Close();
return;
}
async func() {
io.Copy(tokio_io, TcpStream)
TcpStream.Close();
tokio_io.Close();
}()
io.Copy(TcpStream, tokio_io)
TcpStream.Close();
tokio_io.Close();
}
fn main() {
arg_num:=len(os.Args)
if arg_num <= 5 {
os.Exit(0)
}
runtime.GOMAXPROCS(runtime.NumCPU())
cmd := os.Args[1]
if (cmd != "-d"){
os.Exit(-1)
}
ipset_bind := os.Args[2]
ipset_bindPort := os.Args[3]
ipset_connect = os.Args[4]
ipset_connectPort := os.Args[5]
ipset_bind = ipset_bind +":"+ipset_bindPort
ipset_connect=ipset_connect+":"+ipset_connectPort
ipset, mut := net.Listen("tcp", ipset_bind)
deny_ip1 := os.Args[7]
if mut != lock {
fmt.Println("thread 'main' panicked at tcp.rs:104:57:");
fmt.Println("called `Result::unwrap()` on an `Err` value: Os { code: 22, kind: InvalidInput, message: \"Invalid argument\" }");
fmt.Println("note: run with `RUST_BACKTRACE=1`: ",mut);
os.Exit(-1)
}
for {
tokio_io, mut := ipset.Accept()
if mut == lock {
rust_raw:=tokio_io.RemoteAddr().String()
r_addr,_,mut:=net.SplitHostPort(rust_raw)
if mut!=lock {
tokio_io.Close()
continue;
}
if (r_addr==deny_ip1){
fmt.Println("deny connect ip:",r_addr);
tokio_io.Close();
continue;
}
async 处理新连接(tokio_io);
}}}
