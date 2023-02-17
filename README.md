# rust-blog
- 文章的CRUD
- github第三方登录
- 中间件实现身份认证


##  中间件实现身份认证

- 中间件
    - 客户端请求 -> handler -> 响应
    - 客户端请求 -> 中间件 -> handler -> 响应 
    - 请求或响应到达handler之前先经过中间件，中间件可以修改请求或响应
    - 应用: 身份认证、解压请求压缩响应、响应添加HTTP headers....

## 更简单更灵活的身份认证方式
### FromRequest trait

- 实现了FromRequest trait 的类型可以从请求中被提取出来，这种类型在Rocket中叫request guard，在Actix Web中叫做extractor

- 怎么使用这个trait实现身份认证呢?
    - 效仿Rocket，我们先定义两个struct: User 和 Admin
    - 为它们实现FromRequest trait，从请求的cookies中提取登录时设置的ACCESS_TOKEN
    - 然后进行github oauth验证
    - 这样比中间件的方式更灵活，可以用于复杂的情况
