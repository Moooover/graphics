use crate::gradient::{GradStyle, GradStyle::Plain};
use crate::gui::lpicklist::LPickList;
use crate::size::{SizeControls, SizeMessage};
use iced::widget::Column;
use iced::Element;

#[derive(Debug, Clone)]
pub enum ExtrudeMessage {
    Size(SizeMessage),
    GradStyle(GradStyle),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExtrudeControls {
    pub size_controls: SizeControls,
    pub grad_style: Option<GradStyle>,
    pub dirty: bool,
}

impl Default for ExtrudeControls {
    fn default() -> Self {
        Self {
            size_controls: SizeControls::default(),
            grad_style: Some(Plain),
            dirty: false,
        }
    }
}

impl<'a> ExtrudeControls {
    pub fn new(size_controls: SizeControls, grad_style: Option<GradStyle>, dirty: bool) -> Self {
        Self {
            size_controls,
            grad_style,
            dirty,
        }
    }

    pub fn update(&mut self, message: ExtrudeMessage) {
        use self::ExtrudeMessage::*;
        match message {
            Size(x) => {
                self.size_controls.update(x);
                self.dirty = self.size_controls.dirty;
            }
            GradStyle(grad_style) => {
                self.grad_style = Some(grad_style);
                self.dirty = true;
            }
            Null => (),
        }
    }

    pub fn view(&self) -> Element<'a, ExtrudeMessage> {
        use self::GradStyle::*;
        use ExtrudeMessage::*;
        let mut col = Column::new().push(
            SizeControls::new(
                self.size_controls.size_fn,
                self.size_controls.size,
                self.size_controls.direction,
                self.size_controls.size_scale,
                self.size_controls.min_size,
                self.size_controls.dirty,
            )
            .view()
            .map(ExtrudeMessage::Size),
        );
        col = col
            .push(LPickList::new(
                "Gradient Style".to_string(),
                vec![Plain, Light, Dark, Fiber, LightFiber, DarkFiber],
                self.grad_style,
                |x| x.map_or(Null, GradStyle),
            ))
            .spacing(15);
        col.into()
    }
}
