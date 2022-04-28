use kvm_ioctls;

fn main() {
    let kvm = kvm_ioctls::Kvm::new().unwrap();
    let kver_satisfies = kvm.get_api_version() == 12;
    println!("KVM version: {}", kver_satisfies);
}
