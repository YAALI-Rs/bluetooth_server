use std::error::Error;
use bluez::client::*;
use std::path::Path;
use std::fs::File;
use rodio::Sink;
use std::io::BufReader;

mod server;

#[async_std::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = BlueZClient::new().unwrap();

    let version = client.get_mgmt_version().await?;
    println!(
        "management version: {}.{}",
        version.version, version.revision
    );

    let mut controllers = client.get_ext_controller_list().await?;

    let main_controller = controllers.remove(0);

    println!("\navailable controllers:");

    let features = client.get_advertising_features(main_controller.0).await?;

    println!("supported flags: {:?}", features.supported_flags);
    println!("{}", main_controller.0);

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
