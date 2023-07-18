Lecture 03
=============

## Project 内容

基于 Rust 的基本数据结构写一个简单的学生管理系统（比如，学生，社团，班级、课程等），明确类型之间的关系，进行基本的CRUD操作


## 项目依赖
* actix-web
* diesel
* sqlite


## 项目运行
### 编译执行
```shell
> cd lecture_03
> cargo build
> cargo run
> ./runme.py
```

### curl 命令操作
```shell
# index
> curl -i -X GET -H "Content-Type: application/json" http://localhost:8111/students
# get 
> curl -i -X GET -H "Content-Type: application/json" http://localhost:8111/students/<sid>
# post 
> curl -i -X POST -H "Content-Type: application/json" -d '{"sid":"004", "name": "yyy"}' http://localhost:8111/students
# delete
> curl -i -X DELETE -H  "Content-Type: application/json" http://localhost:8111/students/<sid> 
```


## API Reference

API Root: http://localhost:8111

### Students 
#### 列表信息
```api
GET /api/v1/students
```
```json
[
    {
        "id": 1,
        "sid": "002",
        "name": "Jerry",
        "created_at": "2023-07-18T13:08:45",
        "updated_at": "2023-07-18T13:08:45"
    },
    {
        "id": 2,
        "sid": "001",
        "name": "Tom",
        "created_at": "2023-07-18T13:09:02",
        "updated_at": "2023-07-18T13:09:02"
    },
    {
        "id": 3,
        "sid": "003",
        "name": "Goffi",
        "created_at": "2023-07-18T13:09:13",
        "updated_at": "2023-07-18T13:09:13"
    }
]
```
#### 创建学生项
```api
POST /api/v1/students
{"sid":"001", "name": "Tom"}
```
```json
{
    "id": 1,
    "sid": "001",
    "name": "Tom",
    "created_at": "2023-07-18T13:31:47",
    "updated_at": "2023-07-18T13:31:47"
}
```
#### 获取学生信息
```api
GET /api/v1/students/002
```
```json
{
    "id": 2,
    "sid": "001",
    "name": "Tom",
    "created_at": "2023-07-18T13:09:02",
    "updated_at": "2023-07-18T13:09:02"
}
```
#### 删除学生项
```api
DELETE /api/v1/students/001
```

### Class
#### 班级列表信息
```api
GET /api/v1/classes
```
```json
[
    {
        "id": 1,
        "sid": "002",
        "name": "Jerry",
        "created_at": "2023-07-18T13:08:45",
        "updated_at": "2023-07-18T13:08:45"
    },
    {
        "id": 2,
        "sid": "001",
        "name": "Tom",
        "created_at": "2023-07-18T13:09:02",
        "updated_at": "2023-07-18T13:09:02"
    },
    {
        "id": 3,
        "sid": "003",
        "name": "Goffi",
        "created_at": "2023-07-18T13:09:13",
        "updated_at": "2023-07-18T13:09:13"
    }
]
```
#### 创建班级项
```api
POST /api/v1/classes
{"sid":"001", "name": "Tom"}
```
```json
{
    "id": 1,
    "sid": "001",
    "name": "Tom",
    "created_at": "2023-07-18T13:31:47",
    "updated_at": "2023-07-18T13:31:47"
}
```
#### 获取班级信息
```api
GET /api/v1/classes/3-001
```
```json
{
    "class_id": "3-001",
    "name": "高三1班",
    "stu_list": [
        {
            "id": 1,
            "sid": "001",
            "name": "Tom",
            "created_at": "2023-07-18T13:31:47",
            "updated_at": "2023-07-18T13:31:47"
        },
        {
            "id": 2,
            "sid": "002",
            "name": "Jerry",
            "created_at": "2023-07-18T13:31:52",
            "updated_at": "2023-07-18T13:31:52"
        }
    ]
}
```
#### 删除班级项
```api
DELETE /api/v1/classes/3-001
```


### StuClass
学生班级关系注册接口
#### 注册信息列表
```api
GET /api/v1/stucls
```

```json
[
    {
        "id": 1,
        "sid": "001",
        "clsid": "3-001",
        "comment": null,
        "created_at": "2023-07-18T13:57:23",
        "updated_at": "2023-07-18T13:57:23"
    },
    {
        "id": 2,
        "sid": "002",
        "clsid": "3-001",
        "comment": null,
        "created_at": "2023-07-18T13:57:30",
        "updated_at": "2023-07-18T13:57:30"
    },
    {
        "id": 3,
        "sid": "003",
        "clsid": "3-002",
        "comment": null,
        "created_at": "2023-07-18T13:57:33",
        "updated_at": "2023-07-18T13:57:33"
    }
]
```

#### 注册班级学生信息
```api
POST localhost:8111/api/v1/stucls
{"stu_id": "003", "class_id":"3-002", "commment": "高三1班"}
```
```json
{
    "id": 3,
    "sid": "003",
    "clsid": "3-002",
    "comment": null,
    "created_at": "2023-07-18T13:57:33",
    "updated_at": "2023-07-18T13:57:33"
}
```


## TODO
* 加入社团信息
* 加入课程信息

## 总结
通过该项目，基本理解 Rust 项目的构建，简单学生注册项目 web 服务搭建，数据库 sqlite 的访问，已远超出 lecture 03 中相关概念的要求

## Reference
* [Actix-web](https://actix.rs/)
* [Diesel](https://diesel.rs/)