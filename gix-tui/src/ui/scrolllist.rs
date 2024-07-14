use super::style::SharedTheme;
use ratatui::{
	backend::Backend,
	buffer::Buffer,
	layout::Rect,
	style::Style,
	text::{Span, Text},
	widgets::{Block, Borders, List, ListItem, Widget},
	Frame,
};
use std::iter::Iterator;

///
struct ScrollableList<'b, L, S>
where
	S: Into<Text<'b>>,
	L: Iterator<Item = S>,
{
	block: Option<Block<'b>>,
	/// Items to be displayed
	items: L,
	/// Base style of the widget
	style: Style,
}

impl<'b, L, S> ScrollableList<'b, L, S>
where
	S: Into<Text<'b>>,
	L: Iterator<Item = S>,
{
	fn new(items: L) -> Self {
		Self {
			block: None,
			items,
			style: Style::default(),
		}
	}

	#[allow(clippy::missing_const_for_fn)]
	fn block(mut self, block: Block<'b>) -> Self {
		self.block = Some(block);
		self
	}
}

impl<'b, L, S> Widget for ScrollableList<'b, L, S>
where
	S: Into<Text<'b>>,
	L: Iterator<Item = S>,
{
	fn render(self, area: Rect, buf: &mut Buffer) {
		// Render items
		List::new(
			self.items.map(ListItem::new).collect::<Vec<ListItem>>(),
		)
		.block(self.block.unwrap_or_default())
		.style(self.style)
		.render(area, buf);
	}
}

pub fn draw_list<'b, B: Backend, L, S>(
	f: &mut Frame<B>,
	r: Rect,
	title: &'b str,
	items: L,
	selected: bool,
	theme: &SharedTheme,
) where
	S: Into<Text<'b>>,
	L: Iterator<Item = S>,
{
	let list = ScrollableList::new(items).block(
		Block::default()
			.title(Span::styled(title, theme.title(selected)))
			.borders(Borders::ALL)
			.border_style(theme.block(selected)),
	);
	f.render_widget(list, r);
}

pub fn draw_list_block<'b, B: Backend, L, S>(
	f: &mut Frame<B>,
	r: Rect,
	block: Block<'b>,
	items: L,
) where
	S: Into<Text<'b>>,
	L: Iterator<Item = S>,
{
	let list = ScrollableList::new(items).block(block);
	f.render_widget(list, r);
}
