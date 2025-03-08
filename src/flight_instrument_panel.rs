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






/*
Wireshark recording of setting LEDS:

USB URB
[Source: host]
[Destination: 2.5.2]
USBPcap pseudoheader length: 27
IRP ID: 0xffffa08a070f1250
IRP USBD_STATUS: USBD_STATUS_SUCCESS (0x00000000)
URB Function: URB_FUNCTION_BULK_OR_INTERRUPT_TRANSFER (0x0009)
IRP information: 0x00, Direction: FDO -> PDO
URB bus id: 2
Device address: 5
Endpoint: 0x02, Direction: OUT
URB transfer type: URB_BULK (0x03)
Packet Data Length: 44
[bInterfaceClass: Vendor Specific (0xff)]

Leftover Capture Data: 0000000000000001000000000000000000000000000000180000000300000000000000010000000000000000   - set led 0 -> 1
Leftover Capture Data: 0000000000000001000000000000000000000000000000180000000300000001000000010000000000000000   - set led 1 -> 1 (S1)
Leftover Capture Data: 0000000000000001000000000000000000000000000000180000000300000002000000010000000000000000   - set led 2 -> 1 (S2)
Leftover Capture Data: 0000000000000001000000000000000000000000000000180000000300000003000000010000000000000000
Leftover Capture Data: 0000000000000001000000000000000000000000000000180000000300000004000000010000000000000000
Leftover Capture Data: 0000000000000001000000000000000000000000000000180000000300000005000000010000000000000000
Leftover Capture Data: 0000000000000001000000000000000000000000000000180000000300000006000000010000000000000000   - set led 6 -> 1 (S6)
Leftover Capture Data: 0000000000000001000000000000000000000000000000180000000300000007000000010000000000000000   - set led 7 -> 1 (UP_ARROW)
Leftover Capture Data: 0000000000000001000000000000000000000000000000180000000400000008000000010000000000000000   - set led 8 -> 1 (DOWN_ARROW)
Leftover Capture Data: 0000000000000001000000000000000000000000000000180000000300000000000000000000000000000000   - set led 0 -> 0
Leftover Capture Data: 0000000000000001000000000000000000000000000000180000000300000001000000000000000000000000   - set led 1 -> 0 (S1)
Leftover Capture Data: 0000000000000001000000000000000000000000000000180000000300000002000000000000000000000000
Leftover Capture Data: 0000000000000001000000000000000000000000000000180000000300000003000000000000000000000000
Leftover Capture Data: 0000000000000001000000000000000000000000000000180000000300000004000000000000000000000000
Leftover Capture Data: 0000000000000001000000000000000000000000000000180000000300000005000000000000000000000000
Leftover Capture Data: 0000000000000001000000000000000000000000000000180000000300000006000000000000000000000000   - set led 6 -> 0 (S6)
Leftover Capture Data: 0000000000000001000000000000000000000000000000180000000300000007000000000000000000000000   - set led 7 -> 0 (UP_ARROW)
Leftover Capture Data: 0000000000000001000000000000000000000000000000180000000400000008000000000000000000000000   - set led 8 -> 0 (DOWN_ARROW)

*/