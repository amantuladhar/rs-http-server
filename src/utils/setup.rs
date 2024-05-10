use tracing::{subscriber::set_global_default, Level};

pub fn setup() {
    setup_log();
}
pub fn setup_log() {
    #[cfg(debug_assertions)]
    {
        color_eyre::install().expect("should be able to setup color_eyre");
    }
    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        // .without_time()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(false)
        // Build the subscriber
        .with_max_level(Level::TRACE)
        .finish();
    set_global_default(subscriber).expect("unable to set logger global default");
}
