use midix::{events::FileEvent, reader::Reader};

fn loop_through(bytes: &[u8]) {
    let mut reader = Reader::from_byte_slice(bytes);

    loop {
        match reader.read_event() {
            Ok(e) => {
                println!("FileEvent: {:?}", e);
                if e == FileEvent::EOF {
                    break;
                }
            }
            Err(e) => {
                panic!("Error at {}, {:?}", reader.buffer_position(), e);
            }
        }
    }
}

#[test]
fn read_clementi() {
    loop_through(include_bytes!("../test-asset/Clementi.mid"))
}

#[test]
fn read_clementi_rewritten() {
    loop_through(include_bytes!("../test-asset/ClementiRewritten.mid"))
}

#[test]
fn read_crab_rave() {
    loop_through(include_bytes!("../test-asset/CrabRave.mid"))
}

#[test]
fn read_levels() {
    loop_through(include_bytes!("../test-asset/Levels.mid"))
}

#[test]
fn read_pi() {
    loop_through(include_bytes!("../test-asset/Pi.mid"))
}

#[test]
fn read_pi_damaged() {
    let bytes = include_bytes!("../test-asset/PiDamaged.mid");
    let mut reader = Reader::from_byte_slice(bytes);

    while let Ok(e) = reader.read_event() {
        if e == FileEvent::EOF {
            panic!("Corrupted file should not have yielded an eof event")
        }
    }
}

#[test]
fn read_river_flows_in_you() {
    loop_through(include_bytes!("../test-asset/RiverFlowsInYou.mid"))
}

#[test]
fn read_sandstorm() {
    loop_through(include_bytes!("../test-asset/Sandstorm.mid"))
}
