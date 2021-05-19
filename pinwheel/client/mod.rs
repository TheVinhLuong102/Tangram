mod element;
mod fragment;
mod node;
mod signal;
mod signal_vec;
mod text;

pub use self::{
	element::{Element, HtmlElementKind, Namespace},
	fragment::{fragment, Fragment},
	node::Node,
	signal::SignalNode,
	signal_vec::SignalVecNode,
	text::{text, Text},
};

pub struct Handle(pub futures::future::AbortHandle);

impl Drop for Handle {
	fn drop(&mut self) {
		self.0.abort();
	}
}
