use once_cell::sync::Lazy;

pub(crate) static TELEMETRY_RESULT: Lazy<hkt_o11y::metrics::IntCounterVec> = Lazy::new(|| {
    hkt_o11y::metrics::try_create_int_counter_vec(
        "hkt_telemetry_result",
        "Count of 'ok' or 'failed' results of uploading telemetry data",
        &["success"],
    )
    .unwrap()
});
