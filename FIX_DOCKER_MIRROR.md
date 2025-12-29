# 修复 Docker 镜像源问题

## 问题描述

错误信息显示 Docker 镜像源（网易镜像）出现 502 错误：
```
failed to resolve source metadata for docker.io/library/debian:bookworm-slim
unexpected status from HEAD request to http://hub-mirror.c.163.com/v2/library/debian/manifests/bookworm-slim?ns=docker.io: 502 Bad Gateway
```

## 解决方案

### 方案 1：修改 Dockerfile 使用稳定版本（已应用）

已将 `debian:bookworm-slim` 改为 `debian:12-slim`，这是更稳定的版本号。

### 方案 2：配置 Docker 镜像加速器

#### Windows (Docker Desktop)

1. 打开 Docker Desktop
2. 进入 **Settings** → **Docker Engine**
3. 修改配置：

```json
{
  "registry-mirrors": [
    "https://docker.mirrors.ustc.edu.cn",
    "https://hub-mirror.c.163.com",
    "https://mirror.ccs.tencentyun.com"
  ],
  "dns": ["8.8.8.8", "114.114.114.114"]
}
```

4. 点击 **Apply & Restart**

#### Linux

编辑 `/etc/docker/daemon.json`：

```json
{
  "registry-mirrors": [
    "https://docker.mirrors.ustc.edu.cn",
    "https://mirror.ccs.tencentyun.com"
  ]
}
```

重启 Docker：
```bash
sudo systemctl restart docker
```

### 方案 3：手动拉取基础镜像

```bash
# 拉取基础镜像
docker pull debian:12-slim

# 验证镜像
docker images | grep debian
```

### 方案 4：使用 Alpine Linux（更小更快）

修改 [`Dockerfile.compile`](Dockerfile.compile:1)：

```dockerfile
# 构建阶段
FROM rust:1.89 as builder

WORKDIR /app

# 安装必要的工具
RUN apk add --no-cache \
    pkgconf \
    openssl-dev \
    musl-dev

# 复制项目文件
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migration ./migration
COPY assets ./assets
COPY views ./views
COPY config.toml ./

# 编译 Linux 版本
RUN cargo build --release

# 输出阶段
FROM alpine:latest

WORKDIR /app

# 安装运行时依赖
RUN apk add --no-cache \
    ca-certificates \
    libssl3

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/ttbox_salvo /app/
COPY --from=builder /app/assets /app/assets
COPY --from=builder /app/views /app/views
COPY --from=builder /app/config.toml /app/

# 创建非 root 用户
RUN addgroup -g 1000 appuser && \
    adduser -D -u 1000 -G appuser appuser && \
    chown -R appuser:appuser /app

USER appuser

EXPOSE 8008

CMD ["./ttbox_salvo"]
```

### 方案 5：使用本地缓存

```bash
# 清理 Docker 缓存
docker system prune -a

# 重新构建
docker build -f Dockerfile.compile -t ttbox_salvo:linux --no-cache .
```

### 方案 6：临时禁用镜像加速器

如果镜像加速器不稳定，可以临时禁用：

#### Windows (Docker Desktop)

1. 打开 Docker Desktop
2. 进入 **Settings** → **Docker Engine**
3. 删除或注释掉 `registry-mirrors` 配置
4. 点击 **Apply & Restart**

#### Linux

编辑 `/etc/docker/daemon.json`，删除 `registry-mirrors` 配置：

```json
{
  "dns": ["8.8.8.8", "114.114.114.114"]
}
```

重启 Docker：
```bash
sudo systemctl restart docker
```

## 推荐的 Docker 镜像源

| 镜像源 | 地址 | 状态 |
|---------|------|------|
| 中科大 | https://docker.mirrors.ustc.edu.cn | ⭐⭐⭐⭐⭐ |
| 腾讯云 | https://mirror.ccs.tencentyun.com | ⭐⭐⭐⭐ |
| 网易 | https://hub-mirror.c.163.com | ⭐⭐⭐ |
| 阿里云 | https://registry.cn-hangzhou.aliyuncs.com | ⭐⭐⭐⭐ |

## 验证修复

### 1. 测试 Docker 连接

```bash
# 测试拉取镜像
docker pull hello-world

# 运行测试
docker run hello-world
```

### 2. 重新构建项目

```cmd
# Windows
docker-build.bat
```

```bash
# Linux/Mac
./docker-build.sh
```

### 3. 检查 Docker 日志

```bash
# 查看构建日志
docker logs ttbox_salvo
```

## 常见问题

### Q: 镜像拉取超时？

A: 检查网络连接，尝试：
```bash
# 测试网络
ping docker.io

# 使用代理（如果需要）
export HTTP_PROXY=http://proxy:port
export HTTPS_PROXY=http://proxy:port
```

### Q: 镜像拉取失败？

A: 尝试手动拉取：
```bash
docker pull debian:12-slim
```

### Q: 构建缓存问题？

A: 清理缓存重新构建：
```bash
docker system prune -a
docker build -f Dockerfile.compile -t ttbox_salvo:linux --no-cache .
```

### Q: DNS 解析问题？

A: 修改 Docker DNS 配置：

#### Windows (Docker Desktop)
Settings → Docker Engine → 添加 DNS：
```json
{
  "dns": ["8.8.8.8", "1.1.1.1", "114.114.114.114"]
}
```

#### Linux
编辑 `/etc/docker/daemon.json`：
```json
{
  "dns": ["8.8.8.8", "1.1.1.1"]
}
```

## 快速修复步骤

1. **修改 Docker 镜像源**（推荐）
   - 使用中科大或腾讯云镜像源
   - 重启 Docker

2. **手动拉取基础镜像**
   ```bash
   docker pull debian:12-slim
   ```

3. **重新构建**
   ```cmd
   docker-build.bat
   ```

4. **如果仍然失败，使用 Alpine**
   - 修改 Dockerfile 使用 alpine 基础镜像
   - 重新构建

## 总结

**推荐操作顺序：**
1. ✅ 修改 Dockerfile 使用 `debian:12-slim`（已完成）
2. ✅ 配置稳定的镜像加速器（中科大或腾讯云）
3. ✅ 手动拉取基础镜像验证
4. ✅ 重新构建项目

如果问题仍然存在，建议使用 Alpine Linux 基础镜像，它更小且更稳定。
