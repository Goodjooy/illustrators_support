# Illustrators Support 画师包养计划

- 组队画师包养计划收集器后端

## 前置要求

- Mysql>=8.0

## 注意

- 由于制作初期定义为小型服务器，只在小范围内使用，因此没有设置账户密码
- 服务由`Rust`+`Sea-orm`+`rocket`完成，使用时建议拥有`rust`基础

## 功能列表

- /user/new

  - 只要提供服务器所有者的邀请码，带上姓名和 QQ 号，就可以新建账户
  - 一个用户名和 qq 号只能注册一个账号
  - POST

  ```json
  {
    "name": "YouName[4,32]",
    "qq": 1141451919, //YouQQNumber
    "password": "YouPassword[6,16]",
    "invite_code": "InviteCode"
  }
  ```

- /user/login

  - 提供密码和 qq，即可登录
  - 没有登出，登录上别的号就登出了，服务器不保存登录 session
  - POST

  ```json
  {
    "qq": 1141451919, //YouQQNumber
    "password": "YouPassword[6,16]"
  }
  ```

- /illustrator/new

  - 向要包养的画师列表里面添加一个新的画师,要求有画师名称和画师包养的地址
  - 如果画师已经被添加了会添加失败
  - 需要 User 权限（user 登录后的 cookie）
  - POST

  ```json
  {
    "name": "IllustratorName[1,32]",
    "head": "IllustratorName[,256]",
    "home": "IllustratorHome[,256]",
    "sponser": "IllustratorSponserPage[,256]"
  }
  ```

  - Respond：
    ident:后续添加画师画作的引导标记,这个标记只有一个小时生命哦

- /illustrator/add_arts/\<ident>

  - 通过 ident 为指定的画师添加作品
  - ident 就是画师的 iid
  - 需要 User 权限（user 登录后的 cookie）
  - 请求为上传文件名列表
  - POST

  ```json
    [
      "xxx-xxx-xx-xx.png"
      "xxx-xxx-xx-xx.png"
      "xxx-xxx-xx-xx.png"
    ]
  ```

- /illustrator/all

  - 我全都要，获取全部画师简略信息
  - 本来应该有多页翻页啥的，但是没做
  - 需要 User 权限（user 登录后的 cookie）
  - GET
  - Respond:

  ```json
  [
    {
      "iid": 100, //IllustratorID
      "name": "IllustratorName",
      "head": "IllustratorName",
      "home": "IllustratorHome",
      "sponser": "IllustratorSponserPage",
      "wconts": 114 //wants counts
    }
  ]
  ```

- /illustrator/\<id>

  - 获取指定画师的信息
  - 除了基本信息，还有代表作列表和获得的想要组队投票者
  - 需要 User 权限（user 登录后的 cookie）
  - GET
  - Respond:

  ```json
  {
    "iid": 100, //"IllustratorID"
    "head": "IllustratorName",
    "home": "IllustratorHome",
    "sponser": "IllustratorSponserPage",
    "arts": [
      {
        "src": null,
        "file": "xxx-xx-xx-xx-xx.png"
      },
      {
        "src": null,
        "file": "xxx-xx-xx-xx-xx.png"
      }
    ],
    "wants": [{ "name": "WantsName", "qq": "WantsQQ" }]
  }
  ```

- /illustrator/<id\>

  - 欸嘿，想要包养这个画师嘛，那就投票吧
  - 似乎不能取消投票欸，不过问题不大
  - 需要 User 权限（user 登录后的 cookie）
  - POST
  - no body

- /admin/new

  - 新的管理员，注册时需要提供超级管理员验证码
  - 验证码在配置文件里面
  - POST

  ```json
  {
    "super_identify": "SuperUserIdentifyCode",
    "name": "AdminName[1,32]",
    "password": "AdminPassword[6,16]"
  }
  ```

- /admin/login

  - 管理员登录，需要提供账号密码
  - 管理员和用户权限没有重叠，想要投票可以再登录个用户
  - POST

  ```json
  {
    "name": "AdminName[1,32]",
    "password": "AdminPassword[6,16]"
  }
  ```

- /admin/audit/\<art_name>

  - 管理员审核作品（将作品标记为适合的)
  - art_name 为指定作品名称
  - 需要 Admin 权限（admin 登录后的 cookie）
  - POST

- /admin/invite

  - 管理员添加邀请码
  - 每次最多添加 3 个，最少 1 个，每个最长 32，最短 8
  - 需要 Admin 权限（admin 登录后的 cookie）
  - POST

  ```json
  {
    "codes": ["InviteCode1[8,36]", "InviteCode2", "InviteCode3"]
  }
  ```

- /images/\<path>
  - 还记得刚刚的画师作品列表嘛
  - 把文件名放进 path，就可以看到作品了
  - GET

- /images/upload
  - 文件上传接口
  - User 权限 需要
  - POST

  ```form
  src=[Option]https://www.abab.cccc&file=file_to_upload
  ```

  - Respond 
    - 上传的文件的文件名

## 允许上传文件类型

| 文件类型 |
| :------: |
|  .jpeg   |
|   .bmp   |
|  .tiff   |
|   .tga   |
|   .gif   |
|   .png   |
|   .ico   |
|   .cur   |

## 交叉编译整不出来，摸了

- GGG

## 启动配置文件

- `Rocket.toml`
  Rocket 启动配置文件：[配置方法](https://rocket.rs/v0.5-rc/guide/configuration/#configuration)
- `Config.toml`
  附加的配置文件

  ```toml
    [database]
    # 数据库 url 与sea-orm 配置里面的数据库类型保持一致
    url="db://db_user:db_password@db_host:db_port/db_name"
    # 数据库最大连接数
    max_conn=64
    # 数据库最小连接数
    min_conn=4

    [auth]
    # 超级管理员密码 注册管理员时使用
    super_admin_auth="11414519192-ff"

    [consts]
    # 上传文件保存目录
    save_dir="./SAVES/"

    # 该部分可缺省
    [invite_codes]
    # 启动时自动加入数据库的邀请码，[8,36]
    codes=[
      # 可以使用生成的uuid
      "475418ae-c313-4012-af8a-fea68ea61195",
      # 或者自定义
      "welcame to setu site"
      # 但是不允许重复
    ]
  ```

## 启动

- 执行 `pyhton3 script.py` 将`migrations`内sql 文件合并为`./sqls/data.sql`,向数据库添加数据表
- 在执行文件所在目录添加配置文件 `Rocket.toml` 和 `Config.toml`，并完成配置
- 当配置文件准备妥当后，直接运行可执行文件 `illustrators_support` 即可启动服务

## 本地编译

- `rust` stable
- `cargo build --release`
- `target/release/illustrators_support`
