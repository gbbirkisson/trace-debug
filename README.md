<img align="right" height="100" src="https://raw.githubusercontent.com/open-telemetry/opentelemetry-rust/main/assets/logo-text.png">

<h1>trace-debug</h1>

This program was created to debug trace pipelines.

<!-- vim-markdown-toc GFM -->

* [Installing](#installing)
* [Usage](#usage)
* [Using with kubernetes](#using-with-kubernetes)
* [Supported exporters](#supported-exporters)

<!-- vim-markdown-toc -->

## Installing

> [!NOTE]
> You'll need `cargo` and the rust compiler installed on your machine!

```console
$ cargo install --path .
```

## Usage

```console
$ trace-debug --help
Usage: trace-debug [OPTIONS]

Options:
  -e, --exporter <EXPORTER>          What exporter to ship traces with [default: stdout] [possible values: stdout, jaeger]
      --host <HOST>                  Host to ship traces to [default: 127.0.0.1]
      --port <PORT>                  Port to ship traces to [default: <depends on exporter>]
      --service-name <SERVICE_NAME>  Service name [default: trace-debug]
  -t, --tracer-name <TRACER_NAME>    Tracer name [default: trace-debug]
  -s, --span-name <SPAN_NAME>        Span name [default: debug-span]
  -n, --number <NUMBER>              Number of child spans to ship [default: 0]
  -h, --help                         Print help
  -V, --version                      Print version
```

## Using with kubernetes

> [!NOTE]
> You'll need `docker` and `kubectl` installed and setup on your machine.

You can build and run this `trace-debug` in a pod like so:

```console
$ NS=myns POD=mypod CMD='-n 5 -e jaeger --host $$JAEGER_AGENT_HOST' make exec
kubectl -n myns cp trace-debug mypod:/trace-debug
kubectl -n myns exec mypod -- sh -c '/trace-debug -n 5 -e jaeger --host $JAEGER_AGENT_HOST'
Starting trace-debug
Using Args {
    exporter: Jaeger,
    host: "12.0.0.94",
    port: Some(
        6831,
    ),
    service_name: "trace-debug",
    tracer_name: "trace-debug",
    span_name: "debug-span",
    number: 5,
}
Created span with traceid 9bf235ac7e86b64dae821a50b4947932 and spanid b9ee06c548204aba
Created span with traceid 9bf235ac7e86b64dae821a50b4947932 and spanid 1a32d9c700fe1cac
Created span with traceid 9bf235ac7e86b64dae821a50b4947932 and spanid 88408def9236687f
Created span with traceid 9bf235ac7e86b64dae821a50b4947932 and spanid 269021fdb69cdbf0
Created span with traceid 9bf235ac7e86b64dae821a50b4947932 and spanid 2b55e7cda9e73caa
Created span with traceid 9bf235ac7e86b64dae821a50b4947932 and spanid 6666c2897ea22e88
Exiting
```

## Supported exporters

- [x] stdout
- [x] jaeger
- [ ] otlp
- [ ] zipkin
