use crate::event::{self, Event};
use crate::layout;
use crate::mouse;
use crate::overlay;
use crate::renderer;
use crate::widget::container;
use crate::widget::pane_grid::{Draggable, TitleBar};
use crate::{Clipboard, Element, Layout, Point, Rectangle, Shell, Size};

/// The content of a [`Pane`].
///
/// [`Pane`]: crate::widget::pane_grid::Pane
#[allow(missing_debug_implementations)]
pub struct Content<'a, Message, Renderer>
where
    Renderer: crate::Renderer,
    Renderer::Theme: container::StyleSheet,
{
    title_bar: Option<TitleBar<'a, Message, Renderer>>,
    body: Element<'a, Message, Renderer>,
    style: <Renderer::Theme as container::StyleSheet>::Style,
}

impl<'a, Message, Renderer> Content<'a, Message, Renderer>
where
    Renderer: crate::Renderer,
    Renderer::Theme: container::StyleSheet,
{
    /// Creates a new [`Content`] with the provided body.
    pub fn new(body: impl Into<Element<'a, Message, Renderer>>) -> Self {
        Self {
            title_bar: None,
            body: body.into(),
            style: Default::default(),
        }
    }

    /// Sets the [`TitleBar`] of this [`Content`].
    pub fn title_bar(
        mut self,
        title_bar: TitleBar<'a, Message, Renderer>,
    ) -> Self {
        self.title_bar = Some(title_bar);
        self
    }

    /// Sets the style of the [`Content`].
    pub fn style(
        mut self,
        style: impl Into<<Renderer::Theme as container::StyleSheet>::Style>,
    ) -> Self {
        self.style = style.into();
        self
    }
}

impl<'a, Message, Renderer> Content<'a, Message, Renderer>
where
    Renderer: crate::Renderer,
    Renderer::Theme: container::StyleSheet,
{
    /// Draws the [`Content`] with the provided [`Renderer`] and [`Layout`].
    ///
    /// [`Renderer`]: crate::Renderer
    pub fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        use container::StyleSheet;

        let bounds = layout.bounds();

        {
            let style = theme.appearance(self.style);

            container::draw_background(renderer, &style, bounds);
        }

        if let Some(title_bar) = &self.title_bar {
            let mut children = layout.children();
            let title_bar_layout = children.next().unwrap();
            let body_layout = children.next().unwrap();

            let show_controls = bounds.contains(cursor_position);

            title_bar.draw(
                renderer,
                theme,
                style,
                title_bar_layout,
                cursor_position,
                viewport,
                show_controls,
            );

            self.body.draw(
                renderer,
                theme,
                style,
                body_layout,
                cursor_position,
                viewport,
            );
        } else {
            self.body.draw(
                renderer,
                theme,
                style,
                layout,
                cursor_position,
                viewport,
            );
        }
    }

    pub(crate) fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        if let Some(title_bar) = &self.title_bar {
            let max_size = limits.max();

            let title_bar_layout = title_bar
                .layout(renderer, &layout::Limits::new(Size::ZERO, max_size));

            let title_bar_size = title_bar_layout.size();

            let mut body_layout = self.body.layout(
                renderer,
                &layout::Limits::new(
                    Size::ZERO,
                    Size::new(
                        max_size.width,
                        max_size.height - title_bar_size.height,
                    ),
                ),
            );

            body_layout.move_to(Point::new(0.0, title_bar_size.height));

            layout::Node::with_children(
                max_size,
                vec![title_bar_layout, body_layout],
            )
        } else {
            self.body.layout(renderer, limits)
        }
    }

    pub(crate) fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        is_picked: bool,
    ) -> event::Status {
        let mut event_status = event::Status::Ignored;

        let body_layout = if let Some(title_bar) = &mut self.title_bar {
            let mut children = layout.children();

            event_status = title_bar.on_event(
                event.clone(),
                children.next().unwrap(),
                cursor_position,
                renderer,
                clipboard,
                shell,
            );

            children.next().unwrap()
        } else {
            layout
        };

        let body_status = if is_picked {
            event::Status::Ignored
        } else {
            self.body.on_event(
                event,
                body_layout,
                cursor_position,
                renderer,
                clipboard,
                shell,
            )
        };

        event_status.merge(body_status)
    }

    pub(crate) fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let (body_layout, title_bar_interaction) =
            if let Some(title_bar) = &self.title_bar {
                let mut children = layout.children();
                let title_bar_layout = children.next().unwrap();

                let is_over_pick_area = title_bar
                    .is_over_pick_area(title_bar_layout, cursor_position);

                if is_over_pick_area {
                    return mouse::Interaction::Grab;
                }

                let mouse_interaction = title_bar.mouse_interaction(
                    title_bar_layout,
                    cursor_position,
                    viewport,
                    renderer,
                );

                (children.next().unwrap(), mouse_interaction)
            } else {
                (layout, mouse::Interaction::default())
            };

        self.body
            .mouse_interaction(body_layout, cursor_position, viewport, renderer)
            .max(title_bar_interaction)
    }

    pub(crate) fn overlay(
        &mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'_, Message, Renderer>> {
        if let Some(title_bar) = self.title_bar.as_mut() {
            let mut children = layout.children();
            let title_bar_layout = children.next()?;

            match title_bar.overlay(title_bar_layout, renderer) {
                Some(overlay) => Some(overlay),
                None => self.body.overlay(children.next()?, renderer),
            }
        } else {
            self.body.overlay(layout, renderer)
        }
    }
}

impl<'a, Message, Renderer> Draggable for &Content<'a, Message, Renderer>
where
    Renderer: crate::Renderer,
    Renderer::Theme: container::StyleSheet,
{
    fn can_be_dragged_at(
        &self,
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> bool {
        if let Some(title_bar) = &self.title_bar {
            let mut children = layout.children();
            let title_bar_layout = children.next().unwrap();

            title_bar.is_over_pick_area(title_bar_layout, cursor_position)
        } else {
            false
        }
    }
}

impl<'a, T, Message, Renderer> From<T> for Content<'a, Message, Renderer>
where
    T: Into<Element<'a, Message, Renderer>>,
    Renderer: crate::Renderer,
    Renderer::Theme: container::StyleSheet,
{
    fn from(element: T) -> Self {
        Self::new(element)
    }
}
