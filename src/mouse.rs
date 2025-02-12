use stardust_xr_fusion::{
	core::{schemas::flex::flexbuffers, values::Transform},
	data::{PulseReceiver, PulseReceiverHandler},
	fields::Field,
	items::panel::{PanelItem, SurfaceID},
	node::NodeError,
	spatial::Spatial,
	HandlerWrapper,
};
use stardust_xr_molecules::mouse::{MouseEvent, MOUSE_MASK};
use tracing::debug;

pub struct Mouse {
	pub panel_item: Option<PanelItem>,
	// pub panel_item_ui: Weak<Mutex<PanelItemUI>>,
	pub focus: SurfaceID,
}
impl Mouse {
	pub fn new<Fi: Field>(
		spatial_parent: &Spatial,
		transform: Transform,
		field: &Fi,
		panel_item: Option<PanelItem>,
		// panel_item_ui: Weak<Mutex<PanelItemUI>>,
		focus: SurfaceID,
	) -> Result<HandlerWrapper<PulseReceiver, Mouse>, NodeError> {
		PulseReceiver::create(spatial_parent, transform, field, &MOUSE_MASK)?.wrap(Mouse {
			panel_item,
			// panel_item_ui,
			focus,
		})
	}
}
impl PulseReceiverHandler for Mouse {
	fn data(&mut self, _uid: &str, data: &[u8], _data_reader: flexbuffers::MapReader<&[u8]>) {
		if let Some(mouse_event) = MouseEvent::from_pulse_data(data) {
			debug!(?mouse_event, "Mouse event");
			if let Some(panel_item) = &self.panel_item {
				let _ = mouse_event.send_to_panel(panel_item, &self.focus);
			}
			// if let Some(delta) = mouse_event.delta {
			// if let Some(_panel_item_ui) = self.panel_item_ui.upgrade() {
			// debug!(?delta, "Pointer delta");
			// panel_item_ui.lock().pointer_delta(delta);
			// }
			// }
		}
	}
}
