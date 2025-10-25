# axum_demo 记录学习

---

> 关于我
> 博客：[无](http:)

主要用于学习，随便记录一些东西

- 统一的响应体
- 统一的异常枚举（可配置部分异常对用户可见）
- 中间件处理trace_id
- tracing 记录日志
- 定时任务
- redis工具类
- dao数据库操作层

# 构建步骤

```shell
# 构建前端
cd frontend && npm run build
# 构建二进制文件
cargo run build

```

## 使用 GitHub Actions 构建多平台产物

```shell

# 示例见文件夹 .github/workflows

# 触发生成 releases 
# 1、创建标签
git tag v0.1.0
# 2、推送标签
git push origin v0.1.0

# 删除本地 tag
git tag -d v0.1.0
# 删除远程 tag
git push origin --delete v0.1.0

```

# 使用sear-orm-cli生成entity

```
# 安装工具
cargo install sea-orm-cli@^2.0.0-rc   
# 生成 entity
sea-orm-cli generate entity --output-dir ./src/entity --entity-format dense --with-serde both --model-extra-attributes 'serde(rename_all="camelCase")' --with-prelude none

```
