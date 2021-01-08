//! Use a date picker as an input element for picking dates.
//!
//! *This API requires the following crate features to be activated: date_picker*
use std::collections::HashMap;

use crate::style::date_picker::Style;
use crate::style::date_picker::StyleSheet;

use iced_graphics::{Backend, Color, HorizontalAlignment, Primitive, Rectangle, Renderer, VerticalAlignment, backend};
use iced_native::{Element, mouse};
use chrono::{self, Datelike};

use crate::native::date_picker;
pub use crate::native::date_picker::State;

use super::icons::{ICON_FONT, Icon};

/// An input element for picking dates.
/// 
/// This is an alias of an `iced_native` DatePicker with an `iced_wgpu::Renderer`.
pub type DatePicker<'a, Message, Backend> =
    date_picker::DatePicker<'a, Message, Renderer<Backend>>;

// TODO: Merge
#[derive(Eq, Hash, PartialEq)]
enum StyleState {
    Active,
    Hovered,
    Selected,
}

impl<B> date_picker::Renderer for Renderer<B>
where
    B: Backend + backend::Text,
{
    type Style = Box<dyn StyleSheet>;

    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        cursor_position: iced_graphics::Point,
        style_sheet: &Self::Style,
        date: &chrono::NaiveDate,
        year_str: &str,
        month_str: &str,
        cancel_button: &Element<'_, Message, Self>,
        submit_button: &Element<'_, Message, Self>,
        layout: iced_native::Layout<'_>,
    ) -> Self::Output {
        let bounds = layout.bounds();
        let mut children = layout.children();
        let mut date_children = children.next().unwrap().children();

        //let style = style_sheet.active();
        let mut style: HashMap<StyleState, Style> = HashMap::new();
        let _ = style.insert(StyleState::Active, style_sheet.active());
        let _ = style.insert(StyleState::Hovered, style_sheet.hovered());
        let _ = style.insert(StyleState::Selected, style_sheet.selected());
        
        let mouse_interaction = mouse::Interaction::default();

        let style_state = if bounds.contains(cursor_position) {
            StyleState::Hovered
        } else {
            StyleState::Active
        };

        let background = Primitive::Quad {
            bounds: bounds,
            background: style.get(&style_state).unwrap().background, // TODO
            border_radius: style.get(&style_state).unwrap().border_radius as u16, // TODO: will change in the future
            border_width: style.get(&style_state).unwrap().border_width as u16, // TODO: same
            border_color: style.get(&style_state).unwrap().border_color,
        };

        
        // ----------- Year/Month----------------------
        let month_year_layout = date_children.next().unwrap();

        let (month_year, month_year_mouse_interaction) = month_year(
            month_year_layout,
            month_str,
            year_str,
            cursor_position,
            &style,
        );

        // ----------- Days ---------------------------
        let days_layout = date_children.next().unwrap().children().next().unwrap();
        
        let (days, days_mouse_interaction) = days(
            days_layout,
            date,
            cursor_position,
            &style,
        );
        
        // ----------- Buttons ------------------------
        let cancel_button_layout = children.next().unwrap();
        
        let (cancel_button, cancel_mouse_interaciton) = cancel_button.draw(
            self,
            defaults,
            cancel_button_layout,
            cursor_position,
            &bounds,
        );

        let submit_button_layout = children.next().unwrap();

        let (submit_button, submit_mouse_interaction) = submit_button.draw(
            self,
            defaults,
            submit_button_layout,
            cursor_position,
            &bounds,
        );

        (
            Primitive::Group {
                primitives: vec![background, month_year, days, cancel_button, submit_button],
            },
            mouse_interaction
                .max(month_year_mouse_interaction)
                .max(days_mouse_interaction)
                .max(cancel_mouse_interaciton)
                .max(submit_mouse_interaction)
        )
    }
}

