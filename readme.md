# Reforward-rs
Rust version for reforward [https://github.com/notcomsed/reforward](https://github.com/notcomsed/reforward)

using tokio library

### build

```bash
make
```

### Usage
```
reforward -d [listen] [listen Port] [connect] [connect Port] [IP version] [deny IP 1] [deny IP 2]
```

Same as reforward(C version) [https://github.com/notcomsed/reforward](https://github.com/notcomsed/reforward)


## iperf SpeedTest

#### direct iperf
```
------------------------------------------------------------
 Client connecting to 127.0.0.1, TCP port 8888
 
 TCP window size: 2.50 MByte (default)
 
------------------------------------------------------------

[  3] local 127.0.0.1 port 51902 connected with 127.0.0.1 port 8888

[ ID] Interval       Transfer     Bandwidth

[  3] 0.0000-10.0020 sec  57.8 GBytes  49.6 Gbits/sec

```

----------

#### rinetd

```
------------------------------------------------------------
Client connecting to 127.0.0.1, TCP port 8088

TCP window size: 4.00 MByte (default)

------------------------------------------------------------

[  3] local 127.0.0.1 port 56576 connected with 127.0.0.1 port 8088

[ ID] Interval       Transfer     Bandwidth

[  3] 0.0000-30.0031 sec  10.6 GBytes  3.02 Gbits/sec
```

-------------------

#### reforward (C version)

```
------------------------------------------------------------
Client connecting to 127.0.0.1, TCP port 8080

TCP window size: 2.50 MByte (default)

------------------------------------------------------------

[  3] local 127.0.0.1 port 48900 connected with 127.0.0.1 port 8080

[ ID] Interval       Transfer     Bandwidth

[  3] 0.0000-2.0001 sec  1.89 GBytes  8.14 Gbits/sec
```


--------------

#### reforward-rs

```
------------------------------------------------------------
Client connecting to 127.0.0.1, TCP port 8080

TCP window size: 2.50 MByte (default)

------------------------------------------------------------

[  3] local 127.0.0.1 port 48900 connected with 127.0.0.1 port 8080

[ ID] Interval       Transfer     Bandwidth

[  3] 0.0000-10.0067 sec  23.7 GBytes  20.4 Gbits/sec




------------------------------------------------------------
Client connecting to 127.0.0.1, TCP port 8080

TCP window size: 2.50 MByte (default)

------------------------------------------------------------

[  3] local 127.0.0.1 port 48900 connected with 127.0.0.1 port 8080

[ ID] Interval       Transfer     Bandwidth

[  3] 0.0000-10.0065 sec  24.2 GBytes  20.8 Gbits/sec
```

##### 可以看出来,reforward-rs与reforward (C version)的性能没差多少。差了一点，应该是误差。

## Warning
> [!CAUTION]
> reforward-rs is not static compilation like reforward(C version), the running environment depends on the compiled environment's glibc version.
> If the glic of the compilation environment is too high, a lower version of glibc will not be able to run.
>
> reforward-rs并不像reforward(C version)一样使用静态编译，运行环境会依赖于编译环境的glibc版本。如果编译环境的glic太高，在glibc低的版本就无法运行。


## Releases

> [!IMPORTANT]
> 本项目的Releases使用github-actions自动编译发布，请确保你的运行环境不低于github-actions的glibc环境。
> 如果本地的glic环境远低于github-actions的环境，请下载到本地手动`make`编译
>
> The releases of this project are automatically compiled and released using github-actions. Please ensure that your running environment is not lower than the glibc environment of github-actions.
> If the local glic environment is much lower than the github-actions environment, please download the manual `make` compilation locally.
