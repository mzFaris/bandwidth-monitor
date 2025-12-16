use pnet::datalink::linux::interfaces;

fn main() {
    let interfaces = interfaces();
    let default_interface = interfaces
        .iter()
        .find(|&e| e.is_up() && !e.is_loopback() && !e.ips.is_empty())
        .expect("no usable interface found");
    println!("{:#?}", default_interface);
}
