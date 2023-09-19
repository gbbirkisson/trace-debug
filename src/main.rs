use opentelemetry::sdk::trace::TracerProvider;
use opentelemetry::{
    global,
    sdk::{trace::Config, Resource},
    trace::{get_active_span, Tracer},
    KeyValue,
};

use clap::Parser;
use opentelemetry_otlp::WithExportConfig;

#[derive(clap::ValueEnum, Clone, Debug)]
enum Exporter {
    Stdout,
    Jaeger,
    Otlp,
}

#[derive(Parser, Debug)]
#[command(author, version, about = None, long_about = None)]
struct Args {
    /// Exporter type
    #[clap(short, long)]
    #[clap(value_enum, default_value_t=Exporter::Stdout)]
    exporter: Exporter,

    /// Scheme to use
    #[clap(long)]
    #[clap(default_value = "http")]
    scheme: String,

    /// Host to export to
    #[clap(long)]
    #[clap(default_value = "127.0.0.1")]
    host: String,

    /// Port to export to [default: <depends on exporter>]
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

    /// Number of generated child spans
    #[clap(short, long)]
    #[clap(default_value = "0")]
    number: usize,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut args = Args::parse();

    println!("Starting trace-debug");

    // Set default ports based on exporter
    args.port = match (args.port, &args.exporter) {
        (None, Exporter::Jaeger) => Some(6831),
        (None, Exporter::Otlp) => Some(4317),
        other => other.0,
    };

    println!("Using {:#?}", args);
    let config = Config::default().with_resource(Resource::new(vec![KeyValue {
        key: "service.name".into(),
        value: args.service_name.clone().into(),
    }]));

    match args.exporter {
        Exporter::Stdout => {
            let provider = TracerProvider::builder()
                .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
                .with_config(config)
                .build();
            global::set_tracer_provider(provider);
        }
        Exporter::Jaeger => {
            let provider = opentelemetry_jaeger::new_agent_pipeline()
                .with_service_name(args.service_name)
                .with_endpoint((
                    args.host,
                    args.port.expect("default port not set for exporter"),
                ))
                .build_simple()
                .expect("failed to create jager exporter");
            global::set_tracer_provider(provider);
        }
        Exporter::Otlp => {
            opentelemetry_otlp::new_pipeline()
                .tracing()
                .with_trace_config(config)
                .with_exporter(
                    opentelemetry_otlp::new_exporter()
                        .tonic()
                        .with_endpoint(format!(
                            "{}://{}:{}",
                            args.scheme,
                            args.host,
                            args.port.expect("default port not set for exporter")
                        )),
                )
                .install_simple()
                .expect("failed to create otlp tracer");
        }
    };

    global::tracer(args.tracer_name.clone()).in_span(args.span_name.clone(), |_| {
        print_span();
        for _ in 0..args.number {
            global::tracer(args.tracer_name.clone()).in_span(args.span_name.clone(), |_| {
                print_span();
            });
        }
    });

    // Shutdown trace pipeline
    let _ = tokio::task::spawn_blocking(|| {
        global::shutdown_tracer_provider();
    })
    .await;

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
