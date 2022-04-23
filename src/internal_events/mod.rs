pub mod prelude;

mod adaptive_concurrency;
mod add_fields;
mod add_tags;
mod aggregate;
mod ansi_stripper;
#[cfg(feature = "sources-apache_metrics")]
mod apache_metrics;
#[cfg(feature = "api")]
mod api;
#[cfg(any(
    feature = "sinks-aws_cloudwatch_logs",
    feature = "transforms-aws_cloudwatch_logs_subscription_parser",
))]
mod aws_cloudwatch_logs_subscription_parser;
#[cfg(feature = "transforms-aws_ec2_metadata")]
mod aws_ec2_metadata;
#[cfg(feature = "sources-aws_ecs_metrics")]
mod aws_ecs_metrics;
#[cfg(feature = "sources-aws_kinesis_firehose")]
mod aws_kinesis_firehose;
#[cfg(any(feature = "sources-aws_s3", feature = "sources-aws_sqs",))]
mod aws_sqs;
#[cfg(any(feature = "sinks-azure_blob", feature = "sinks-datadog_archives"))]
pub mod azure_blob;
mod batch;
#[cfg(feature = "transforms-coercer")]
mod coercer;
mod common;
#[cfg(feature = "transforms-concat")]
mod concat;
mod conditions;
#[cfg(feature = "sinks-console")]
mod console;
#[cfg(feature = "sinks-datadog_metrics")]
mod datadog_metrics;
#[cfg(feature = "sinks-datadog_traces")]
mod datadog_traces;
#[cfg(any(feature = "codecs"))]
mod decoder;
#[cfg(feature = "transforms-dedupe")]
mod dedupe;
#[cfg(feature = "sources-demo_logs")]
mod demo_logs;
#[cfg(feature = "sources-dnstap")]
mod dnstap;
#[cfg(feature = "sources-docker_logs")]
mod docker_logs;
mod elasticsearch;
mod encoding_transcode;
#[cfg(feature = "sources-eventstoredb_metrics")]
mod eventstoredb_metrics;
#[cfg(feature = "sources-exec")]
mod exec;
#[cfg(feature = "transforms-filter")]
mod filter;
#[cfg(feature = "sources-fluent")]
mod fluent;
#[cfg(feature = "transforms-geoip")]
mod geoip;
mod heartbeat;
mod http;
pub mod http_client;
#[cfg(feature = "sources-internal_logs")]
mod internal_logs;
#[cfg(all(unix, feature = "sources-journald"))]
mod journald;
#[cfg(feature = "transforms-json_parser")]
mod json_parser;
#[cfg(any(feature = "sources-kafka", feature = "sinks-kafka"))]
mod kafka;
#[cfg(feature = "transforms-key_value_parser")]
mod key_value_parser;
#[cfg(feature = "sources-kubernetes_logs")]
mod kubernetes_logs;
#[cfg(feature = "transforms-log_to_metric")]
mod log_to_metric;
mod logplex;
#[cfg(feature = "sinks-loki")]
mod loki;
#[cfg(feature = "transforms-lua")]
mod lua;
#[cfg(feature = "transforms-metric_to_log")]
mod metric_to_log;
#[cfg(feature = "sources-mongodb_metrics")]
mod mongodb_metrics;
#[cfg(any(feature = "sources-nats", feature = "sinks-nats"))]
mod nats;
#[cfg(feature = "sources-nginx_metrics")]
mod nginx_metrics;
mod open;
#[cfg(any(
    feature = "sinks-datadog_events",
    feature = "transforms-geoip",
    feature = "transforms-log_to_metric",
    feature = "transforms-grok_parser",
    feature = "transforms-json_parser",
    feature = "transforms-key_value_parser",
    feature = "transforms-logfmt_parser",
    feature = "transforms-regex_parser",
    feature = "transforms-split",
    feature = "transforms-tokenizer",
))]
mod parser;
#[cfg(feature = "sources-postgresql_metrics")]
mod postgresql_metrics;
mod process;
#[cfg(any(feature = "sources-prometheus", feature = "sinks-prometheus"))]
mod prometheus;
mod pulsar;
#[cfg(any(feature = "sources-redis", feature = "sinks-redis"))]
mod redis;
#[cfg(feature = "transforms-reduce")]
mod reduce;
mod remap;
#[cfg(feature = "transforms-remove_fields")]
mod remove_fields;
#[cfg(feature = "transforms-rename_fields")]
mod rename_fields;
mod sample;
#[cfg(feature = "sinks-sematext")]
mod sematext_metrics;
mod socket;
#[cfg(any(feature = "sources-splunk_hec", feature = "sinks-splunk_hec"))]
mod splunk_hec;
#[cfg(feature = "sinks-statsd")]
mod statsd_sink;
#[cfg(feature = "sources-statsd")]
mod statsd_source;
mod stdin;
#[cfg(feature = "sources-syslog")]
mod syslog;
#[cfg(feature = "transforms-tag_cardinality_limit")]
mod tag_cardinality_limit;
mod tcp;
mod template;
#[cfg(feature = "transforms-throttle")]
mod throttle;
mod udp;
mod unix;
mod vector;

