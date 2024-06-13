use kube::CustomResourceExt;

fn main() {
    print!(
        "{}",
        serde_json::to_string(&dutlink_cli::crd::DutLink::crd()).unwrap()
    )
}
