use hidapi::{HidApi, HidDevice};

mod multi_panel;
mod radio_panel;

struct Flightpanels {
    api: HidApi,
    radio_panel: Option<HidDevice>,
    multi_panel: Option<HidDevice>
}

impl Flightpanels {
    fn new() -> Option<Self> {
        if let Ok(api) = hidapi::HidApi::new() {
            let multi_panel;
            if let Ok(mp) = api.open(multi_panel::ID.0, multi_panel::ID.1) {
                multi_panel = Some(mp);
            }
            else {
                multi_panel = None;
            }
            let radio_panel;
            if let Ok(rp) = api.open(radio_panel::ID.0, radio_panel::ID.1) {
                radio_panel = Some(rp);
            }
            else {
                radio_panel = None;
            }

            if multi_panel.is_none() && radio_panel.is_none() {
                return None
            }
            else {
                return Some(Flightpanels {
                    api: api,
                    multi_panel: multi_panel,
                    radio_panel: radio_panel
                })
            }
        }
        None
    }
}
