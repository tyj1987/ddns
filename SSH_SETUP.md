# 🔑 添加 SSH 密钥到 GitHub

## 生成的 SSH 密钥

**公钥内容** (需要添加到 GitHub):
```
ssh-edd25519 AAAAC3NzaC1lZDI1NTE5AAAAINoxhYsRAReWU0jJuUR29h2vFNflqKAbj2snhVMEm68P tuoyongjun1987@qq.com
```

---

## 📋 添加步骤

### 1. 访问 GitHub SSH 设置页面

点击这个链接打开:
👉 **https://github.com/settings/keys**

### 2. 添加新的 SSH 密钥

1. 点击绿色按钮 **"New SSH key"**
2. 填写表单:
   - **Title**: `DDNS Tool Development`
   - **Key type**: `Authentication Key`
   - **Key**: 粘贴上面的公钥内容:
     ```
     ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAINoxhYsRAReWU0jJuUR29h2vFNflqKAbj2snhVMEm68P tuoyongjun1987@qq.com
     ```
3. 点击 **"Add SSH key"**

### 3. 验证添加成功

添加后,你应该在列表中看到:
- **DDNS Tool Development**
- 类型: `Authentication Key`
- 日期: 今天

---

## ✅ 添加完成后

回到终端,运行以下命令推送代码:

```bash
cd /home/tyj/ddns

# 测试 SSH 连接
ssh -T git@github.com

# 如果看到 "Hi tyj1987! You've successfully authenticated..." 则成功!

# 推送代码
git push -u origin main
```

---

## 🔧 如果推送失败

### 选项 1: 使用 HTTPS + Token

```bash
cd /home/tyj/ddns

# 切换到 HTTPS
git remote set-url origin https://github.com/tyj1987/ddns.git

# 推送 (会提示输入用户名和密码)
git push -u origin main
# 用户名: tyj1987
# 密码: 粘贴你的 GitHub Token
```

### 选项 2: 使用 Token 直接推送

```bash
cd /home/tyj/ddns

# 使用 Token 在 URL 中
git remote set-url origin https://tyj1987:YOUR_TOKEN@github.com/tyj1987/ddns.git

# 推送
git push -u origin main
```

注意: 将 `YOUR_TOKEN` 替换为你的实际 token。

---

## 🎉 推送成功后

1. **访问仓库**: https://github.com/tyj1987/ddns
2. **查看 CI/CD**: https://github.com/tyj1987/ddns/actions
3. **创建 Release**: https://github.com/tyj1987/ddns/releases/new

---

**快速复制公钥**:
```
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAINoxhYsRAReWU0jJuUR29h2vFNflqKAbj2snhVMEm68P tuoyongjun1987@qq.com
```
