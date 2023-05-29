# 基于 Axum 封装的web框架

不管是在Debug编译模式还是Release编译模式，编译好的二进制档都会带有调试信息。Unix-like环境下，通过Release编译模式编译出来的二进制档，可以再通过strip指令，将其中不必要的标头和调试信息移除。

strip ./target/release/express