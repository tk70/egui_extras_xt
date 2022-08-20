use egui::{self, Response, Sense, Ui, Widget};
use emath::{Rot2, Vec2};
use epaint::Stroke;

use crate::common::WidgetShape;
use crate::display::{DisplayStyle, DisplayStylePreset};

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct LedDisplay<'a> {
    value: f32,
    diameter: f32,
    shape: WidgetShape<'a>,
    style: DisplayStyle,
    animated: bool,
}

impl<'a> LedDisplay<'a> {
    pub fn new(value: f32) -> Self {
        Self {
            value,
            diameter: 32.0,
            shape: WidgetShape::Circle,
            style: DisplayStylePreset::Default.style(),
            animated: false,
        }
    }

    pub fn from_bool(value: bool) -> Self {
        Self::new(if value { 1.0 } else { 0.0 })
    }

    pub fn diameter(mut self, diameter: impl Into<f32>) -> Self {
        self.diameter = diameter.into();
        self
    }

    pub fn shape(mut self, shape: WidgetShape<'a>) -> Self {
        self.shape = shape;
        self
    }

    pub fn style(mut self, style: DisplayStyle) -> Self {
        self.style = style;
        self
    }

    pub fn style_preset(mut self, preset: DisplayStylePreset) -> Self {
        self.style = preset.style();
        self
    }

    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }
}

impl<'a> Widget for LedDisplay<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::splat(self.diameter);

        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        if ui.is_rect_visible(rect) {
            let value = if self.animated {
                ui.ctx()
                    .animate_value_with_time(response.id, self.value, 0.1)
            } else {
                self.value
            };
            // TODO: clamp

            ui.painter().rect(
                rect,
                ui.style().visuals.noninteractive().rounding,
                self.style.background_color,
                Stroke::none(),
            );

            self.shape.paint_shape(
                ui,
                rect.center(),
                self.diameter / 2.0,
                self.style.foreground_color_blend(value),
                self.style.foreground_stroke_blend(value),
                Rot2::default(),
            );
        }

        response
    }
}