/// Draws the month/year row
fn month_year(
    layout: iced_native::Layout<'_>,
    month: &str,
    year: &str,
    cursor_position: iced_graphics::Point,
    //style: &Style,
    style: &HashMap<StyleState, Style>,
) -> (Primitive, mouse::Interaction) {
    let mut children = layout.children();

    let month_layout = children.next().unwrap();
    let year_layout = children.next().unwrap();

    let f = |layout: iced_native::Layout<'_>, text: &str| {
        let mut children = layout.children();

        let left_bounds = children.next().unwrap().bounds();
        let center_bounds = children.next().unwrap().bounds();
        let right_bounds = children.next().unwrap().bounds();

        let mut mouse_interaction = mouse::Interaction::default();

        let left_arrow_hovered = left_bounds.contains(cursor_position);
        let right_arrow_hovered = right_bounds.contains(cursor_position);

        if left_arrow_hovered || right_arrow_hovered {
            mouse_interaction = mouse_interaction.max(mouse::Interaction::Pointer);
        }

        let primitive = Primitive::Group {
            primitives: vec![
                Primitive::Text {
                    content: Icon::CaretLeftFill.into(),
                    bounds: Rectangle {
                        x: left_bounds.center_x(),
                        y: left_bounds.center_y(),
                        .. left_bounds
                    },
                    //color: style.text_color,
                    color: style.get(&StyleState::Active).unwrap().text_color,
                    size: left_bounds.height + if left_arrow_hovered { 5.0 } else { 0.0 },
                    font: ICON_FONT,
                    horizontal_alignment: HorizontalAlignment::Center,
                    vertical_alignment: VerticalAlignment::Center,
                },

                Primitive::Text {
                    content: text.to_owned(),
                    bounds: Rectangle {
                        x: center_bounds.center_x(),
                        y: center_bounds.center_y(),
                        .. center_bounds
                    },
                    color: style.get(&StyleState::Active).unwrap().text_color,
                    size: center_bounds.height,
                    font: Default::default(),
                    horizontal_alignment: HorizontalAlignment::Center,
                    vertical_alignment: VerticalAlignment::Center,
                },

                Primitive::Text {
                    content: Icon::CaretRightFill.into(),
                    bounds: Rectangle {
                        x: right_bounds.center_x(),
                        y: right_bounds.center_y(),
                        .. right_bounds
                    },
                    color: style.get(&StyleState::Active).unwrap().text_color,
                    size: right_bounds.height + if right_arrow_hovered { 5.0 } else { 0.0 },
                    font: ICON_FONT,
                    horizontal_alignment: HorizontalAlignment::Center,
                    vertical_alignment: VerticalAlignment::Center,
                },
            ]
        };

        (primitive, mouse_interaction)
    };

    let mouse_interaction = mouse::Interaction::default();

    let (month, month_mouse_interaction) = f(month_layout, month);

    let (year, year_mouse_interaction) = f(year_layout, year);

    (
        Primitive::Group {
            primitives: vec![month, year],
        },
        mouse_interaction
            .max(month_mouse_interaction)
            .max(year_mouse_interaction)
    )
}

/// Draws the days
fn days(
    layout: iced_native::Layout<'_>,
    date: &chrono::NaiveDate,
    cursor_position: iced_graphics::Point,
    //style: &Style,
    style: &HashMap<StyleState, Style>,
) -> (Primitive, mouse::Interaction) {
    let mut children = layout.children();

    let day_labels_layout = children.next().unwrap();
    let labels = day_labels(day_labels_layout, style);
    
    let (table, table_mouse_interaction) = day_table(
        &mut children,
        date,
        cursor_position,
        style
    );

    (
        Primitive::Group {
            primitives: vec![labels, table]
        },
        table_mouse_interaction,
    )
}

/// Draws the day labels
fn day_labels(
    layout: iced_native::Layout<'_>,
    //style: &Style,
    style: &HashMap<StyleState, Style>,
) -> Primitive {
    let mut labels: Vec<Primitive> = Vec::new();

    for (i, label) in layout.children().enumerate() {
        let bounds = label.bounds();

        labels.push(
            Primitive::Text {
                content: format!("{}", crate::core::date::WEEKDAY_LABELS[i]),
                bounds: Rectangle {
                    x: bounds.center_x(),
                    y: bounds.center_y(),
                    .. bounds
                },
                color: style.get(&StyleState::Active).unwrap().text_color,
                size: bounds.height + 5.0,
                font: Default::default(),
                horizontal_alignment: HorizontalAlignment::Center,
                vertical_alignment: VerticalAlignment::Center,
            }
        )
    }

    Primitive::Group {
        primitives: labels,
    }
}

/// Draws the day table
fn day_table(
    children: &mut dyn Iterator<Item=iced_native::Layout<'_>>,
    date: &chrono::NaiveDate,
    cursor_position: iced_graphics::Point,
    //style: &Style,
    style: &HashMap<StyleState, Style>,
) -> (Primitive, mouse::Interaction) {
    let mut primitives: Vec<Primitive> = Vec::new();

    let mut mouse_interaction = mouse::Interaction::default();

    for (y, row) in children.enumerate() {
        for (x, label) in row.children().enumerate() {
            let bounds = label.bounds();
            let (number, is_in_month) = crate::core::date::position_to_day(x, y, date.year(), date.month());
            
            let mouse_over = bounds.contains(cursor_position);
            if mouse_over {
                mouse_interaction = mouse_interaction.max(mouse::Interaction::Pointer);
            }

            let selected = date.day() == number as u32 && is_in_month == 0;

            let style_state = if mouse_over {
                StyleState::Hovered
            } else if selected {
                StyleState::Selected
            } else {
                StyleState::Active
            };

            primitives.push(
                Primitive::Quad {
                    bounds: bounds,
                    background: style.get(&style_state).unwrap().day_background,
                    border_radius: bounds.height as u16 / 2,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                }
            );

            primitives.push(
                Primitive::Text {
                    content: format!("{:02}", number),
                    bounds: Rectangle {
                        x: bounds.center_x(),
                        y: bounds.center_y(),
                        .. bounds
                    },
                    color: if is_in_month == 0 {
                        style.get(&style_state).unwrap().text_color
                    } else {
                        style.get(&style_state).unwrap().text_attenuated_color
                    },
                    size: if bounds.width < bounds.height {
                        bounds.width
                    } else {
                        bounds.height
                    },
                    font: Default::default(),
                    horizontal_alignment: HorizontalAlignment::Center,
                    vertical_alignment: VerticalAlignment::Center,
                }
            )
        }
    }

    (
        Primitive::Group {
            primitives: primitives,
        },
        mouse_interaction
    )
}
