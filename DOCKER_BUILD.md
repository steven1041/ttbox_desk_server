# 使用 Docker 编译 Linux 版本

使用 Docker 编译可以避免在 Windows 上安装交叉编译工具链的问题，这是最简单和可靠的方法。

## 前置要求

1. 安装 Docker Desktop for Windows
   - 下载地址：https://www.docker.com/products/docker-desktop
   - 安装后启动 Docker Desktop

2. 确保 Docker 正在运行
   ```cmd
   docker info
   ```

## 快速开始

### Windows 用户

```cmd
# 直接运行编译脚本
docker-build.bat
```

### Linux/Mac 用户

```bash
# 赋予执行权限
chmod +x docker-build.sh

# 运行编译脚本
./docker-build.sh
```

## 编译过程

编译脚本会自动完成以下步骤：

1. **构建 Docker 镜像**
   - 使用 Rust 1.89 基础镜像
   - 安装必要的依赖（pkg-config, libssl-dev）
   - 编译项目

2. **提取编译结果**
   - 从 Docker 容器中提取二进制文件
   - 提取配置文件和资源文件

3. **创建发布包**
   - 打包所有必要文件
   - 生成 `ttbox_salvo-linux.tar.gz`

## 输出文件

编译成功后会生成：

```
ttbox_salvo/
├── ttbox_salvo          # Linux 可执行文件
├── config.toml          # 配置文件
├── .env.example         # 环境变量示例
├── assets/              # 静态资源
└── views/               # 模板文件

ttbox_salvo-linux.tar.gz  # 发布包
```

## 部署到 Linux 服务器

### 方法 1：使用发布包（推荐）

```bash
# 1. 上传到服务器
scp ttbox_salvo-linux.tar.gz user@server:/tmp/

# 2. 连接到服务器
ssh user@server

# 3. 解压
cd /tmp
tar -xzf ttbox_salvo-linux.tar.gz

# 4. 安装
sudo cp ttbox_salvo /usr/local/bin/
sudo chmod +x /usr/local/bin/ttbox_salvo

# 5. 创建目录
sudo mkdir -p /opt/ttbox
sudo mkdir -p /var/log/ttbox

# 6. 复制文件
sudo cp -r assets /opt/ttbox/
sudo cp -r views /opt/ttbox/
sudo cp config.toml /etc/ttbox/

# 7. 运行
ttbox_salvo
```

### 方法 2：直接使用 Docker 镜像

```bash
# 1. 保存镜像
docker save ttbox_salvo:linux | gzip > ttbox_salvo-docker.tar.gz

# 2. 上传到服务器
scp ttbox_salvo-docker.tar.gz user@server:/tmp/

# 3. 在服务器上加载镜像
ssh user@server
docker load < /tmp/ttbox_salvo-docker.tar.gz

# 4. 运行容器
docker run -d \
  --name ttbox_salvo \
  -p 8008:8008 \
  -v /opt/ttbox/config.toml:/app/config.toml \
  -v /opt/ttbox/logs:/app/logs \
  ttbox_salvo:linux
```

## Dockerfile 说明

### 构建阶段

```dockerfile
FROM rust:1.89 as builder
```

- 使用官方 Rust 镜像
- 安装编译依赖（pkg-config, libssl-dev）
- 编译项目

### 运行时阶段

```dockerfile
FROM debian:bookworm-slim
```

- 使用轻量级 Debian 镜像
- 只安装运行时依赖（ca-certificates, libssl3）
- 从构建阶段复制二进制文件
- 创建非 root 用户

## 手动编译

如果不想使用自动化脚本，可以手动执行：

```bash
# 1. 构建镜像
docker build -f Dockerfile.compile -t ttbox_salvo:linux .

# 2. 运行容器
docker run --rm -v $(pwd):/output ttbox_salvo:linux \
  bash -c "cp /app/ttbox_salvo /output/"

# 3. 提取文件
docker create --name ttbox_temp ttbox_salvo:linux
docker cp ttbox_temp:/app/ttbox_salvo ./
docker rm ttbox_temp
```

## 优化编译

### 使用缓存加速编译

```bash
# 利用 Docker 缓存层
docker build -f Dockerfile.compile -t ttbox_salvo:linux .
```

### 多阶段构建优化

Dockerfile 已经使用多阶段构建，可以显著减小最终镜像大小。

## 常见问题

### Q: Docker 镜像太大怎么办？

A: 使用 `.dockerignore` 文件排除不必要的文件：

```dockerignore
# .dockerignore
target/
.git/
.env
logs/
*.md
```

### Q: 编译速度慢？

A: 使用 BuildKit 加速：

```bash
export DOCKER_BUILDKIT=1
docker build -f Dockerfile.compile -t ttbox_salvo:linux .
```

### Q: 如何调试编译错误？

A: 进入容器调试：

```bash
docker run -it --rm \
  -v $(pwd):/app \
  rust:1.89 \
  bash
```

### Q: 如何指定 Rust 版本？

A: 修改 Dockerfile 中的基础镜像：

```dockerfile
FROM rust:1.75 as builder
```

## CI/CD 集成

### GitHub Actions 示例

```yaml
name: Build and Deploy

on:
  push:
    branches: [ main ]

jobs:
  build:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Build Docker image
      run: |
        docker build -f Dockerfile.compile -t ttbox_salvo:linux .
    
    - name: Extract binary
      run: |
        docker create --name ttbox_temp ttbox_salvo:linux
        docker cp ttbox_temp:/app/ttbox_salvo ./
        docker rm ttbox_temp
    
    - name: Upload artifact
      uses: actions/upload-artifact@v4
      with:
        name: ttbox_salvo-linux
        path: ttbox_salvo
```

## 性能对比

| 方法 | 编译时间 | 复杂度 | 推荐度 |
|------|----------|----------|----------|
| Docker | 5-10 分钟 | 低 | ⭐⭐⭐⭐⭐ |
| WSL2 | 3-5 分钟 | 中 | ⭐⭐⭐⭐ |
| 交叉编译 | 10-20 分钟 | 高 | ⭐⭐ |

## 总结

**Docker 方案的优势：**
- ✅ 不需要安装额外工具
- ✅ 环境一致，避免"在我机器上能跑"问题
- ✅ 可以直接生成 Docker 镜像用于部署
- ✅ 适合 CI/CD 自动化
- ✅ 跨平台兼容

**推荐使用 Docker 方案进行编译和部署！**
