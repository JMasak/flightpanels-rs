// needs LLVM to compile, download from https://github.com/llvm/llvm-project/releases and install, then set env variable LIBCLANG_PATH
// install vjoy 2.2.2.0 and set Number of Buttons in Configuration to 76

use flightpanels_rs::{ComSelection, EngineSelection, Flightpanels, InputState, SettingSelection};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;
use vjoy::{ButtonState, Device, VJoy};

fn get_button_state(state: bool) -> ButtonState {
    if state {
        ButtonState::Pressed
    } else {
        ButtonState::Released
    }
}

fn update_button(device: &mut Device, button: u8, state: bool) {
    _ = device.set_button(button, get_button_state(state));
}

fn set_range(device: &mut Device, buttons: std::ops::Range<i32>, state: bool) {
    for b in buttons {
        if b <= u8::MAX as i32 {
            _ = device.set_button(b as u8, get_button_state(state));
        }
    }
}

fn main() {
    // let mut vjoy = VJoy::from_default_dll_location()?;
    // let steps = 8;
    // let step = (i16::MAX / steps) as i32;
    // for count in 1..9 {
    //     let device_1 = vjoy.get_device_state_mut(1)?;
    //     let av = count as i32 * step;
    //     device_1.set_button(count, ButtonState::Pressed)?;
    //     device_1.set_axis(1, av)?;
    //     thread::sleep(time::Duration::from_secs(1));
    //     vjoy.update_all_devices()?;
    // }
    //
    let (tx, rx): (Sender<InputState>, Receiver<InputState>) = channel();
    if let Ok(mut vjoy) = VJoy::from_default_dll_location()
        && let Some(mut fp) = Flightpanels::new()
    {
        fp.set_channel(tx);
        thread::spawn(move || {
            fp.rx_tread();
        });
        loop {
            match rx.recv() {
                Ok(state) => {
                    if let Ok(device) = vjoy.get_device_state_mut(1) {
                        if state.multi_panel_initialized {
                            update_button(device, 1, state.multi_panel_state.ap());
                            update_button(device, 2, state.multi_panel_state.hdg());
                            update_button(device, 3, state.multi_panel_state.nav());
                            update_button(device, 4, state.multi_panel_state.ias());
                            update_button(device, 5, state.multi_panel_state.alt());
                            update_button(device, 6, state.multi_panel_state.vs());
                            update_button(device, 7, state.multi_panel_state.apr());
                            update_button(device, 8, state.multi_panel_state.rev());
                            update_button(device, 9, state.multi_panel_state.jog_inc());
                            update_button(device, 10, state.multi_panel_state.jog_dec());
                            update_button(device, 11, state.multi_panel_state.auto_throttle());
                            update_button(device, 12, state.multi_panel_state.flaps_up());
                            update_button(device, 13, state.multi_panel_state.flaps_down());
                            update_button(device, 14, state.multi_panel_state.pitch_up());
                            update_button(device, 15, state.multi_panel_state.pitch_down());
                            set_range(device, 16..21, false);
                            match state.multi_panel_state.selector() {
                                SettingSelection::ALT => update_button(device, 16, true),
                                SettingSelection::VS => update_button(device, 17, true),
                                SettingSelection::IAS => update_button(device, 18, true),
                                SettingSelection::HDG => update_button(device, 19, true),
                                SettingSelection::CRS => update_button(device, 20, true),
                                SettingSelection::Invalid => (),
                            }
                        }
                        if state.radio_panel_initialized {
                            set_range(device, 21..28, false);
                            match state.radio_panel_state.selector1() {
                                ComSelection::COM1 => update_button(device, 21, true),
                                ComSelection::COM2 => update_button(device, 22, true),
                                ComSelection::NAV1 => update_button(device, 23, true),
                                ComSelection::NAV2 => update_button(device, 24, true),
                                ComSelection::ADF => update_button(device, 25, true),
                                ComSelection::DME => update_button(device, 26, true),
                                ComSelection::XPDR => update_button(device, 27, true),
                                ComSelection::Invalid => (),
                            }
                            update_button(device, 28, state.radio_panel_state.coarse_inc1());
                            update_button(device, 29, state.radio_panel_state.coarse_dec1());
                            update_button(device, 30, state.radio_panel_state.fine_inc1());
                            update_button(device, 31, state.radio_panel_state.fine_dec1());
                            update_button(device, 32, state.radio_panel_state.swap1());
                            set_range(device, 33..40, false);
                            match state.radio_panel_state.selector2() {
                                ComSelection::COM1 => update_button(device, 33, true),
                                ComSelection::COM2 => update_button(device, 34, true),
                                ComSelection::NAV1 => update_button(device, 35, true),
                                ComSelection::NAV2 => update_button(device, 36, true),
                                ComSelection::ADF => update_button(device, 37, true),
                                ComSelection::DME => update_button(device, 38, true),
                                ComSelection::XPDR => update_button(device, 39, true),
                                ComSelection::Invalid => (),
                            }
                            update_button(device, 40, state.radio_panel_state.coarse_inc2());
                            update_button(device, 41, state.radio_panel_state.coarse_dec2());
                            update_button(device, 42, state.radio_panel_state.fine_inc2());
                            update_button(device, 43, state.radio_panel_state.fine_dec2());
                            update_button(device, 44, state.radio_panel_state.swap2());
                        }
                        if state.switch_panel_initialized {
                            set_range(device, 45..50, false);
                            match state.switch_panel_state.engine_selector() {
                                EngineSelection::OFF => update_button(device, 45, true),
                                EngineSelection::RIGHT => update_button(device, 46, true),
                                EngineSelection::LEFT => update_button(device, 47, true),
                                EngineSelection::BOTH => update_button(device, 48, true),
                                EngineSelection::START => update_button(device, 49, true),
                                EngineSelection::Invalid => (),
                            }
                            update_button(device, 50, state.switch_panel_state.cowl());
                            update_button(device, 51, state.switch_panel_state.battery());
                            update_button(device, 52, state.switch_panel_state.alt());
                            update_button(device, 53, state.switch_panel_state.avionics());
                            update_button(device, 54, state.switch_panel_state.fuel_pump());
                            update_button(device, 55, state.switch_panel_state.de_ice());
                            update_button(device, 56, state.switch_panel_state.pitot_heat());
                            update_button(device, 57, state.switch_panel_state.panel_lights());
                            update_button(device, 58, state.switch_panel_state.beacon_lights());
                            update_button(device, 59, state.switch_panel_state.navigation_lights());
                            update_button(device, 60, state.switch_panel_state.strobe_lights());
                            update_button(device, 61, state.switch_panel_state.taxi_lights());
                            update_button(device, 62, state.switch_panel_state.landing_lights());
                            update_button(device, 63, state.switch_panel_state.gear_up());
                            update_button(device, 64, state.switch_panel_state.gear_down());
                        }
                        if state.fip_initialized {
                            update_button(device, 65, state.fip_state.s1());
                            update_button(device, 66, state.fip_state.s2());
                            update_button(device, 67, state.fip_state.s3());
                            update_button(device, 68, state.fip_state.s4());
                            update_button(device, 69, state.fip_state.s5());
                            update_button(device, 70, state.fip_state.s6());
                            update_button(device, 71, state.fip_state.up());
                            update_button(device, 72, state.fip_state.down());
                            update_button(device, 73, state.fip_state.left_encoder_inc());
                            update_button(device, 74, state.fip_state.left_encoder_dec());
                            update_button(device, 75, state.fip_state.right_encoder_inc());
                            update_button(device, 76, state.fip_state.right_encoder_dec());
                        }
                    }
                    match vjoy.update_all_devices() {
                        Ok(_) => (),
                        Err(e) => {
                            println!("Error updating vjoy devices: {}", e);
                            return;
                        }
                    }
                }
                Err(e) => {
                    println!("Thread receive error: {}", e);
                    return;
                }
            }
        }
    }
}
