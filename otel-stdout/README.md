# otel-stdout

Original: https://crates.io/crates/tracing-opentelemetry

```shell
cargo run
```

```json
{
  "resourceSpans": [
    {
      "resource": {
        "attributes": [
          {
            "key": "service.name",
            "value": {"stringValue": "unknown_service"}
          }
        ]
      },
      "scopeSpans": [
        {
          "scope": {
            "name": "readme_example"
          },
          "spans": [
            {
              "traceId": "61c83c8226e1bf8465f27247c9ec024d",
              "spanId": "76e80accefd6b094",
              "parentSpanId": "",
              "name": "app_start",
              "kind": 1,
              "startTimeUnixNano": 1698575515924038000,
              "endTimeUnixNano": 1698575515924392000,
              "attributes": [
                {
                  "key": "thread.name",
                  "value": {"stringValue": "main"}
                },
                {
                  "key": "code.filepath",
                  "value": {"stringValue": "otel-stdout/src/main.rs"}
                },
                {
                  "key": "thread.id",
                  "value": {"intValue": 1}
                },
                {
                  "key": "idle_ns",
                  "value": {"intValue": 296958}
                },
                {
                  "key": "work_units",
                  "value": {"intValue": 2}
                },
                {
                  "key": "busy_ns",
                  "value": {"intValue": 50667}
                },
                {
                  "key": "code.namespace",
                  "value": {"stringValue": "otel_stdout"}
                },
                {
                  "key": "code.lineno",
                  "value": {"intValue": 24}
                }
              ],
              "droppedAttributesCount": 0,
              "events": [
                {
                  "name": "This event will be logged in the root span.",
                  "attributes": [
                    {
                      "key": "level",
                      "value": {"stringValue": "ERROR"}
                    },
                    {
                      "key": "target",
                      "value": {"stringValue": "otel_stdout"}
                    },
                    {
                      "key": "code.filepath",
                      "value": {"stringValue": "otel-stdout/src/main.rs"}
                    },
                    {
                      "key": "code.namespace",
                      "value": {"stringValue": "otel_stdout"}
                    },
                    {
                      "key": "code.lineno",
                      "value": {"intValue": 27}
                    }
                  ],
                  "droppedAttributesCount": 0
                }
              ],
              "droppedEventsCount": 0,
              "droppedLinksCount": 0,
              "status": {
                "message": "",
                "code": 1
              }
            }
          ]
        }
      ]
    }
  ]
}
```
