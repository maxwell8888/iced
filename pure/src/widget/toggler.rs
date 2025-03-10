//! Show toggle controls using togglers.
use crate::widget::{Tree, Widget};
use crate::Element;

use iced_native::event::{self, Event};
use iced_native::layout::{self, Layout};
use iced_native::mouse;
use iced_native::renderer;
use iced_native::text;
use iced_native::widget;
use iced_native::{Clipboard, Length, Point, Rectangle, Shell};

pub use iced_native::widget::toggler::{Appearance, StyleSheet, Toggler};

impl<'a, Message, Renderer> Widget<Message, Renderer>
    for Toggler<'a, Message, Renderer>
where
    Renderer: text::Renderer,
    Renderer::Theme: StyleSheet + widget::text::StyleSheet,
{
    fn width(&self) -> Length {
        <Self as iced_native::Widget<Message, Renderer>>::width(self)
    }

    fn height(&self) -> Length {
        <Self as iced_native::Widget<Message, Renderer>>::height(self)
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        <Self as iced_native::Widget<Message, Renderer>>::layout(
            self, renderer, limits,
        )
    }

    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        <Self as iced_native::Widget<Message, Renderer>>::draw(
            self,
            renderer,
            theme,
            style,
            layout,
            cursor_position,
            viewport,
        )
    }

    fn mouse_interaction(
        &self,
        _state: &Tree,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        <Self as iced_native::Widget<Message, Renderer>>::mouse_interaction(
            self,
            layout,
            cursor_position,
            viewport,
            renderer,
        )
    }

    fn on_event(
        &mut self,
        _state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        <Self as iced_native::Widget<Message, Renderer>>::on_event(
            self,
            event,
            layout,
            cursor_position,
            renderer,
            clipboard,
            shell,
        )
    }
}

impl<'a, Message, Renderer> From<Toggler<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: text::Renderer + 'a,
    Renderer::Theme: StyleSheet + widget::text::StyleSheet,
{
    fn from(toggler: Toggler<'a, Message, Renderer>) -> Self {
        Self::new(toggler)
    }
}
