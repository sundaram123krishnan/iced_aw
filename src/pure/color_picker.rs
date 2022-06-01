//! Use a color picker as an input element for picking colors.
//!
//! *This API requires the following crate features to be activated: `color_picker`*

use iced_graphics::{Backend, Renderer};
use iced_native::{
    event, mouse, overlay, renderer, Clipboard, Color, Event, Layout, Length, Point, Rectangle,
    Shell,
};
use iced_pure::{
    widget::{
        tree::{self, Tag},
        Tree,
    },
    Element, Widget,
};

use crate::native::overlay::color_picker::ColorPickerOverlay;
pub use crate::style::color_picker::{Style, StyleSheet};

use crate::native::color_picker::State;

//TODO: Remove ignore when Null is updated. Temp fix for Test runs
/// An input element for picking colors.
///
/// # Example
/// ```ignore
/// # use iced_aw::color_picker;
/// # use iced_native::{widget::{button, Button, Text}, Color, renderer::Null};
/// #
/// # pub type ColorPicker<'a, Message> = iced_aw::native::ColorPicker<'a, Message, Null>;
/// #[derive(Clone, Debug)]
/// enum Message {
///     Open,
///     Cancel,
///     Submit(Color),
/// }
///
/// let mut button_state = button::State::new();
/// let mut state = color_picker::State::new();
/// state.show(true);
///
/// let color_picker = ColorPicker::new(
///     &mut state,
///     Button::new(&mut button_state, Text::new("Pick color"))
///         .on_press(Message::Open),
///     Message::Cancel,
///     Message::Submit,
/// );
/// ```
#[allow(missing_debug_implementations)]
pub struct ColorPicker<'a, Message, B>
where
    Message: Clone,
    B: Backend,
{
    /// Show the picker.
    show_picker: bool,
    /// The underlying element.
    underlay: Element<'a, Message, Renderer<B>>,
    /// The message that is send if the cancel button of the [`ColorPickerOverlay`](ColorPickerOverlay) is pressed.
    on_cancel: Message,
    /// The function thet produces a message when the submit button of the [`ColorPickerOverlay`](ColorPickerOverlay) is pressed.
    on_submit: Box<dyn Fn(Color) -> Message>,
    /// The style of the [`ColorPickerOverlay`](ColorPickerOverlay).
    style_sheet: Box<dyn StyleSheet + 'a>,
}

impl<'a, Message, B> ColorPicker<'a, Message, B>
where
    Message: Clone,
    B: Backend,
{
    /// Creates a new [`ColorPicker`](ColorPicker) wrapping around the given underlay.
    ///
    /// It expects:
    ///     * a mutable reference to the [`ColorPicker`](ColorPicker)'s [`State`](State).
    ///     * the underlay [`Element`](iced_native::Element) on which this [`ColorPicker`](ColorPicker)
    ///         will be wrapped around.
    ///     * a message that will be send when the cancel button of the [`ColorPicker`](ColorPicker)
    ///         is pressed.
    ///     * a function that will be called when the submit button of the [`ColorPicker`](ColorPicker)
    ///         is pressed, which takes the picked [`Color`](iced_native::Color) value.
    pub fn new<U, F>(show_picker: bool, underlay: U, on_cancel: Message, on_submit: F) -> Self
    where
        U: Into<Element<'a, Message, Renderer<B>>>,
        F: 'static + Fn(Color) -> Message,
    {
        Self {
            show_picker,
            underlay: underlay.into(),
            on_cancel,
            on_submit: Box::new(on_submit),
            style_sheet: std::boxed::Box::default(),
        }
    }

    /// Sets the style of the [`ColorPicker`](ColorPicker).
    #[must_use]
    pub fn style(mut self, style_sheet: impl Into<Box<dyn StyleSheet>>) -> Self {
        self.style_sheet = style_sheet.into();
        self
    }
}

impl<'a, Message, B> Widget<Message, Renderer<B>> for ColorPicker<'a, Message, B>
where
    Message: 'static + Clone,
    B: 'a + Backend + iced_graphics::backend::Text,
{
    fn tag(&self) -> Tag {
        Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.underlay)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.underlay));
    }

    fn width(&self) -> Length {
        self.underlay.as_widget().width()
    }

    fn height(&self) -> Length {
        self.underlay.as_widget().width()
    }

    fn layout(
        &self,
        renderer: &Renderer<B>,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        self.underlay.as_widget().layout(renderer, limits)
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer<B>,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        self.underlay.as_widget_mut().on_event(
            &mut state.children[0],
            event,
            layout,
            cursor_position,
            renderer,
            clipboard,
            shell,
        )
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        renderer: &Renderer<B>,
    ) -> mouse::Interaction {
        self.underlay.as_widget().mouse_interaction(
            &state.children[0],
            layout,
            cursor_position,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        state: &iced_pure::widget::Tree,
        renderer: &mut Renderer<B>,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        self.underlay.as_widget().draw(
            &state.children[0],
            renderer,
            style,
            layout,
            cursor_position,
            viewport,
        );
    }

    fn overlay<'b>(
        &'b self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer<B>,
    ) -> Option<overlay::Element<'b, Message, Renderer<B>>> {
        let picker_state: &mut State = state.state.downcast_mut();

        if !self.show_picker {
            return self
                .underlay
                .as_widget()
                .overlay(&mut state.children[0], layout, renderer);
        }

        let bounds = layout.bounds();
        let position = Point::new(bounds.center_x(), bounds.center_y());

        Some(
            ColorPickerOverlay::new(
                picker_state,
                self.on_cancel.clone(),
                &self.on_submit,
                position,
                &self.style_sheet,
            )
            .overlay(),
        )
    }
}

impl<'a, Message, B> From<ColorPicker<'a, Message, B>> for Element<'a, Message, Renderer<B>>
where
    Message: 'static + Clone,
    B: 'a + Backend + iced_graphics::backend::Text,
{
    fn from(color_picker: ColorPicker<'a, Message, B>) -> Self {
        Element::new(color_picker)
    }
}
