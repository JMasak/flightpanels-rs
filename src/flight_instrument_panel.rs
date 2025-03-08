use bitfield_struct::bitfield;
use hidapi::HidApi;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;

const ID: (u16, u16) = (0x06A3, 0xA2AE);

pub struct FlightInstrumentPanel {
}

impl FlightInstrumentPanel {
    pub fn receive(api: &HidApi, tx: Sender<crate::InputData>, rx: Receiver<OutputCommands>) -> Result<&'static str, &'static str> {
        if let Ok(device) = api.open(ID.0, ID.1) {
            thread::spawn(move || {
                let mut input_buffer = [0u8; 2];
                loop {
                    match device.read_timeout(&mut input_buffer, 250) {
                        Ok(_) => {
                            tx.send(crate::InputData::FIPInputData(
                                FlightInstrumentPanelInputs::from(u16::from_le_bytes(input_buffer[0..2].try_into().expect("incorrect input length")))
                            )).expect("could not send");
                        },
                        Err(_e) => ()
                    }
                }
            });
            return Ok("super")
        }
        else {
            return Err("Could not open FIP device")
        }
    }
}


#[bitfield(u16)]
#[derive(PartialEq, Eq)]
pub struct FlightInstrumentPanelInputs {
    pub s1: bool,
    pub s2: bool,
    pub s3: bool,
    pub s4: bool,
    pub s5: bool,
    pub s6: bool,
    pub left_encoder_dec: bool,
    pub left_encoder_inc: bool,
    pub up: bool,
    pub down: bool,
    pub right_encoder_dec: bool,
    pub right_encoder_inc: bool,
    #[bits(4)]
    _pad: u16
}

pub enum OutputCommands {

}