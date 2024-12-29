# --------------------------------------------------
# 第 1 阶段：构建阶段 (Builder)
# --------------------------------------------------
    FROM rust:alpine AS builder

    # 安装编译所需依赖
    RUN apk add --no-cache \
        build-base \
        clang \
        pkgconfig \
        openssl-dev \
        openssl-libs-static
    
    # 如果你想强制 OpenSSL 走静态链接
    ENV OPENSSL_STATIC=1 \
        PKG_CONFIG_ALL_STATIC=1
    
    # 设置工作目录
    WORKDIR /work
    
    # --------- 复制并预编译依赖（可选做法，利用缓存） ---------
    # 先复制 Cargo.toml 和 Cargo.lock，进行依赖预编译
    COPY Cargo.toml Cargo.lock ./
    RUN cargo build --release || true
    
    # --------- 复制全部源码并正式编译 ---------
    COPY . .
    # 若你只需要 x86_64-musl，可指定：
    RUN cargo build --release
    # 如果想一次性编译多个目标，也可再加一行：
    # RUN cargo build --release --target aarch64-unknown-linux-musl
    
    # --------------------------------------------------
    # 第 2 阶段：运行阶段 (Runtime)
    # --------------------------------------------------
    FROM alpine:latest AS runtime
    
    # 如果需要 HTTPS 等功能，安装 CA 证书
    RUN apk add --no-cache ca-certificates
    
    # 设置工作目录
    WORKDIR /app
    
    # 从构建阶段复制编译产物到运行环境
    # 若你编译的是 x86_64-musl，就复制对应的二进制
    COPY --from=builder /work/target/release/twitter_mini_rs /usr/local/bin/my_app
    
    # 暴露端口（如果你的应用在 8080 端口监听）
    EXPOSE 3000
    
    # 容器启动时的默认命令
    CMD ["/usr/local/bin/my_app"]