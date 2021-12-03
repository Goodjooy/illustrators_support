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
         "name":YouName,
         "qq":YouQQNumber,
         "invite_code": InviteCode
     }
  ```

- /user/login

  - 提供姓名和 qq，即可登录（没有密码验证，不要被别人知道你在这）
  - 没有登出，登录上别的号就登出了
  - POST

  ```json
      {
          "name":YouName,
          "qq":YouQQNumber,
      }
  ```

- /illustrator/new

  - 向要包养的画师列表里面添加一个新的画师,要求有画师名称和画师包养的地址
  - 如果画师已经被添加了会添加失败
  - POST

  ```json
      {
          "name":IllustratorName,
          "home":IllustratorHome
      }
  ```

  - Respond：
    ident:后续添加画师画作的引导标记

- /illustrator/add_arts/<ident\>

  - 通过 ident 为指定的画师添加作品（是不是还要有个清理 ident 的接口？）
  - ident 就是先前 new 时的响应体中的 uuid
  - 请求体是原始文件二进制（不是 form!!）
  - POST  
    body->file to upload  
    **记得在`Content-Type`中指明文件类型，否则如果依靠文件头匹配不到类型将上传失败**

- /illustrator/all
  - 我全都要，获取全部画师简略信息
  - 本来应该有多页翻页啥的，但是没做
  - GET
  - Respond:
  ```json
  [
      {
          "iid":IllustratorID,
          "name":IllustratorName,
          "home":IllustratorHome
      }
  ]
  ```
- /illustrator/<id>
  - 获取指定画师的信息
  - 除了基本信息，还有代表作列表和获得的想要组队投票者（话说，这个投票者信息是不是就可以拿去登录了？）
  - GET
  - Respond:
  ```json
  {
      "iid":IllustratorID,
      "name":IllustratorName,
      "home":IllustratorHome,
      "arts":[
            "xxx-xx-xx-xx-xx.png",
            "xxx-xx-xx-xx-xx.jpg",
            "xxx-xx-xx-xx-xx.gif"
      ],
        "wants":[
            [
                WantsName,
                WantsQQ
            ]
        ]
  }
  ```

* /illustrator/<id\>

  - 欸嘿，想要包养这个画师嘛，那就投票吧
  - 似乎不能取消投票欸，不过问题不大
  - POST
  - no body

* /images/<path..>
  - 还记得刚刚的画师作品列表嘛
  - 把文件名放进 path，就可以看到作品了
  - GET

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