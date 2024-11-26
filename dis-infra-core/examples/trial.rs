use dis_infra_core::runtime::default_runtime;

fn main() {
    let mut runtime = match default_runtime() {
        Ok(rt) => { rt }
        Err(_) => { assert!(false); return }
    };

    runtime.run();
}