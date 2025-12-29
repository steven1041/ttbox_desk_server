@echo off
REM Docker 编译 Linux 版本脚本

echo 🐳 使用 Docker 编译 Linux 版本...

REM 检查 Docker 是否运行
docker info >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ Docker 未运行或未安装
    echo 请先安装并启动 Docker Desktop
    pause
    exit /b 1
)

REM 构建镜像
echo 🔨 构建镜像...
docker build -f Dockerfile.compile -t ttbox_salvo:linux .

if %errorlevel% neq 0 (
    echo ❌ 镜像构建失败
    pause
    exit /b 1
)

REM 创建临时容器提取文件
echo 📦 提取二进制文件...
docker create --name ttbox_temp ttbox_salvo:linux

docker cp ttbox_temp:/app/ttbox_salvo .\ttbox_salvo
docker cp ttbox_temp:/app/config.toml .\config.toml.docker

if exist assets (
    docker cp ttbox_temp:/app/assets .\assets.docker
)

if exist views (
    docker cp ttbox_temp:/app/views .\views.docker
)

REM 删除临时容器
docker rm ttbox_temp

echo ✅ 编译完成!
echo 📁 输出文件: .\ttbox_salvo

REM 创建发布目录
if exist release-linux rmdir /s /q release-linux
mkdir release-linux

copy ttbox_salvo release-linux\
copy config.toml.docker release-linux\config.toml
if exist .env.example copy .env.example release-linux\

if exist assets.docker (
    xcopy /E /I /Y assets.docker release-linux\assets
)

if exist views.docker (
    xcopy /E /I /Y views.docker release-linux\views
)

REM 清理临时文件
if exist assets.docker rmdir /s /q assets.docker
if exist views.docker rmdir /s /q views.docker
del config.toml.docker

REM 打包
echo 📦 创建发布包...
tar -czf ttbox_salvo-linux.tar.gz -C release-linux .

echo.
echo 🎉 构建完成!
echo 📦 发布包: ttbox_salvo-linux.tar.gz
echo.
echo 部署方法:
echo 1. 上传到服务器: scp ttbox_salvo-linux.tar.gz user@server:/tmp/
echo 2. 解压: tar -xzf ttbox_salvo-linux.tar.gz
echo 3. 运行: ./ttbox_salvo

pause
