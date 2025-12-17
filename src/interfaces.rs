use pcap::Device;

pub fn list_interfaces() -> Result<(), pcap::Error> {
    let devices = Device::list()?;

    for dev in devices {
        println!(
            "{}{}",
            dev.name,
            dev.desc
                .as_ref()
                .map(|d| format!(" – {}", d))
                .unwrap_or_default()
        );
    }

    Ok(())
}

