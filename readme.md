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

![SpeedTest image](https://github.com/notcomsed/reforward-rs/assets/91320192/4ccb5131-b46b-456f-8098-609a576a770c)


##### reforward-rs与reforward (C version)的性能差了一点，应该是误差。

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
