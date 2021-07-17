mod article;
mod color;
mod properties;
mod user;

use actix_web::{middleware, App, HttpServer};

use crate::properties::AppProperties;
use crate::user::configure_user_service;
use tracing::debug;
use tracing_subscriber::{
    filter::{EnvFilter, LevelFilter},
    fmt, FmtSubscriber,
};

use tracing_log::LogTracer;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::layer::SubscriberExt;


#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    configure_tracing()?;

    let app_props = AppProperties::new()?;
    debug!( EffectiveAppProperties=?app_props);

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(middleware::NormalizePath::default())
            .configure(|cfg| configure_user_service(&app_props.user, cfg))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await?;

    Ok(())
}

fn configure_tracing() -> anyhow::Result<()> {
    let filter = EnvFilter::from_default_env()
        // Set the base level when not matched by other directives to WARN.
        .add_directive(LevelFilter::INFO.into())
        // Set the max level for `my_crate::my_mod` to DEBUG, overriding
        // any directives parsed from the env variable.
        .add_directive("rust_test=trace".parse()?);

    LogTracer::init()?;

    let f = fmt::format().with_timer(fmt::time::ChronoLocal::default());

    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()

        .event_format(f)
        .pretty()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_env_filter(filter)
        // completes the builder.
        .finish();


    let tracer = opentelemetry_otlp::new_pipeline().with_endpoint("http://localhost:8200").with_http().install_simple()?;

    let layered = subscriber.with(tracing_opentelemetry::layer().with_tracer(tracer));
    tracing::subscriber::set_global_default(layered)?;
    Ok(())
}
