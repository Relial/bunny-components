use bunny_plugin::bunny_ui::{
    Color32, Rect, Vec2,
    align::Align2,
    paint::{
        corner_radius::CornerRadius,
        stroke::{Stroke, StrokeKind},
    },
    painter::Painter,
    vec2,
};

use crate::position::RelativePosition;

pub struct ProgressBar {
    progress: f32,
    size: Vec2,
    position: RelativePosition,
    pivot: Align2,
    border_stroke: Option<Stroke>,
    progress_color: Color32,
    background_color: Option<Color32>,
}

impl ProgressBar {
    pub fn new(progress: f32) -> Self {
        Self {
            progress: progress.clamp(0.0, 1.0),
            size: vec2(100.0, 20.0),
            position: RelativePosition {
                anchor: Align2::LEFT_TOP,
                offset: Vec2 { x: 0.0, y: 0.0 },
            },
            pivot: Align2::LEFT_TOP,
            border_stroke: None,
            progress_color: Color32::WHITE,
            background_color: None,
        }
    }

    #[inline]
    pub fn size(mut self, size: impl Into<Vec2>) -> Self {
        self.size = size.into();
        self
    }

    #[inline]
    pub fn width(mut self, width: f32) -> Self {
        self.size.x = width;
        self
    }

    #[inline]
    pub fn height(mut self, height: f32) -> Self {
        self.size.y = height;
        self
    }

    #[inline]
    pub fn pos_rel(mut self, pos: RelativePosition) -> Self {
        self.position = pos;
        self
    }

    #[inline]
    pub fn pos(mut self, pos: impl Into<Vec2>) -> Self {
        self.position.offset = pos.into();
        self
    }

    #[inline]
    pub fn anchor(mut self, anchor: Align2) -> Self {
        self.position.anchor = anchor;
        self
    }

    #[inline]
    pub fn pivot(mut self, pivot: Align2) -> Self {
        self.pivot = pivot;
        self
    }

    #[inline]
    pub fn stroke(mut self, stroke: Stroke) -> Self {
        self.border_stroke = Some(stroke);
        self
    }

    #[inline]
    pub fn bar_color(mut self, color: Color32) -> Self {
        self.progress_color = color;
        self
    }

    #[inline]
    pub fn background_color(mut self, color: Color32) -> Self {
        self.background_color = Some(color);
        self
    }
}

impl ProgressBar {
    pub fn paint(self, painter: &Painter, max_rect: &Rect) -> Rect {
        let ProgressBar {
            progress,
            size,
            position,
            pivot,
            border_stroke,
            progress_color,
            background_color,
        } = self;
        let pos = position.pos_in_rect(max_rect);
        let outer_rect = pivot.anchor_size(pos, size);
        let inner_rect = if let Some(stroke) = border_stroke {
            painter.rect_stroke(outer_rect, CornerRadius::ZERO, stroke, StrokeKind::Inside);
            outer_rect.shrink(stroke.width)
        } else {
            outer_rect
        };
        if let Some(bg_fill) = background_color {
            painter.rect_filled(inner_rect, CornerRadius::ZERO, bg_fill);
        }
        let progress_width = inner_rect.width() * progress;
        let progress_rect =
            Rect::from_min_size(inner_rect.min, vec2(progress_width, inner_rect.height()));
        painter.rect_filled(progress_rect, CornerRadius::ZERO, progress_color);
        outer_rect
    }
}
