# 基于 Axum 封装的web框架

## 目录结构

```text
.
├── api //接口
├── biz // 业务逻辑的组装层，类似 DDD 的 domain 层，data 类似 DDD 的 repo，而 repo 接口在这里定义，使用依赖倒置的原则。
├── build.rs // grpc生成
├── Cargo.lock
├── Cargo.toml
├── config // 配置信息
├── configs // 配置相关代码 
├── data // 业务数据访问
├── kratos_core_rs // kratos-rs核心代码
├── LICENSE
├── proto // grpc proto文件
├── protos // proto生成文件
├── README.md
├── rustfmt.toml
├── rust-toolchain.toml
├── service // 实现了 api 定义的服务层，类似 DDD 的 application 层，处理 DTO 到 biz 领域实体的转换(DTO -> DO)，同时协同各类 biz 交互，但是不应处理复杂逻辑
├── src
└── target
```
