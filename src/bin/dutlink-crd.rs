use kube::CustomResourceExt;

fn main() {
    print!(
        "{}",
        serde_yaml::to_string(&dutlink_cli::crd::DutLink::crd()).unwrap()
    )
}
