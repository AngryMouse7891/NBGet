---
name: version-bump
description: 根据语义版本调整Cargo.toml的版本并创建git tag。分析与上次git tag的差异，自动判定版本类型（major/minor/patch）。“升级版本”、“发布”、“version bump”、“版本升级”等请求启动。
---

# Version Bump

安装Cargo.toml版本并创建git tag。

## 工作流程

### 1. 分析当前状态

```bash
#获取最新版本标签
git describe --tags --abbrev=0 2>/dev/null || echo "v0.0.0"

#获取与前一个标签的差异
git diff <上回标签结果>..HEAD --stat
git diff <上回标签结果>..HEAD
```

### 2. 确定版本类型

分析差异内容，根据以下标准判断版本类型：

| 类别 | 判定标准 |
| ------ | ---------- |
| **major** | 破坏性变更：公共API的签名变更、删除、不兼容的变更 |
| **minor** | 新增功能：新增公共函数/结构/模块，新增功能 |
| **patch** | 错误修复：现有功能修复，重构，文档更新，依赖关系更新 |

### 3. 用户建议

将结果呈现给用户，并在AskUserQuestion中确认：

```
当前版本：X.Y.Z
建议版本：X.Y.Z +1 （patch）

主要变化：
-[更改摘要]

这个版本可以吗？
```

选项：
（patch/minor/major）
-选择不同的版本类型

### 4. 更新版本

用户授权后：

1. 更新`Cargo.toml`的`version`字段
2. 用`cargo check`检查
3. 提交更改：`chore: bump version to X.Y.Z `
4. 创建git tag: `vX.Y.Z`

### 5. 完成报告

```
将版本更新为v{新版本}。

请使用git push && git push --tags远程反映。
```

## 注意事项

- 标签带有前缀（例如v1.2.3）
- Cargo.lock会自动更新
- 工作区仅更新root的Cargo.toml