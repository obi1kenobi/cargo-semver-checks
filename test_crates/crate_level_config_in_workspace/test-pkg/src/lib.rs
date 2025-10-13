pub fn get_max_steal_operations(metrics: &tokio_metrics::RuntimeMetrics) -> u64 {
    metrics.max_steal_operations
}

// Rustdoc doesn't compile or check method bodies, so we manually add a compile error
// when `--cfg tokio_unstable` isn't set.
#[cfg(not(tokio_unstable))]
core::compile_error!("requires `--cfg tokio_unstable`");
