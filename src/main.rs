use std::error::Error;
use bluez::client::*;
use std::path::Path;
use std::fs::File;
use rodio::Sink;
use std::io::BufReader;

mod bluetooth_comms;


#[async_std::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = BlueZClient::new().unwrap();

    let version = client.get_mgmt_version().await?;
    println!(
        "management version: {}.{}",
        version.version, version.revision
    );

    let controllers = client.get_ext_controller_list().await?;

    println!("\navailable controllers:");

    for (controller, controller_type, controller_bus) in controllers {
        println!(
            "\t{:?} ({:?}, {:?})",
            controller, controller_type, controller_bus
        );
        let info = client.get_controller_info(controller).await?;

        println!("\t\tname: {:?}", info.name);
        println!("\t\tshort name: {:?}", info.short_name);
        println!("\t\taddress: {}", info.address);
        println!("\t\tsupported settings: {:?}", info.supported_settings);
        println!("\t\tcurrent settings: {:?}", info.current_settings);
        println!("\t\tmanufacturer: 0x{:04x}", info.manufacturer);
        println!("\t\tbluetooth version: 0x{:02x}", info.bluetooth_version);
        println!("\t\tclass of device: {:?}", info.class_of_device);
    }

    Ok(())
}

fn test_rodio() { 
    let path = Path::new("public/wii_shop.mp3");
    let file = File::open(path).unwrap();
    let device = rodio::default_output_device().unwrap();

    let sink = Sink::new(&device);
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

    sink.append(source);

    sink.play();

    sink.sleep_until_end();
}
