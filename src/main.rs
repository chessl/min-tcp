use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut config = tun::Configuration::default();

    config
        .address((10, 0, 0, 9))
        .netmask((255, 255, 255, 0))
        .destination((10, 0, 0, 1))
        .up();

    let mut dev = tun::create(&config)?;
    let mut buf = [0; 4096];

    loop {
        let amount = dev.read(&mut buf)?;

        match etherparse::Ipv4HeaderSlice::from_slice(&buf) {
            Ok(iph) => {
                let src = iph.source_addr();
                let dst = iph.destination_addr();
                let proto = iph.protocol();

                match etherparse::TcpHeaderSlice::from_slice(&buf[iph.slice().len()..]) {
                    // If TCP header parsing was successful, proceed.
                    Ok(tcph) => {
                        // Print the details: Source IP, Destination IP, and the Destination Port.
                        eprintln!(
                            "{} -> {}: TCP to port {}",
                            src,
                            dst,
                            tcph.destination_port()
                        );
                    }
                    // Handle potential errors while parsing the TCP header.
                    Err(e) => {
                        eprintln!("An error occurred while parsing TCP packet: {e:?}");
                    }
                }
            }
            Err(e) => {
                eprintln!("An error occurred while parsing IP packet: {e:?}");
            }
        }
        println!("{:?}", &buf[0..amount]);
    }
}
