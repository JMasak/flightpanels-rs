use bitfield_struct::bitfield;
use hidapi::HidApi;
use std::sync::mpsc::Sender;
use std::result::Result;
use std::thread;

/*
outputs: 13bytes
byte 0: 0    // HID report ID
bytes 1-5: 5 Ziffern oberes Display (kein ASCII - Werte 0-9)
bytes 6-10: 5 Ziffern unteres Display
byte 11: leds
  bit0: ap
  bit1: hdg
  bit2: nav
  bit3: ias
  bit4: alt
  bit5: vs
  bit6: apr
  bit7: rev
byte12: padding?
*/


const ID: (u16, u16) = (0x06A3, 0x0D06);

pub struct MultiPanel {
}

impl MultiPanel {
    pub fn receive(api: &HidApi, tx: Sender<crate::InputData>) -> Result<&'static str, &'static str> {
        if let Ok(device) = api.open(ID.0, ID.1) {
            thread::spawn(move || {
                let mut input_buffer = [0u8; 4];
                loop {
                    match device.read(&mut input_buffer) {
                        Ok(_) => {
                            tx.send(crate::InputData::MultiInputData(
                                MultiPanelInputs::from(u32::from_le_bytes(input_buffer[0..4].try_into().expect("incorrect input length")))
                            )).expect("could not send");
                        },
                        Err(_e) => ()
                    }
                }
            });
            return Ok("super")
        }
        else {
            return Err("Could not open device")
        }
    }
}

#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MultiPanelInputs {
    #[bits(5)]
    selector: SettingSelection,
    jog_inc: bool,
    jog_dec: bool,
    ap: bool,
    hdg: bool,
    nav: bool,
    ias: bool,
    alt: bool,
    vs: bool,
    apr: bool,
    rev: bool,
    auto_throttle: bool,
    flaps_up: bool,
    flaps_down: bool,
    pitch_down: bool,
    pitch_up: bool,
    #[bits(12)]
    _pad: u32
}

pub struct MultiPanelOutputs {
    pub upper_display: [u8; 5],
    pub lower_display: [u8; 5],
    pub leds: MultiPanelOutputLeds,
}

impl MultiPanelOutputs {
    pub fn as_bytes(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::with_capacity(13);
        data.push(0);   // hid report no.
        data.extend_from_slice(&self.upper_display[0..]);
        data.extend_from_slice(&self.lower_display[0..]);
        data.push(self.leds.into());
        data.push(0); // padding?
        data
    }
}

#[bitfield(u8)]
#[derive(PartialEq, Eq)]
pub struct MultiPanelOutputLeds {
    pub ap: bool,
    pub hdg: bool,
    pub nav: bool,
    pub ias: bool,
    pub alt: bool,
    pub vs: bool,
    pub apr: bool,
    pub rev: bool
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SettingSelection {
    Invalid = 0,
    ALT = 1,
    VS = 2,
    IAS = 4,
    HDG = 8,
    CRS = 16
}

impl Into<u32> for SettingSelection {
    fn into(self) -> u32 {
        self as u8 as u32
    }
}

impl From<u32> for SettingSelection {
    fn from(value: u32) -> Self {
        match value {
            0 => SettingSelection::Invalid,
            1 => SettingSelection::ALT,
            2..=3 => SettingSelection::VS,
            4..=7 => SettingSelection::IAS,
            8..=15 => SettingSelection::HDG,
            16..=u32::MAX => SettingSelection::CRS
        }
    }
}