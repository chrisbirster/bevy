use xshell::{cmd, pushd};

fn main() {
    // When run locally, results may differ from actual CI runs triggered by
    // .github/workflows/ci.yml
    // - Official CI runs latest stable
    // - Local runs use whatever the default Rust is locally

    // See if any code needs to be formatted
    cmd!("cargo fmt --all -- --check")
        .run()
        .expect("Please run 'cargo fmt --all' to format your code.");

    // See if clippy has any complaints.
    // - Type complexity must be ignored because we use huge templates for queries
    cmd!("cargo clippy --workspace --all-targets --all-features -- -D warnings -A clippy::type_complexity -W clippy::doc_markdown")
        .run()
        .expect("Please fix clippy errors in output above.");

    // Run UI tests (they do not get executed with the workspace tests)
    // - See crates/bevy_ecs_compile_fail_tests/README.md
    {
        let _bevy_ecs_compile_fail_tests = pushd("crates/bevy_ecs_compile_fail_tests")
            .expect("Failed to navigate to the 'bevy_ecs_compile_fail_tests' crate");
        cmd!("cargo test")
            .run()
            .expect("Compiler errors of the ECS compile fail tests seem to be different than expected! Check locally and compare rust versions.");
    }
}
