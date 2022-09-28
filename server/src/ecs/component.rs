pub type ComponentId = u32;

pub trait Component: Send + Sync + 'static {
	fn get_id(&self) -> ComponentId;
	fn set_id(&mut self, id: ComponentId);
}