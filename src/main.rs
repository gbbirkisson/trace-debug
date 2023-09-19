use opentelemetry::{
    global,
    sdk::{
        trace::{Config, TracerProvider},
        Resource,
    },
    trace::{get_active_span, Tracer},
    KeyValue,
};

use clap::Parser;

#[derive(clap::ValueEnum, Clone, Debug)]
enum Exporter {
    Stdout,
    Jaeger,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// What exporter to ship traces with
    #[clap(short, long)]
    #[clap(value_enum, default_value_t=Exporter::Stdout)]
    exporter: Exporter,

    /// Host to ship traces to
    #[clap(long)]
    #[clap(default_value = "127.0.0.1")]
    host: String,

    /// Port to ship traces to [default: <depends on protocol>]
    #[clap(long)]
    port: Option<u16>,

    /// Service name
    #[clap(long)]
    #[clap(default_value = "trace-debug")]
    service_name: String,

    /// Tracer name
    #[clap(short, long)]
    #[clap(default_value = "trace-debug")]
    tracer_name: String,

    /// Span name
    #[clap(short, long)]
    #[clap(default_value = "debug-span")]
    span_name: String,

    /// Number of child spans to ship
    #[clap(short, long)]
    #[clap(default_value = "0")]
    number: usize,
}

fn main() {
    let mut args = Args::parse();

    println!("Starting trace-debug");

    // Set default ports based on exporter
    args.port = match (args.port, &args.exporter) {
        (None, Exporter::Jaeger) => Some(6831),
        other => other.0,
    };

    println!("Using {:#?}", args);

    let provider = match args.exporter {
        Exporter::Stdout => TracerProvider::builder()
            .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
            .with_config(
                Config::default().with_resource(Resource::new(vec![KeyValue {
                    key: "service.name".into(),
                    value: args.service_name.into(),
                }])),
            )
            .build(),
        Exporter::Jaeger => opentelemetry_jaeger::new_agent_pipeline()
            .with_service_name(args.service_name)
            .with_endpoint((
                args.host,
                args.port.expect("default port not set for exporter"),
            ))
            .build_simple()
            .expect("failed to create jager exporter"),
    };

    global::set_tracer_provider(provider);

    global::tracer(args.tracer_name.clone()).in_span(args.span_name.clone(), |_| {
        print_span();
        for _ in 0..args.number {
            global::tracer(args.tracer_name.clone()).in_span(args.span_name.clone(), |_| {
                print_span();
            });
        }
    });

    // Shutdown trace pipeline
    global::shutdown_tracer_provider();

    println!("Exiting");
}

fn print_span() {
    get_active_span(|span| {
        println!(
            "Created span with traceid {} and spanid {}",
            span.span_context().trace_id(),
            span.span_context().span_id()
        );
    });
}
