### 上报属性事件

```json
{
  "header": {
    "message_id": 1,
    "timestamp": 123333
  },
  "body": {
    "identifier": "pressureUp",
    "data": [
      {
        "identifier": "pressure",
        "value": 30
      },
      {
        "identifier": "status",
        "value": 1
      }
    ]
  }
}
```