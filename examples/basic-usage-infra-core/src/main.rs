use dis_infra_core::runtime::default_runtime;

fn main() {
    let runtime = default_runtime().unwrap();

    runtime.run();
}
