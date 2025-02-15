---
title: Jaeger
---

[Jaeger](https://github.com/jaegertracing/jaeger) is an open-source, end-to-end distributed tracing tool that originated from [Uber](https://www.uber.com/). It helps monitor and troubleshoot microservices-based applications.

Databend has the ability to export tracing data to Jaeger by integrating with the [OpenTelemetry](https://opentelemetry.io/) SDK. The following tutorial shows you how to deploy and use Jaeger to trace Databend.

## Tutorial: Trace Databend with Jaeger

### Step 1. Deploy Jaeger

This tutorial uses the All In One image to deploy Jaeger in Docker. If you already have a running Jaeger instance, you can skip this step.

```bash
docker run -d -p6831:6831/udp -p14268:14268 -p16686:16686 --name jaeger jaegertracing/all-in-one:latest
```

### Step 2. Set Environment Variables

Set the following environment variables according to your actual tracing level requirements and Jaeger endpoint.
    - `DATABEND_TRACING_CAPTURE_LOG_LEVEL`: Sets the log level that will attach to spans.
    - `DATABEND_JAEGER_ENDPOINT`: Sets the endpoint the Jaeger agent is listening on.

```bash
export DATABEND_TRACING_CAPTURE_LOG_LEVEL=DEBUG
export DATABEND_JAEGER_ENDPOINT=http://localhost:14268/api/traces
```

### Step 3. Deploy Databend

1. Follow the [Deployment Guide](https://databend.rs/doc/deploy) to deploy Databend.

2. Run the following SQL statements:

```sql
CREATE TABLE t1(a INT);
INSERT INTO t1 VALUES(1);
INSERT INTO t1 SELECT * FROM t1;
```

### Step 4. Check Tracing Information on Jaegar

1. Go to <http://127.0.0.1:16686/> and select the **Search** tab.

2. Select a service in the **Service** drop-down list. For example, select the databend-query service.

3. Click **Find Traces** to show the traces.

![](https://datafuse-1253727613.cos.ap-hongkong.myqcloud.com/jaeger-tracing-show.png)
