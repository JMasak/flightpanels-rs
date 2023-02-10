use bitfield_struct::bitfield;
use bitflags::bitflags;
use hidapi::HidApi;
use std::sync::mpsc::{Sender, Receiver};
use std::result::Result;
use std::thread;
use std::time::Duration;

const ID: (u16, u16) = (0x06A3, 0x0D67);

pub struct SwitchPanel {
}

impl SwitchPanel {
    pub fn receive(api: &HidApi, tx: Sender<crate::InputData>, rx: Receiver<OutputCommands>) -> Result<&'static str, &'static str> {
        if let Ok(device) = api.open(ID.0, ID.1) {
            thread::spawn(move || {
                let mut input_buffer = [0u8; 4];
                let mut current_leds: u8 = 0;
                loop {
                    match device.read_timeout(&mut input_buffer, 250) {
                        Ok(_) => {
                            tx.send(crate::InputData::SwitchInputData(
                                SwitchPanelInputs::from(u32::from_le_bytes(input_buffer[0..4].try_into().expect("incorrect input length")))
                            )).expect("could not send");
                        },
                        Err(_e) => ()
                    }
                    if let Ok(command) = rx.recv_timeout(Duration::from_millis(10)) {
                        match command {
                            OutputCommands::SetLeds(value) => {
                                current_leds = value;
                            }
                            OutputCommands::SetAllLedsTo(color) => {
                                current_leds = match color {
                                    LedColors::Off => 0,
                                    LedColors::Green => (GearLedsStates::UP_GREEN | GearLedsStates::LEFT_GREEN | GearLedsStates::RIGHT_GREEN).bits,
                                    LedColors::Yellow => (GearLedsStates::UP_YELLOW | GearLedsStates::LEFT_YELLOW | GearLedsStates::RIGHT_YELLOW).bits,
                                    LedColors::Red => (GearLedsStates::UP_RED | GearLedsStates::LEFT_RED | GearLedsStates::RIGHT_RED).bits,
                                }
                            },
                            OutputCommands::SetUpLedTo(color) => {
                                current_leds &= GearLedsStates::UP_MASK.bits;
                                current_leds |= match color {
                                    LedColors::Off => 0,
                                    LedColors::Green => GearLedsStates::UP_GREEN.bits,
                                    LedColors::Yellow => GearLedsStates::UP_YELLOW.bits,
                                    LedColors::Red => GearLedsStates::UP_RED.bits
                                }
                            }
                            OutputCommands::SetLeftLedTo(color) => {
                                current_leds &= GearLedsStates::LEFT_MASK.bits;
                                current_leds |= match color {
                                    LedColors::Off => 0,
                                    LedColors::Green => GearLedsStates::LEFT_GREEN.bits,
                                    LedColors::Yellow => GearLedsStates::LEFT_YELLOW.bits,
                                    LedColors::Red => GearLedsStates::LEFT_RED.bits
                                }
                            },
                            OutputCommands::SetRightLedTo(color) => {
                                current_leds &= GearLedsStates::RIGHT_MASK.bits;
                                current_leds |= match color {
                                    LedColors::Off => 0,
                                    LedColors::Green => GearLedsStates::RIGHT_GREEN.bits,
                                    LedColors::Yellow => GearLedsStates::RIGHT_YELLOW.bits,
                                    LedColors::Red => GearLedsStates::RIGHT_RED.bits
                                }
                            }
                        }
                        device.send_feature_report(&[0, current_leds]);
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
pub struct SwitchPanelInputs {
    pub battery: bool,
    pub alt: bool,
    pub avionics: bool,
    pub fuel_pump: bool,
    pub de_ice: bool,
    pub pitot_heat: bool,
    pub cowl: bool,
    pub panel_lights: bool,
    pub beacon_lights: bool,
    pub navigation_lights: bool,
    pub strobe_lights: bool,
    pub taxi_lights: bool,
    pub landing_lights: bool,
    #[bits(5)]
    pub engine_selector: EngineSelection,
    pub gear_up: bool,
    pub gear_down: bool,
    #[bits(12)]
    _pad: u16
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EngineSelection {
    Invalid = 0,
    OFF = 1,
    RIGHT = 2,
    LEFT = 4,
    BOTH = 8,
    START = 16
}

bitflags! {
    pub struct GearLedsStates: u8 {
        const ALL_OFF = 0b00000000;
        const UP_GREEN = 0b00000001;
        const UP_RED = 0b00001000;
        const UP_YELLOW = Self::UP_GREEN.bits | Self::UP_RED.bits;
        const UP_MASK = !(Self::UP_GREEN.bits | Self::UP_RED.bits);
        const LEFT_GREEN = 0b00000010;
        const LEFT_RED = 0b00010000;
        const LEFT_YELLOW = Self::LEFT_GREEN.bits | Self::LEFT_RED.bits;
        const LEFT_MASK = !(Self::LEFT_GREEN.bits | Self::LEFT_RED.bits);
        const RIGHT_GREEN = 0b00000100;
        const RIGHT_RED = 0b00100000;
        const RIGHT_YELLOW = Self::RIGHT_GREEN.bits | Self::RIGHT_RED.bits;
        const RIGHT_MASK = !(Self::RIGHT_GREEN.bits | Self::RIGHT_RED.bits);
    }
}

pub enum LedColors {
    Off,
    Green,
    Yellow,
    Red
}

pub enum OutputCommands {
    SetLeds(u8),
    SetAllLedsTo(LedColors),
    SetUpLedTo(LedColors),
    SetLeftLedTo(LedColors),
    SetRightLedTo(LedColors)
}

impl Into<u32> for EngineSelection {
    fn into(self) -> u32 {
        self as u8 as u32
    }
}

impl From<u32> for EngineSelection {
    fn from(value: u32) -> Self {
        match value {
            0 => EngineSelection::Invalid,
            1 => EngineSelection::OFF,
            2..=3 => EngineSelection::RIGHT,
            4..=7 => EngineSelection::LEFT,
            8..=15 => EngineSelection::BOTH,
            16..=u32::MAX => EngineSelection::START
        }
    }
}