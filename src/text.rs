use bunny_ui::{
    Color32,
    ROption::RSome,
    RString, Rect, Vec2,
    align::Align2,
    paint::{
        shapes::{shape::Shape, text_shape::TextShape},
        text::{
            fonts::FontId,
            text_layout_types::{LayoutJob, TextFormat},
        },
    },
    painter::Painter,
};

use crate::position::RelativePosition;

pub struct Text {
    pub text: RString,
    pub position: RelativePosition,
    pub pivot: Align2,
    pub font_id: FontId,
    pub text_color: Color32,
    pub highlight: Option<TextHighlight>,
}

impl Text {
    pub fn new(text: impl Into<RString>, font_id: FontId) -> Self {
        Self {
            text: text.into(),
            position: RelativePosition {
                anchor: Align2::LEFT_TOP,
                offset: Vec2 { x: 0.0, y: 0.0 },
            },
            pivot: Align2::LEFT_TOP,
            font_id,
            text_color: Color32::WHITE,
            highlight: None,
        }
    }

    #[inline]
    pub fn with_pos(mut self, pos: impl Into<Vec2>) -> Self {
        self.position.offset = pos.into();
        self
    }

    #[inline]
    pub fn with_anchor(mut self, anchor: Align2) -> Self {
        self.position.anchor = anchor;
        self
    }

    #[inline]
    pub fn with_pos_rel(mut self, pos: RelativePosition) -> Self {
        self.position = pos;
        self
    }

    #[inline]
    pub fn with_pivot(mut self, pivot: Align2) -> Self {
        self.pivot = pivot;
        self
    }

    #[inline]
    pub fn with_font(mut self, font: FontId) -> Self {
        self.font_id = font;
        self
    }

    #[inline]
    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_id.size = size;
        self
    }

    #[inline]
    pub fn with_color(mut self, color: Color32) -> Self {
        self.text_color = color;
        self
    }

    #[inline]
    pub fn with_highlight(mut self, highlight: impl Into<TextHighlight>) -> Self {
        self.highlight = Some(highlight.into());
        self
    }

    #[inline]
    pub fn with_shadow(self, shadow: TextShadow) -> Self {
        self.with_highlight(shadow)
    }

    #[inline]
    pub fn with_background(self, background: TextBackground) -> Self {
        self.with_highlight(background)
    }
}

impl Text {
    #[inline]
    pub fn highlight_color_mut(&mut self) -> Option<&mut Color32> {
        self.highlight.as_mut().map(|h| match h {
            TextHighlight::Shadow(text_shadow) => &mut text_shadow.color,
            TextHighlight::Background(text_background) => &mut text_background.color,
        })
    }
}

impl Text {
    pub fn paint(self, painter: &Painter, max_rect: Rect) {
        let Text {
            text,
            position,
            pivot,
            font_id,
            text_color,
            highlight,
        } = self;
        let adjusted_pos = position.pos_in_rect(&max_rect);
        if let Some(highlight) = highlight {
            match highlight {
                TextHighlight::Shadow(text_shadow) => {
                    let layout_job = LayoutJob::simple_singleline(text, font_id, text_color);
                    let text_shape = TextShape::new(adjusted_pos, layout_job, pivot, text_color);
                    let mut shadow_shape = text_shape.clone();
                    shadow_shape.pos += text_shadow.offset;
                    shadow_shape.override_text_color = RSome(text_shadow.color);
                    painter.extend([Shape::Text(shadow_shape), Shape::Text(text_shape)]);
                }
                TextHighlight::Background(text_background) => {
                    let format = TextFormat {
                        font_id,
                        color: text_color,
                        background: text_background.color,
                        expand_bg: text_background.expand,
                        ..Default::default()
                    };
                    let layout_job = LayoutJob::simple_format(text, format);
                    painter.text_with_layout_job(adjusted_pos, pivot, layout_job, text_color);
                }
            }
        } else {
            painter.text(adjusted_pos, pivot, text, font_id, text_color);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextHighlight {
    Shadow(TextShadow),
    Background(TextBackground),
}

impl From<TextShadow> for TextHighlight {
    #[inline]
    fn from(value: TextShadow) -> Self {
        Self::Shadow(value)
    }
}

impl From<TextBackground> for TextHighlight {
    #[inline]
    fn from(value: TextBackground) -> Self {
        Self::Background(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextShadow {
    pub offset: Vec2,
    pub color: Color32,
}

impl Default for TextShadow {
    fn default() -> Self {
        Self {
            offset: Vec2 { x: 2.0, y: 2.0 },
            color: Color32::BLACK,
        }
    }
}

impl TextShadow {
    pub fn new(offset: impl Into<Vec2>, color: Color32) -> Self {
        Self {
            offset: offset.into(),
            color,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextBackground {
    pub color: Color32,
    pub expand: f32,
}

impl Default for TextBackground {
    fn default() -> Self {
        Self {
            color: Color32::from_black_alpha(128),
            expand: 1.0,
        }
    }
}

impl TextBackground {
    pub fn new(color: Color32, expand: f32) -> Self {
        Self { color, expand }
    }
}
