```shell
sqlx database create
sqlx database drop

 sqlx migrate run 
```

协议说明:
mqtt
事件上报
/{productId}/{deviceCode}/event

```json
{
  "header": {
    "parent": "Optional[String] 父级编号,如果是主设备不需要填写",
    "productKey": "Optional[String] 设备不允许接入时,需要填写",
    "timestamp": "数据事件戳",
    "message_id": "消息编号"
  },
  "body": {
    "identifier": "事件标识符",
    "data": [
      {
        "identifier": "标识符",
        "value": "数据值"
      }
    ]
  }
}
```

事件回复(服务端回复)
/{productId}/{deviceCode}/event_reply

```json
{
  "header": {
    "timestamp": "数据事件戳",
    "message_id": "消息编号"
  },
  "code": "事件回复码"
}
```

事件回复码

```json
{
  "0": "成功",
  "1": "时间错误",
  "2": "数据格式错误",
  "3": "设备不存在,需要传递productKey添加"
}
```

遗嘱消息格式
/{productId}/{deviceCode}/offline|online
如果是独立设备,直接注册遗嘱消息,做为离线, 如果是子设备,需要通过主设备操作上下线,默认情况下和主设备同步上下线

```json
{
  "timestamp": "数据事件戳",
  "message_id": "消息编号",
  "parent": "Optional[String]  子设备离线和上线时,需要填写,"
}
```

指令下发
/{productKey}/{deviceCode}/service

```json
{
  "header": {
    "timestamp": "数据事件戳",
    "message_id": "消息编号"
  },
  "body": {
    "identifier": "服务标识符",
    "data": [
      {
        "identifier": "参数标识符",
        "value": "参数值值"
      }
    ]
  }
}
```

指令回复
/{productKey}/{deviceCode}/service

```json
{
  "header": {
    "timestamp": "数据事件戳",
    "message_id": "消息编号"
  },
  "body": {
    "identifier": "服务响应标识符",
    "data": [
      {
        "identifier": "参数标识符",
        "value": "参数值值"
      }
    ]
  }
}
```