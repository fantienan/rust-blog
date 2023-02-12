# rust-blog
- 文章的CRUD
- github第三方登录
- 中间件实现身份验证
- 中间件
    - 客户端请求 -> handler -> 响应
    - 客户端请求 -> 中间件 -> handler -> 响应 
    - 请求或响应到达handler之前先经过中间件，中间件可以修改请求或响应
    - 应用: 身份验证、解压请求压缩响应、响应添加HTTP headers....