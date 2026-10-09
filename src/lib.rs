use hidapi::HidApi;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

mod flight_instrument_panel;
mod multi_panel;
mod radio_panel;
mod switch_panel;

pub use multi_panel::SettingSelection;
pub use radio_panel::ComSelection;
pub use switch_panel::EngineSelection;

pub struct Flightpanels {
    api: HidApi,
    rx: Receiver<InputData>,
    tx: Option<Sender<InputState>>,
    state: InputState,
    radio_output: Option<Sender<radio_panel::OutputCommands>>,
    switch_output: Option<Sender<switch_panel::OutputCommands>>,
    fip_output: Option<Sender<flight_instrument_panel::OutputCommands>>,
}

pub enum InputData {
    RadioInputData(radio_panel::RadioPanelInputs),
    MultiInputData(multi_panel::MultiPanelInputs),
    SwitchInputData(switch_panel::SwitchPanelInputs),
    FIPInputData(flight_instrument_panel::FlightInstrumentPanelInputs),
}

#[derive(Clone, PartialEq, Eq)]
pub struct InputState {
    pub radio_panel_state: radio_panel::RadioPanelInputs,
    pub multi_panel_state: multi_panel::MultiPanelInputs,
    pub switch_panel_state: switch_panel::SwitchPanelInputs,
    pub fip_state: flight_instrument_panel::FlightInstrumentPanelInputs,
    pub radio_panel_initialized: bool,
    pub multi_panel_initialized: bool,
    pub switch_panel_initialized: bool,
    pub fip_initialized: bool,
}

impl InputState {
    pub fn new() -> Self {
        InputState {
            radio_panel_state: radio_panel::RadioPanelInputs::new(),
            multi_panel_state: multi_panel::MultiPanelInputs::new(),
            switch_panel_state: switch_panel::SwitchPanelInputs::new(),
            fip_state: flight_instrument_panel::FlightInstrumentPanelInputs::new(),
            radio_panel_initialized: false,
            multi_panel_initialized: false,
            switch_panel_initialized: false,
            fip_initialized: false,
        }
    }

    pub fn update(&mut self, data: InputData) -> bool {
        let mut send_update = false;
        match data {
            InputData::RadioInputData(data) => {
                if (data != self.radio_panel_state) || (self.radio_panel_initialized == false) {
                    self.radio_panel_initialized = true;
                    self.radio_panel_state = data;
                    send_update = true;
                }
            }
            InputData::MultiInputData(data) => {
                if (data != self.multi_panel_state) || (self.multi_panel_initialized == false) {
                    self.multi_panel_initialized = true;
                    self.multi_panel_state = data;
                    send_update = true;
                }
            }
            InputData::SwitchInputData(data) => {
                if (data != self.switch_panel_state) || (self.switch_panel_initialized == false) {
                    self.switch_panel_initialized = true;
                    self.switch_panel_state = data;
                    send_update = true;
                }
            }
            InputData::FIPInputData(data) => {
                if (data != self.fip_state) || (self.fip_initialized == false) {
                    self.fip_initialized = true;
                    self.fip_state = data;
                    send_update = true;
                }
            }
        }
        send_update
    }
}

impl Flightpanels {
    pub fn new() -> Option<Self> {
        let state = InputState::new();
        if let Ok(api) = hidapi::HidApi::new() {
            let (tx, rx): (Sender<InputData>, Receiver<InputData>) = mpsc::channel();
            let (switch_tx, switch_rx): (
                Sender<switch_panel::OutputCommands>,
                Receiver<switch_panel::OutputCommands>,
            ) = mpsc::channel();
            let (radio_tx, radio_rx): (
                Sender<radio_panel::OutputCommands>,
                Receiver<radio_panel::OutputCommands>,
            ) = mpsc::channel();
            let (fip_tx, fip_rx): (
                Sender<flight_instrument_panel::OutputCommands>,
                Receiver<flight_instrument_panel::OutputCommands>,
            ) = mpsc::channel();

            if multi_panel::MultiPanel::receive(&api, tx.clone()).is_err() {
                println!("MultiPanel not found");
            }
            let radio_output =
                if radio_panel::RadioPanel::receive(&api, tx.clone(), radio_rx).is_err() {
                    println!("RadioPanel not found");
                    None
                } else {
                    Some(radio_tx)
                };
            let switch_output =
                if switch_panel::SwitchPanel::receive(&api, tx.clone(), switch_rx).is_err() {
                    println!("SwitchPanel not found");
                    None
                } else {
                    Some(switch_tx)
                };
            let fip_output = if flight_instrument_panel::FlightInstrumentPanel::receive(
                &api,
                tx.clone(),
                fip_rx,
            )
            .is_err()
            {
                println!("FIP not found");
                None
            } else {
                Some(fip_tx)
            };
            return Some(Flightpanels {
                api,
                rx,
                tx: None,
                state,
                radio_output,
                switch_output,
                fip_output,
            });
        }
        None
    }

    pub fn set_channel(&mut self, tx: Sender<InputState>) {
        self.tx = Some(tx);
    }

    pub fn rx_tread(&mut self) {
        loop {
            match self.rx.recv() {
                Ok(rec) => {
                    if self.state.update(rec)
                        && let Some(tx) = &self.tx
                    {
                        _ = tx.send(self.state.clone());
                    }
                }
                Err(e) => {
                    println!("Error {}", e);
                    return;
                }
            }
        }
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
