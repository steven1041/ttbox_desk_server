# GitHub Actions 自动构建和发布

## 工作流程说明

新的 GitHub Actions 工作流会自动编译 Linux 版本并发布到 GitHub Releases。

## 触发条件

工作流在以下情况下触发：

1. **推送标签**（推荐）
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

2. **手动触发**
   - 进入 GitHub 仓库的 Actions 页面
   - 选择 "Build and Release" 工作流
   - 点击 "Run workflow"

## 工作流程

### 1. 编译 Linux 二进制文件
- 在 Ubuntu 环境中编译
- 使用 Rust stable 工具链
- 启用缓存加速编译

### 2. 创建发布包
包含以下文件：
- `ttbox_salvo` - Linux 可执行文件
- `config.toml` - 配置文件
- `.env` - 环境变量模板
- `assets/` - 静态资源
- `views/` - 模板文件
- `deploy.sh` - 自动部署脚本

### 3. 上传到 GitHub Releases
- 自动创建 Release
- 上传 `ttbox_salvo-linux.tar.gz`
- 生成 Release Notes

## 使用方法

### 方法 1：使用标签触发（推荐）

```bash
# 1. 创建标签
git tag v1.0.0

# 2. 推送标签
git push origin v1.0.0

# 3. GitHub Actions 会自动构建并发布
```

### 方法 2：手动触发

1. 访问 GitHub 仓库
2. 点击 "Actions" 标签
3. 选择 "Build and Release" 工作流
4. 点击 "Run workflow" 按钮

## 下载和部署

### 下载发布包

1. 访问 GitHub 仓库的 "Releases" 页面
2. 找到对应的版本（如 v1.0.0）
3. 下载 `ttbox_salvo-linux.tar.gz`

### 部署到服务器

```bash
# 1. 上传到服务器
scp ttbox_salvo-linux.tar.gz user@server:/tmp/

# 2. 连接到服务器
ssh user@server

# 3. 解压
cd /tmp
tar -xzf ttbox_salvo-linux.tar.gz

# 4. 运行部署脚本
sudo ./deploy.sh
```

### 手动部署（不使用脚本）

```bash
# 1. 解压
tar -xzf ttbox_salvo-linux.tar.gz

# 2. 创建用户
sudo useradd -m -r ttbox

# 3. 创建目录
sudo mkdir -p /opt/ttbox
sudo mkdir -p /var/log/ttbox
sudo mkdir -p /etc/ttbox

# 4. 复制文件
sudo cp ttbox_salvo /opt/ttbox/
sudo chmod +x /opt/ttbox/ttbox_salvo
sudo chown ttbox:ttbox /opt/ttbox/ttbox_salvo

sudo cp -r assets /opt/ttbox/
sudo cp -r views /opt/ttbox/
sudo chown -R ttbox:ttbox /opt/ttbox/assets
sudo chown -R ttbox:ttbox /opt/ttbox/views

# 5. 配置文件
sudo cp config.toml /etc/ttbox/
sudo cp .env /etc/ttbox/.env

# 6. 创建 systemd 服务
sudo nano /etc/systemd/system/ttbox_salvo.service
```

systemd 服务配置：

```ini
[Unit]
Description=TTBox Salvo Application
After=network.target mysql.service

[Service]
Type=simple
User=ttbox
Group=ttbox
WorkingDirectory=/opt/ttbox
ExecStart=/opt/ttbox/ttbox_salvo
Restart=always
RestartSec=10
Environment="RUST_LOG=info"

# 安全设置
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/opt/ttbox /var/log/ttbox

[Install]
WantedBy=multi-user.target
```

启动服务：

```bash
sudo systemctl daemon-reload
sudo systemctl enable ttbox_salvo
sudo systemctl start ttbox_salvo
sudo systemctl status ttbox_salvo
```

## 版本号规范

推荐使用语义化版本号：

- `v1.0.0` - 主版本.次版本.修订版本
- `v1.0.1` - Bug 修复
- `v1.1.0` - 新功能（向后兼容）
- `v2.0.0` - 重大变更（不兼容）

## 配置说明

### GitHub Secrets

不需要任何 GitHub Secrets，工作流会自动创建 Release。

### 服务器配置

部署前需要配置：

1. **编辑配置文件**
   ```bash
   sudo nano /etc/ttbox/config.toml
   ```

2. **编辑环境变量**
   ```bash
   sudo nano /etc/ttbox/.env
   ```

3. **配置数据库连接**
   ```bash
   # .env 文件
   DATABASE_URL=mysql://user:password@localhost:3306/database_name
   ```

## 服务管理

```bash
# 启动服务
sudo systemctl start ttbox_salvo

# 停止服务
sudo systemctl stop ttbox_salvo

# 重启服务
sudo systemctl restart ttbox_salvo

# 查看状态
sudo systemctl status ttbox_salvo

# 查看日志
sudo journalctl -u ttbox_salvo -f

# 开机自启
sudo systemctl enable ttbox_salvo

# 禁用开机自启
sudo systemctl disable ttbox_salvo
```

## 更新应用

```bash
# 1. 下载新版本
# 从 GitHub Releases 下载最新的 ttbox_salvo-linux.tar.gz

# 2. 上传到服务器
scp ttbox_salvo-linux.tar.gz user@server:/tmp/

# 3. 连接到服务器
ssh user@server

# 4. 停止服务
sudo systemctl stop ttbox_salvo

# 5. 备份当前版本
sudo cp /opt/ttbox/ttbox_salvo /opt/ttbox/ttbox_salvo.backup

# 6. 解压并部署
cd /tmp
tar -xzf ttbox_salvo-linux.tar.gz
sudo ./deploy.sh

# 7. 启动服务
sudo systemctl start ttbox_salvo

# 8. 检查状态
sudo systemctl status ttbox_salvo
```

## 回滚

```bash
# 停止服务
sudo systemctl stop ttbox_salvo

# 恢复备份
sudo mv /opt/ttbox/ttbox_salvo.backup /opt/ttbox/ttbox_salvo

# 启动服务
sudo systemctl start ttbox_salvo
```

## 常见问题

### Q: 如何查看构建状态？

A: 访问 GitHub 仓库的 Actions 页面，查看 "Build and Release" 工作流的运行状态。

### Q: 构建失败怎么办？

A: 查看构建日志，检查错误信息。常见问题：
- 代码编译错误
- 依赖版本冲突
- 测试失败

### Q: 如何手动触发构建？

A: 进入 GitHub Actions 页面，选择工作流，点击 "Run workflow"。

### Q: Release 创建失败？

A: 确保仓库有创建 Release 的权限。检查：
- Settings → Actions → General → Workflow permissions
- 启用 "Read and write permissions"

### Q: 如何删除旧的 Release？

A: 进入 Releases 页面，点击 Release 的 "..." 菜单，选择 "Delete release"。

## 优势

✅ **自动化**：推送标签自动构建和发布
✅ **版本管理**：清晰的版本号和 Release Notes
✅ **下载方便**：直接从 GitHub 下载
✅ **部署简单**：包含自动部署脚本
✅ **回滚容易**：保留备份文件
✅ **持续集成**：每次推送都会验证代码

## 下一步

1. 创建并推送标签：
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

2. 等待 GitHub Actions 完成

3. 从 Releases 页面下载发布包

4. 部署到服务器

5. 测试应用是否正常运行
