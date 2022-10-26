pub use makepad_draw_2d::makepad_platform;
pub use makepad_draw_2d::makepad_image_formats;
pub use makepad_draw_2d;

pub mod button;
pub mod label;
pub mod desktop_button;
pub mod desktop_window;
pub mod scroll_shadow;
pub mod scroll_bar;
pub mod scroll_bars;
pub mod link_label;
pub mod list_box;
pub mod data_binding;
pub mod file_tree;
pub mod slides_view;
pub mod log_list;
pub mod log_icon;

//pub mod live_design;

pub mod drop_down;

pub mod dock;
pub mod tab;
pub mod tab_bar;
pub mod tab_close_button;
pub mod color_picker;
pub mod text_input;
pub mod slider;

pub mod check_box;

pub mod popup_menu;

#[macro_use]
pub mod window_menu;

pub use makepad_derive_widget;
pub use makepad_platform::*;
pub use makepad_derive_widget::*;

pub mod bare_window;
pub mod fold_button;

pub mod splitter;
pub mod fold_header;

pub mod debug_view;

//pub mod imgui;
pub mod frame;

pub mod nav_control;

pub mod widget;

mod theme;

pub use crate::{
    data_binding::{DataBinding, MapBindTable, BoolBindTable, DataBindTable},
    bare_window::BareWindow,
    button::*,
    frame::*,
    label::*,
    slider::*,
    check_box::*,
    drop_down::*,
    text_input::{TextInput},
    link_label::{LinkLabel},
    desktop_window::{DesktopWindow},
    scroll_bars::{ScrollBars},
    scroll_shadow::{ScrollShadow},
    scroll_bar::{ScrollBar},
    widget::{
        WidgetUid,
        WidgetDraw,
        WidgetDrawApi,
        CreateAt,
        WidgetActions,
        WidgetActionsApi,
        WidgetActionItem,
        WidgetRef,
        Widget,
        WidgetRegistry,
        WidgetFactory,
        WidgetAction,
    }
};


pub fn live_design(cx: &mut Cx) {
    makepad_draw_2d::live_design(cx);
    crate::log_list::live_design(cx);
    crate::log_icon::live_design(cx);
    crate::debug_view::live_design(cx);
    crate::fold_header::live_design(cx);
    crate::splitter::live_design(cx);
    crate::theme::live_design(cx);
    crate::slider::live_design(cx);
    crate::label::live_design(cx);
    crate::nav_control::live_design(cx);
    crate::frame::live_design(cx);
    crate::fold_button::live_design(cx);
    crate::text_input::live_design(cx);
    crate::link_label::live_design(cx);
    crate::scroll_shadow::live_design(cx);
    crate::button::live_design(cx);
    crate::desktop_button::live_design(cx);
    crate::desktop_window::live_design(cx);
    crate::bare_window::live_design(cx);
    crate::window_menu::live_design(cx);
    crate::scroll_bar::live_design(cx);
    crate::scroll_bars::live_design(cx);
    crate::check_box::live_design(cx);
    crate::tab_close_button::live_design(cx);
    crate::tab::live_design(cx);
    crate::tab_bar::live_design(cx);
    crate::dock::live_design(cx);
    crate::color_picker::live_design(cx);
    crate::file_tree::live_design(cx);
    crate::slides_view::live_design(cx);
    crate::list_box::live_design(cx);
    crate::popup_menu::live_design(cx);
    crate::drop_down::live_design(cx);
}