#[cfg(any(
    feature = "sources-file",
    feature = "sources-kubernetes_logs",
    feature = "sinks-file",
))]
mod file;
mod windows;

pub mod kubernetes;

#[cfg(feature = "sources-mongodb_metrics")]
pub use mongodb_metrics::*;

#[cfg(feature = "transforms-add_fields")]
pub use self::add_fields::*;
#[cfg(feature = "transforms-add_tags")]
pub use self::add_tags::*;
#[cfg(feature = "transforms-aggregate")]
pub use self::aggregate::*;
#[cfg(feature = "transforms-ansi_stripper")]
pub use self::ansi_stripper::*;
#[cfg(feature = "sources-apache_metrics")]
pub use self::apache_metrics::*;
#[cfg(feature = "api")]
pub use self::api::*;
#[cfg(any(
    feature = "sinks-aws_cloudwatch_logs",
    feature = "transforms-aws_cloudwatch_logs_subscription_parser",
))]
pub use self::aws_cloudwatch_logs_subscription_parser::*;
#[cfg(feature = "transforms-aws_ec2_metadata")]
pub use self::aws_ec2_metadata::*;
#[cfg(feature = "sources-aws_ecs_metrics")]
pub use self::aws_ecs_metrics::*;
#[cfg(feature = "sources-aws_kinesis_firehose")]
pub use self::aws_kinesis_firehose::*;
#[cfg(any(feature = "sources-aws_s3", feature = "sources-aws_sqs",))]
pub use self::aws_sqs::*;
#[cfg(feature = "transforms-coercer")]
pub use self::coercer::*;
#[cfg(feature = "transforms-concat")]
pub use self::concat::*;
#[cfg(feature = "sinks-datadog_metrics")]
pub use self::datadog_metrics::*;
#[cfg(feature = "sinks-datadog_traces")]
pub use self::datadog_traces::*;
#[cfg(any(feature = "codecs"))]
pub use self::decoder::*;
#[cfg(feature = "transforms-dedupe")]
pub use self::dedupe::*;
#[cfg(feature = "sources-demo_logs")]
pub use self::demo_logs::*;
#[cfg(feature = "sources-dnstap")]
pub use self::dnstap::*;
#[cfg(feature = "sources-docker_logs")]
pub use self::docker_logs::*;
#[cfg(feature = "sinks-elasticsearch")]
pub use self::elasticsearch::*;
#[cfg(feature = "sources-eventstoredb_metrics")]
pub use self::eventstoredb_metrics::*;
#[cfg(feature = "sources-exec")]
pub use self::exec::*;
#[cfg(any(
    feature = "sources-file",
    feature = "sources-kubernetes_logs",
    feature = "sinks-file",
))]
pub use self::file::*;
#[cfg(feature = "transforms-filter")]
pub use self::filter::*;
#[cfg(feature = "sources-fluent")]
pub use self::fluent::*;
#[cfg(feature = "transforms-geoip")]
pub use self::geoip::*;
#[cfg(any(
    feature = "sources-utils-http",
    feature = "sources-utils-http-encoding",
    feature = "sources-datadog_agent",
    feature = "sources-splunk_hec",
    feature = "sources-aws_ecs_metrics",
))]
pub use self::http::*;
#[cfg(feature = "sources-internal_logs")]
pub use self::internal_logs::*;
#[cfg(all(unix, feature = "sources-journald"))]
pub use self::journald::*;
#[cfg(feature = "transforms-json_parser")]
pub use self::json_parser::*;
#[cfg(any(feature = "sources-kafka", feature = "sinks-kafka"))]
pub use self::kafka::*;
#[cfg(feature = "transforms-key_value_parser")]
pub use self::key_value_parser::*;
#[cfg(feature = "sources-kubernetes_logs")]
pub use self::kubernetes_logs::*;
#[cfg(feature = "transforms-log_to_metric")]
pub use self::log_to_metric::*;
#[cfg(feature = "sources-heroku_logs")]
pub use self::logplex::*;
#[cfg(feature = "sinks-loki")]
pub use self::loki::*;
#[cfg(feature = "transforms-lua")]
pub use self::lua::*;
#[cfg(feature = "transforms-metric_to_log")]
pub use self::metric_to_log::*;
#[cfg(any(feature = "sources-nats", feature = "sinks-nats"))]
pub use self::nats::*;
#[cfg(feature = "sources-nginx_metrics")]
pub use self::nginx_metrics::*;
#[cfg(any(
    feature = "sinks-datadog_events",
    feature = "transforms-geoip",
    feature = "transforms-log_to_metric",
    feature = "transforms-grok_parser",
    feature = "transforms-json_parser",
    feature = "transforms-key_value_parser",
    feature = "transforms-logfmt_parser",
    feature = "transforms-regex_parser",
    feature = "transforms-split",
    feature = "transforms-tokenizer",
))]
pub use self::parser::*;
#[cfg(feature = "sources-postgresql_metrics")]
pub use self::postgresql_metrics::*;
#[cfg(any(feature = "sources-prometheus", feature = "sinks-prometheus"))]
pub use self::prometheus::*;
#[cfg(feature = "sinks-pulsar")]
pub use self::pulsar::*;
#[cfg(any(feature = "sources-redis", feature = "sinks-redis"))]
pub use self::redis::*;
#[cfg(feature = "transforms-reduce")]
pub use self::reduce::*;
#[cfg(feature = "transforms-remap")]
pub use self::remap::*;
#[cfg(feature = "transforms-remove_fields")]
pub use self::remove_fields::*;
#[cfg(feature = "transforms-rename_fields")]
pub use self::rename_fields::*;
#[cfg(feature = "transforms-sample")]
pub use self::sample::*;
#[cfg(feature = "sinks-sematext")]
pub use self::sematext_metrics::*;
#[cfg(any(feature = "sources-splunk_hec", feature = "sinks-splunk_hec"))]
pub use self::splunk_hec::*;
#[cfg(feature = "sinks-statsd")]
pub use self::statsd_sink::*;
#[cfg(feature = "sources-statsd")]
pub use self::statsd_source::*;
#[cfg(feature = "sources-stdin")]
pub use self::stdin::*;
#[cfg(feature = "sources-syslog")]
pub use self::syslog::*;
#[cfg(feature = "transforms-tag_cardinality_limit")]
pub use self::tag_cardinality_limit::*;
#[cfg(feature = "transforms-throttle")]
pub use self::throttle::*;
#[cfg(all(
    any(
        feature = "sinks-socket",
        feature = "sinks-statsd",
        feature = "sources-dnstap",
        feature = "sources-metrics",
        feature = "sources-statsd",
        feature = "sources-syslog",
        feature = "sources-socket"
    ),
    unix
))]
pub use self::unix::*;
#[cfg(feature = "sources-vector")]
pub use self::vector::*;
#[cfg(windows)]
pub use self::windows::*;
pub use self::{
    adaptive_concurrency::*, batch::*, common::*, conditions::*, encoding_transcode::*,
    heartbeat::*, open::*, process::*, socket::*, tcp::*, template::*, udp::*,
};

// this version won't be needed once all `InternalEvent`s implement `name()`
#[cfg(test)]
#[macro_export]
macro_rules! emit {
    ($event:expr) => {
        vector_core::internal_event::emit(vector_core::internal_event::DefaultName {
            event: $event,
            name: stringify!($event),
        })
    };
}

#[cfg(not(test))]
#[macro_export]
macro_rules! emit {
    ($event:expr) => {
        vector_core::internal_event::emit($event)
    };
}
