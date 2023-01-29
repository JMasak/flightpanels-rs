use bitfield_struct::bitfield;

/*
outputs: 23bytes
byte 0: 0    // HID report ID
bytes 1-5: 5 Ziffern oberes linkes Display (kein ASCII - Werte 0-9 für Ziffern, D0-D1 für Ziffer gefolgt von Dezimalpunkt, 0xFF für leerzeichen))
bytes 6-10: 5 Ziffern oberes rechtes Display (kein ASCII - Werte 0-9)
bytes 11-15: 5 Ziffern unteres linkes Display (kein ASCII - Werte 0-9)
bytes 16-20: 5 Ziffern unteres rechtes Display (kein ASCII - Werte 0-9)
bytes 21-22: padding?
*/



pub const ID: (u16, u16) = (0x06A3, 0x0D05);

#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct RadioPanelInputs {
    #[bits(7)]
    selector1: ComSelection,
    #[bits(7)]
    selector2: ComSelection,
    swap1: bool,
    swap2: bool,
    fine_inc1: bool,
    fine_dec1: bool,
    coarse_inc1: bool,
    coarse_dec1: bool,
    fine_inc2: bool,
    fine_dec2: bool,
    coarse_inc2: bool,
    coarse_dec2: bool,
    #[bits(8)]
    _pad: u32
}

pub enum RadioDisplay {
    UpperActive,
    UpperStandby,
    LowerActive,
    LowerStandby
}

pub struct RadioPanelOutputs {
    pub upper_active_display: [u8; 5],
    pub upper_standby_display: [u8; 5],
    pub lower_active_display: [u8; 5],
    pub lower_standby_display: [u8; 5],
}

impl RadioPanelOutputs {
    pub fn as_bytes(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::with_capacity(23);
        data.push(0);   // hid report no.
        data.extend_from_slice(&self.upper_active_display[0..]);
        data.extend_from_slice(&self.upper_standby_display[0..]);
        data.extend_from_slice(&self.lower_active_display[0..]);
        data.extend_from_slice(&self.lower_standby_display[0..]);
        data.push(0); // padding?
        data.push(0);
        data
    }

    pub fn set_display(mut self, display: RadioDisplay, value: f32) -> Result<(), &'static str>{
        let mut display_data: [u8; 5] = [0xff; 5];
        if value < 0.0 {
            return Err("Displays cannot show negative values");
        }
        if value > 99999.0 {
            return Err("Displays cannot show more than 5 figures")
        }
        let mut first_digit = true;
        let shift: u8;
        let mut tmp_value: u32;
        if value >= 10000.0 { shift = 0; tmp_value = value as u32;}
        else if value >= 1000.0 { shift = 1; tmp_value = (value * 10.0) as u32;}
        else { shift = 2; tmp_value = (value * 100.0) as u32;}
        let mut figure: u8 = (tmp_value / 10000).try_into().expect("could not convert to figure");
        if figure > 0 {
            display_data[0] = figure;
            first_digit = false;
            tmp_value %= 10000;
        }
        figure = (tmp_value / 1000).try_into().expect("could not convert to figure");
        if figure > 0 {
            display_data[1] = figure;
            first_digit = false;
            tmp_value %= 1000;
        }
        else if !first_digit {
            display_data[1] = 0;
        }
        figure = (tmp_value / 100).try_into().expect("could not convert to figure");
        if figure > 0 {
            if shift == 2 {
                figure += 0xD0;
            }
            display_data[2] = figure;
            first_digit = false;
            tmp_value %= 100;
        }
        else {
            if shift == 2 {
                display_data[2] = 0xD0;
            }
            else if !first_digit {
                display_data[2] = 0;
            }
        }
        figure = (tmp_value / 10).try_into().expect("could not convert to figure");
        if figure > 0 {
            if shift == 1 {
                figure += 0xD0;
            }
            display_data[3] = figure;
            first_digit = false;
            tmp_value %= 10;
        }
        else {
            if shift == 1 {
                display_data[3] = 0xD0;
            }
            else if !first_digit {
                display_data[3] = 0;
            }
        }
        display_data[4] = (tmp_value % 10).try_into().expect("could not convert to figure");
        match display {
            RadioDisplay::UpperActive => { self.upper_active_display.swap_with_slice(&mut display_data) },
            RadioDisplay::UpperStandby => { self.upper_standby_display.swap_with_slice(&mut display_data) },
            RadioDisplay::LowerActive => { self.lower_active_display.swap_with_slice(&mut display_data) },
            RadioDisplay::LowerStandby => { self.lower_standby_display.swap_with_slice(&mut display_data) }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ComSelection {
    Invalid = 0,
    COM1 = 1,
    COM2 = 2,
    NAV1 = 4,
    NAV2 = 8,
    ADF = 16,
    DME = 32,
    XPDR = 64
}

impl Into<u32> for ComSelection {
    fn into(self) -> u32 {
        self as u8 as u32
    }
}

impl From<u32> for ComSelection {
    fn from(value: u32) -> Self {
        match value {
            0 => ComSelection::Invalid,
            1 => ComSelection::COM1,
            2..=3 => ComSelection::COM2,
            4..=7 => ComSelection::NAV1,
            8..=15 => ComSelection::NAV2,
            16..=31 => ComSelection::ADF,
            32..=63 => ComSelection::DME,
            64..=u32::MAX => ComSelection::XPDR
        }
    }
}
