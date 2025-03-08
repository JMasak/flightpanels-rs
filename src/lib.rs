use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use hidapi::HidApi;

mod multi_panel;
mod radio_panel;
mod switch_panel;
mod flight_instrument_panel;

struct Flightpanels {
    api: HidApi
}

pub enum InputData {
    RadioInputData(radio_panel::RadioPanelInputs),
    MultiInputData(multi_panel::MultiPanelInputs),
    SwitchInputData(switch_panel::SwitchPanelInputs),
    FIPInputData(flight_instrument_panel::FlightInstrumentPanelInputs)
}

impl Flightpanels {
    fn new() -> Option<Self> {
        if let Ok(api) = hidapi::HidApi::new() {
            let (tx, rx): (Sender<InputData>, Receiver<InputData>) = mpsc::channel();
            let (switch_tx, switch_rx): (Sender<switch_panel::OutputCommands>, Receiver<switch_panel::OutputCommands>) = mpsc::channel();
            let (radio_tx, radio_rx): (Sender<radio_panel::OutputCommands>, Receiver<radio_panel::OutputCommands>) = mpsc::channel();
            let (fip_tx, fip_rx): (Sender<flight_instrument_panel::OutputCommands>, Receiver<flight_instrument_panel::OutputCommands>) = mpsc::channel();

            multi_panel::MultiPanel::receive(&api, tx.clone()).expect("could not create thread for multi panel");
            radio_panel::RadioPanel::receive(&api, tx.clone(), radio_rx).expect("could not create thread for radio panel");
            switch_panel::SwitchPanel::receive(&api, tx.clone(), switch_rx).expect("could not create thread for switch panel");
            flight_instrument_panel::FlightInstrumentPanel::receive(&api, tx.clone(), fip_rx).expect("could not create thread for FIP");

            loop {
                match rx.recv() {
                    Ok(rec) => match rec {
                        InputData::MultiInputData(data) => (),//println!("{:#?}", data),
                        InputData::RadioInputData(data) => (),//println!("{:#?}", data),
                        InputData::SwitchInputData(data) => (),//{
                        //     match data.engine_selector() {
                        //         switch_panel::EngineSelection::LEFT => {switch_tx.send(switch_panel::OutputCommands::SetLeftLedTo(switch_panel::LedColors::Green));},
                        //         switch_panel::EngineSelection::RIGHT => {switch_tx.send(switch_panel::OutputCommands::SetRightLedTo(switch_panel::LedColors::Green));},
                        //         switch_panel::EngineSelection::BOTH => {switch_tx.send(switch_panel::OutputCommands::SetUpLedTo(switch_panel::LedColors::Green));},
                        //         switch_panel::EngineSelection::OFF => {switch_tx.send(switch_panel::OutputCommands::SetAllLedsTo(switch_panel::LedColors::Off));},
                        //         switch_panel::EngineSelection::START => {switch_tx.send(switch_panel::OutputCommands::SetLeds((switch_panel::GearLedsStates::LEFT_GREEN | switch_panel::GearLedsStates::UP_YELLOW | switch_panel::GearLedsStates::RIGHT_RED).bits()));},
                        //         switch_panel::EngineSelection::Invalid => ()
                        //     }
                        //     println!("{:#?}", data)
                        // },
                        InputData::FIPInputData(data) => (),//println!("{:#?}", data),
                    },
                    Err(e) => println!("Error {}", e)
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::switch_panel;

    #[test]
    fn basic_test() {
        crate::Flightpanels::new();
    }
}
