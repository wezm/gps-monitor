use std::io::prelude::*;
use std::time::Duration;
use std::{env, io};

use nmea_parser::{NmeaStore, ParseError, ParsedSentence};
use serialport::SerialPortSettings;
use std::io::BufReader;

fn main() -> Result<(), io::Error> {
    let device = env::args_os().skip(1).next().expect("Usage: /dev/ttyUSB0");
    let port_name = device.to_string_lossy();

    let settings = SerialPortSettings {
        baud_rate: 4800,
        timeout: Duration::from_millis(1000),
        ..Default::default()
    };
    let port = serialport::open_with_settings(&device, &settings);
    let mut store = NmeaStore::new();

    match port {
        Ok(port) => {
            let reader = BufReader::new(port);
            for line in reader.lines() {
                let line = match line {
                    Ok(line) => line,
                    Err(_err) => continue,
                };

                match nmea_parser::parse_sentence(&line, &mut store) {
                    Ok(ParsedSentence::VesselDynamicData(vdd)) => {
                        println!("{:#?}", vdd);
                    }
                    Ok(ParsedSentence::VesselStaticData(vds)) => {
                        println!("{:#?}", vds);
                    }
                    Ok(ParsedSentence::Gga(gga)) => {
                        println!("{:#?}", gga);
                    }
                    Ok(ParsedSentence::Rmc(rmc)) => {
                        println!("{:#?}", rmc);
                    }
                    Ok(ParsedSentence::Gsa(gsa)) => {
                        println!("{:#?}", gsa);
                    }
                    Ok(ParsedSentence::Gsv(gsv)) => {
                        println!("{:#?}", gsv);
                    }
                    Ok(ParsedSentence::Vtg(vtg)) => {
                        println!("{:#?}", vtg);
                    }
                    Ok(ParsedSentence::Gll(gll)) => {
                        println!("{:#?}", gll);
                    }
                    Err(e) => eprintln!("{:?}", e),
                    Ok(ParsedSentence::Incomplete) => {}
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }

    Ok(())
}
