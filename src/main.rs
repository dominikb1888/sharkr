use pcap::{Device, Capture};

fn main() {
    let devices = Device::list().unwrap();
    for (id, device) in devices.clone().into_iter().enumerate() {
        println!("{id} | {} | {:?}", device.name, device.flags.connection_status);
    }

    let mut cap = Capture::from_device(devices[1].clone()).unwrap()
        .open().unwrap();

    while let Ok(packet) = cap.next_packet() {
        println!("received packet! {:?}", packet);
    }
}


