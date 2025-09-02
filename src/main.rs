use std::{env, error::Error, io::{self, Read, Write}, process::Command, time::Duration};

use buttplug::{
    client::{ButtplugClient, ButtplugClientError, ScalarValueCommand},
    core::connector::new_json_ws_client_connector,
};
use tokio::time::sleep;

async fn vibrate(client: &ButtplugClient, strength: f64) -> Result<(), ButtplugClientError> {
    let devices = client.devices();

    if devices.len() > 0 {
        for device in &devices {
            device.vibrate(&ScalarValueCommand::ScalarValue(strength)).await?;
        }
    } else {
        eprintln!("vibe: no devices found");
    }

    Ok(())
}

fn print_usage() {
    println!("usage: vibe [command [arg ...]]");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        print_usage();
        return Err("incorrect usage".into());
    } else if &args[1] == "-h" {
        print_usage();
        return Ok(());
    }

    let address = env::var("VIBE_ADDRESS").unwrap_or("ws://localhost:12345".to_string());
    let connector = new_json_ws_client_connector(&address);
    let client = ButtplugClient::new("Vibe Client");

    if let Err(e) = client.connect(connector).await {
        eprintln!("Could not connect: {e}");

        print!("Continue anyway? [Y/n] ");
        let _ = io::stdout().flush();
        let mut yn = [0];
        let _ = io::stdin().read(&mut yn);
        if yn[0] == b'n' || yn[0] == b'N' {
            return Err(e.into());
        }
    }

    // vibration strength used as a baseline
    let strength = match env::var("VIBE_STRENGTH").ok() {
        Some(s) => s.parse::<f64>().unwrap_or(0.25),
        None => 0.25,
    };

    // run the command that was supplied to us
    let mut child = match Command::new(&args[1]).args(&args[2..]).spawn() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error running command: {e}");
            return Err(e.into());
        }
    };

    vibrate(&client, strength).await?;

    if child.wait()?.success() {
        // reward success by smoothly going out
        vibrate(&client, 0.25 * strength).await?;
    } else {
        // go harder for a brief burst
        vibrate(&client, 4.0 * strength).await?;
    }

    sleep(Duration::from_millis(250)).await;

    client.stop_all_devices().await?;

    Ok(())
}
