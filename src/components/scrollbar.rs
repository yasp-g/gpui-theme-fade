use crate::AppView;
use gpui::{
    prelude::*, px, size, point, quad, App, Bounds, Element, ElementId,
    GlobalElementId, Hsla, IntoElement, LayoutId, MouseButton, MouseDownEvent, MouseMoveEvent,
    MouseUpEvent, Pixels, Point, Position, ScrollHandle, Style, Window, hsla, IsZero, InspectorElementId,
};

// --- 1. Public Function ---
pub fn render_scrollbar(
    id: impl Into<ElementId>,
    scroll_handle: &ScrollHandle,
) -> ScrollbarElement {
    ScrollbarElement {
        id: id.into(),
        scroll_handle: scroll_handle.clone(),
    }
}

// --- 2. State Management ---

const SCROLLBAR_THUMB_WIDTH: Pixels = px(8.0);
const MIN_THUMB_HEIGHT: Pixels = px(24.0);

#[derive(Clone, PartialEq, Default)]
enum ThumbState {
    #[default]
    Inactive,
    Hovered,
    Dragging {
        drag_start_pos: Point<Pixels>,
        scroll_handle_start_offset: Point<Pixels>,
    },
}

#[derive(Clone)]
struct ScrollbarState {
    thumb_state: ThumbState,
}

// --- 3. The Custom Element ---

pub struct ScrollbarElement {
    id: ElementId,
    scroll_handle: ScrollHandle,
}

impl IntoElement for ScrollbarElement {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}

// --- 4. Prepaint State ---
#[derive(Default, Clone)]
pub struct ScrollbarPrepaintState {
    thumb_bounds: Bounds<Pixels>,
    scrollable_height: Pixels,
    scroll_track_height: Pixels,
}

// --- 5. The Element Trait Implementation ---

impl Element for ScrollbarElement {
    type RequestLayoutState = ();
    type PrepaintState = ScrollbarPrepaintState;

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let style = Style {
            position: Position::Absolute,
            inset: gpui::Edges::default(),
            ..Default::default()
        };
        (window.request_layout(style, None, cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
        let scroll_track_bounds = self.scroll_handle.bounds();
        let content_size = self.scroll_handle.max_offset() + scroll_track_bounds.size;
        let scrollable_height = content_size.height;

        if scrollable_height <= scroll_track_bounds.size.height {
            return ScrollbarPrepaintState::default();
        }

        let thumb_height = (scroll_track_bounds.size.height / scrollable_height)
            * scroll_track_bounds.size.height;
        let thumb_height = thumb_height.max(MIN_THUMB_HEIGHT);

        let scroll_top = self.scroll_handle.offset().y.abs();
        let scroll_ratio = scroll_top / (scrollable_height - scroll_track_bounds.size.height);

        let thumb_top = scroll_ratio * (scroll_track_bounds.size.height - thumb_height);

        let thumb_bounds = Bounds {
            origin: bounds.origin + point(bounds.size.width - SCROLLBAR_THUMB_WIDTH, thumb_top),
            size: size(SCROLLBAR_THUMB_WIDTH, thumb_height),
        };

        ScrollbarPrepaintState {
            thumb_bounds,
            scrollable_height,
            scroll_track_height: scroll_track_bounds.size.height,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _layout: &mut Self::RequestLayoutState,
        prepaint_state: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        if prepaint_state.scrollable_height.is_zero() {
            return;
        }

        let state_entity = window.use_keyed_state(self.id.clone(), cx, |_, _| {
            ScrollbarState {
                thumb_state: ThumbState::default(),
            }
        });

        let current_state = state_entity.read(cx).clone();

        let bg_color = match current_state.thumb_state {
            ThumbState::Inactive => hsla(0.0, 0.0, 0.5, 0.5),
            ThumbState::Hovered => hsla(0.0, 0.0, 0.6, 0.6),
            ThumbState::Dragging { .. } => hsla(0.0, 0.0, 0.7, 0.7),
        };

        window.paint_quad(quad(
            prepaint_state.thumb_bounds,
            gpui::Corners::all(px(4.0)),
            bg_color,
            gpui::Edges::default(),
            gpui::black(),
            gpui::BorderStyle::default(),
        ));

        let state_entity_for_down = state_entity.clone();
        let scroll_handle_for_down = self.scroll_handle.clone();
        let prepaint_state_for_down = prepaint_state.clone();
        window.on_mouse_event(move |event: &MouseDownEvent, phase, window, cx| {
            if phase.bubble()
                && event.button == MouseButton::Left
                && prepaint_state_for_down.thumb_bounds.contains(&event.position)
            {
                cx.stop_propagation();
                state_entity_for_down.update(cx, |state, _| {
                    state.thumb_state = ThumbState::Dragging {
                        drag_start_pos: event.position,
                        scroll_handle_start_offset: scroll_handle_for_down.offset(),
                    };
                });
                window.refresh();
            }
        });

        let state_entity_for_up = state_entity.clone();
        let prepaint_state_for_up = prepaint_state.clone();
        window.on_mouse_event(move |event: &MouseUpEvent, phase, window, cx| {
            if phase.bubble() && event.button == MouseButton::Left {
                if let ThumbState::Dragging { .. } = state_entity_for_up.read(cx).thumb_state {
                    state_entity_for_up.update(cx, |state, _| {
                        state.thumb_state = ThumbState::Inactive;
                    });
                    window.refresh();
                }
            }
        });

        let state_entity_for_move = state_entity.clone();
        let scroll_handle_for_move = self.scroll_handle.clone();
        let prepaint_state_for_move = prepaint_state.clone();
        window.on_mouse_event(move |event: &MouseMoveEvent, phase, window, cx| {
            if !phase.bubble() {
                return;
            }

            match state_entity_for_move.read(cx).thumb_state.clone() {
                ThumbState::Dragging {
                    drag_start_pos,
                    scroll_handle_start_offset,
                } => {
                    let delta_y = event.position.y - drag_start_pos.y;
                    let scrollable_dist =
                        prepaint_state_for_move.scrollable_height - prepaint_state_for_move.scroll_track_height;
                    let scroll_delta =
                        (delta_y / prepaint_state_for_move.scroll_track_height) * scrollable_dist;

                    let new_offset_y = (scroll_handle_start_offset.y + scroll_delta)
                        .clamp(-scrollable_dist, px(0.0));
                    scroll_handle_for_move.set_offset(point(scroll_handle_start_offset.x, new_offset_y));
                    window.refresh();
                }
                _ => {
                    if prepaint_state_for_move.thumb_bounds.contains(&event.position) {
                        if state_entity_for_move.read(cx).thumb_state != ThumbState::Hovered {
                            state_entity_for_move.update(cx, |state, _| {
                                state.thumb_state = ThumbState::Hovered;
                            });
                            window.refresh();
                        }
                    } else if state_entity_for_move.read(cx).thumb_state != ThumbState::Inactive {
                        state_entity_for_move.update(cx, |state, _| {
                            state.thumb_state = ThumbState::Inactive;
                        });
                        window.refresh();
                    }
                }
            }
        });
    }
}