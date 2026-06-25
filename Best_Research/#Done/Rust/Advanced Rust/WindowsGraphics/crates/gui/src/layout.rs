// public types
/// A position expressed as a fraction of the screen or an owning box.
pub type ScreenRelativePosition = [f32; 2];

/// A colour expressed as red, green, blue, and alpha components.
pub(crate) type UserInterfaceColor = [f32; 4];

/// Space around or inside a user-interface box, expressed in pixels.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct UserInterfaceInsets
{
    /// Space along the left edge.
    pub left: f32,
    /// Space along the top edge.
    pub top: f32,
    /// Space along the right edge.
    pub right: f32,
    /// Space along the bottom edge.
    pub bottom: f32,
}

/// An explicit length used to override a box dimension.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum UserInterfaceLength
{
    /// Uses the size allocated by the parent layout.
    Fill,
    /// Uses a fixed number of pixels, clamped to the available size.
    Pixels(f32),
}

/// A rectangle whose values are fractions of its parent rectangle.
///
/// Root boxes use the screen as their parent. Child boxes use their immediate
/// parent. A value of `1.0` represents the full corresponding parent dimension.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct UserInterfaceRelativeRectangle
{
    /// Horizontal position relative to the left edge of the parent.
    pub left: f32,
    /// Vertical position relative to the top edge of the parent.
    pub top: f32,
    /// Width relative to the width of the parent.
    pub width: f32,
    /// Height relative to the height of the parent.
    pub height: f32,
}

/// A resolved rectangle in viewport pixels.
///
/// This is output from layout resolution. Product layout code should normally
/// use [`UserInterfaceRelativeRectangle`] instead.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct UserInterfacePixelRectangle
{
    /// Horizontal position in viewport pixels.
    pub left: f32,
    /// Vertical position in viewport pixels.
    pub top: f32,
    /// Width in viewport pixels.
    pub width: f32,
    /// Height in viewport pixels.
    pub height: f32,
}

/// The arrangement rule for a box's direct children.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum UserInterfaceChildrenLayout
{
    /// Each child uses its own relative rectangle within the parent.
    #[default]
    Freeform,
    /// Direct children are joined edge-to-edge from left to right.
    Horizontal,
    /// Direct children are joined edge-to-edge from top to bottom.
    Vertical,
}

/// The drawing layer for a user-interface box.
///
/// Lower order values draw first. Equal layers retain tree order, which means
/// a child draws over its parent when both use the same layer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct UserInterfaceLayer
{
    order: i32,
}

/// The layout and child-arrangement settings of one user-interface box.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct UserInterfaceBoxLayout
{
    /// Bounds relative to the screen for a root box or its parent for a child box.
    pub relative_bounds: UserInterfaceRelativeRectangle,
    /// Determines how direct child boxes are placed.
    pub children_layout: UserInterfaceChildrenLayout,
    /// Share of available space when this box belongs to a glued layout.
    pub layout_weight: f32,
    /// Optional width override applied after the parent allocates the box.
    pub width: UserInterfaceLength,
    /// Optional height override applied after the parent allocates the box.
    pub height: UserInterfaceLength,
    /// Space outside this box.
    pub margin: UserInterfaceInsets,
    /// Space inside this box before its children are arranged.
    pub padding: UserInterfaceInsets,
}

/// The visual properties of one user-interface box.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct UserInterfaceBoxAppearance
{
    /// Fill colour used to draw the box.
    pub background_color: UserInterfaceColor,
    /// Whether the box background should be drawn.
    pub is_background_visible: bool,
    /// Whether this box and all its children should be drawn.
    pub is_visible: bool,
}

/// A public node returned by the `gui::ui` construction macros.
pub struct UserInterfaceNode
{
    user_interface_box: UserInterfaceBox,
}

/// A retained box that can own child boxes.
pub(crate) struct UserInterfaceBox
{
    layout: UserInterfaceBoxLayout,
    appearance: UserInterfaceBoxAppearance,
    layer: UserInterfaceLayer,
    text: Option<String>,
    children: Vec<UserInterfaceBox>,
}

/// A resolved, drawable box produced from a [`UserInterfaceLayout`].
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ResolvedUserInterfaceBox
{
    /// The box bounds in the current viewport.
    pub pixel_rectangle: UserInterfacePixelRectangle,
    /// The area available for this box's text and children after padding.
    pub content_pixel_rectangle: UserInterfacePixelRectangle,
    /// The box fill colour.
    pub background_color: UserInterfaceColor,
    /// Whether the box fill should be emitted as draw data.
    pub is_background_visible: bool,
    /// The layer used to order drawing.
    pub layer: UserInterfaceLayer,
    /// Optional text drawn inside the box.
    pub text: Option<String>,
}

/// A complete retained layout for one screen.
pub struct UserInterfaceLayout
{
    root_boxes: Vec<UserInterfaceBox>,
    zoom_factor: f32,
    zoom_centre: ScreenRelativePosition,
}
// public types

// domain constants
const DEFAULT_BACKGROUND_COLOR: UserInterfaceColor = [0.12, 0.14, 0.18, 1.0];
const DEFAULT_BUTTON_BACKGROUND_COLOR: UserInterfaceColor = [0.16, 0.28, 0.42, 1.0];
const DEFAULT_LIST_BACKGROUND_COLOR: UserInterfaceColor = [0.08, 0.10, 0.14, 1.0];
const DEFAULT_PANEL_PADDING: f32 = 12.0;
const DEFAULT_BUTTON_VERTICAL_PADDING: f32 = 8.0;
const DEFAULT_BUTTON_HORIZONTAL_PADDING: f32 = 12.0;
#[cfg(test)]
const MINIMUM_ZOOM_FACTOR: f32 = 0.01;
const DEFAULT_ZOOM_CENTRE: ScreenRelativePosition = [0.5, 0.5];
// domain constants

impl UserInterfaceRelativeRectangle
{
    /// The full area of the parent rectangle.
    pub(crate) const FULL: Self = Self {
        left: 0.0,
        top: 0.0,
        width: 1.0,
        height: 1.0,
    };

    /// Creates a rectangle relative to its parent.
    pub(crate) const fn new(left: f32, top: f32, width: f32, height: f32) -> Self
    {
        return Self {
            left,
            top,
            width,
            height,
        };
    }
}

impl UserInterfaceInsets
{
    /// Creates equal spacing on every edge.
    pub(crate) const fn all(value: f32) -> Self
    {
        return Self {
            left: value,
            top: value,
            right: value,
            bottom: value,
        };
    }

    /// Creates independent spacing for every edge.
    pub(crate) const fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self
    {
        return Self {
            left,
            top,
            right,
            bottom,
        };
    }
}

impl From<f32> for UserInterfaceInsets
{
    fn from(value: f32) -> Self
    {
        return Self::all(value);
    }
}

impl From<i32> for UserInterfaceInsets
{
    fn from(value: i32) -> Self
    {
        return Self::all(value as f32);
    }
}

impl From<f32> for UserInterfaceLength
{
    fn from(value: f32) -> Self
    {
        return Self::Pixels(value);
    }
}

impl From<i32> for UserInterfaceLength
{
    fn from(value: i32) -> Self
    {
        return Self::Pixels(value as f32);
    }
}

impl UserInterfaceLayer
{
    /// The layer behind regular content.
    #[cfg(test)]
    pub(crate) const BACKGROUND: Self = Self { order: 0 };
    /// The default content layer.
    pub(crate) const CONTENT: Self = Self { order: 100 };
    /// The layer for floating content such as tooltips.
    #[cfg(test)]
    pub(crate) const OVERLAY: Self = Self { order: 200 };
}

impl Default for UserInterfaceBoxLayout
{
    fn default() -> Self
    {
        return Self {
            relative_bounds: UserInterfaceRelativeRectangle::FULL,
            children_layout: UserInterfaceChildrenLayout::Freeform,
            layout_weight: 1.0,
            width: UserInterfaceLength::Fill,
            height: UserInterfaceLength::Fill,
            margin: UserInterfaceInsets::default(),
            padding: UserInterfaceInsets::default(),
        };
    }
}

impl Default for UserInterfaceBoxAppearance
{
    fn default() -> Self
    {
        return Self {
            background_color: DEFAULT_BACKGROUND_COLOR,
            is_background_visible: true,
            is_visible: true,
        };
    }
}

impl UserInterfaceNode
{
    pub(crate) fn from_box(user_interface_box: UserInterfaceBox) -> Self
    {
        return Self { user_interface_box };
    }

    pub(crate) fn into_box(self) -> UserInterfaceBox
    {
        return self.user_interface_box;
    }

    #[cfg(test)]
    pub(crate) fn layout(&self) -> UserInterfaceBoxLayout
    {
        return self.user_interface_box.layout();
    }

    #[cfg(test)]
    pub(crate) fn text(&self) -> Option<&str>
    {
        return self.user_interface_box.text();
    }

    #[cfg(test)]
    pub(crate) fn children(&self) -> &[UserInterfaceBox]
    {
        return self.user_interface_box.children();
    }

    pub(crate) fn set_width(&mut self, width: f32)
    {
        self.user_interface_box.set_width(width);
    }

    pub(crate) fn set_height(&mut self, height: f32)
    {
        self.user_interface_box.set_height(height);
    }

    pub(crate) fn set_layout_weight(&mut self, layout_weight: f32)
    {
        self.user_interface_box.set_layout_weight(layout_weight);
    }

    pub(crate) fn set_margin(&mut self, margin: f32)
    {
        self.user_interface_box.set_margin(margin);
    }

    pub(crate) fn set_padding(&mut self, padding: f32)
    {
        self.user_interface_box.set_padding(padding);
    }

    pub(crate) fn set_text(&mut self, text: impl Into<String>)
    {
        self.user_interface_box.set_text(text);
    }

    pub(crate) fn add_child(&mut self, child: UserInterfaceNode)
    {
        self.user_interface_box.add_child(child.into_box());
    }
}

impl UserInterfaceBox
{
    /// Creates an invisible container that arranges its direct children vertically.
    pub(crate) fn new_container(mut layout: UserInterfaceBoxLayout) -> Self
    {
        layout.children_layout = UserInterfaceChildrenLayout::Vertical;

        let mut container = Self::new(layout);
        let appearance = UserInterfaceBoxAppearance {
            is_background_visible: false,
            ..Default::default()
        };

        container.set_appearance(appearance);

        return container;
    }

    /// Creates an invisible container that arranges its direct children horizontally.
    pub(crate) fn new_row(mut layout: UserInterfaceBoxLayout) -> Self
    {
        layout.children_layout = UserInterfaceChildrenLayout::Horizontal;

        let mut row = Self::new(layout);
        let appearance = UserInterfaceBoxAppearance {
            is_background_visible: false,
            ..Default::default()
        };

        row.set_appearance(appearance);

        return row;
    }

    /// Creates a visible box with default appearance on the content layer.
    pub(crate) fn new(layout: UserInterfaceBoxLayout) -> Self
    {
        return Self {
            layout,
            appearance: UserInterfaceBoxAppearance::default(),
            layer: UserInterfaceLayer::CONTENT,
            text: None,
            children: Vec::new(),
        };
    }

    /// Creates a panel that arranges its direct children vertically.
    pub(crate) fn new_panel(mut layout: UserInterfaceBoxLayout) -> Self
    {
        layout.children_layout = UserInterfaceChildrenLayout::Vertical;

        if layout.padding == UserInterfaceInsets::default()
        {
            layout.padding = UserInterfaceInsets::all(DEFAULT_PANEL_PADDING);
        }

        return Self::new(layout);
    }

    /// Creates a box with the default visual treatment for a button.
    ///
    /// The button becomes interactive when the caller supplies an input
    /// snapshot through the widget layer. Until then it remains a retained,
    /// visible control in the layout tree.
    pub(crate) fn new_button(layout: UserInterfaceBoxLayout) -> Self
    {
        let mut button_layout = layout;

        if button_layout.padding == UserInterfaceInsets::default()
        {
            button_layout.padding = UserInterfaceInsets::new(
                DEFAULT_BUTTON_HORIZONTAL_PADDING,
                DEFAULT_BUTTON_VERTICAL_PADDING,
                DEFAULT_BUTTON_HORIZONTAL_PADDING,
                DEFAULT_BUTTON_VERTICAL_PADDING,
            );
        }

        let mut button = Self::new(button_layout);
        let appearance = UserInterfaceBoxAppearance {
            background_color: DEFAULT_BUTTON_BACKGROUND_COLOR,
            ..Default::default()
        };

        button.set_appearance(appearance);

        return button;
    }

    /// Creates a button with visible text.
    pub(crate) fn new_button_with_text(
        layout: UserInterfaceBoxLayout,
        text: impl Into<String>,
    ) -> Self
    {
        let mut button = Self::new_button(layout);

        button.set_text(text);

        return button;
    }

    /// Creates a vertically arranged box with the default visual treatment for a list.
    pub(crate) fn new_list(mut layout: UserInterfaceBoxLayout) -> Self
    {
        layout.children_layout = UserInterfaceChildrenLayout::Vertical;

        let mut list = Self::new(layout);
        let appearance = UserInterfaceBoxAppearance {
            background_color: DEFAULT_LIST_BACKGROUND_COLOR,
            ..Default::default()
        };

        list.set_appearance(appearance);

        return list;
    }

    /// Creates a text-only box that fills its layout bounds without a background.
    pub(crate) fn new_text(layout: UserInterfaceBoxLayout, text: impl Into<String>) -> Self
    {
        let mut text_box = Self::new(layout);
        let appearance = UserInterfaceBoxAppearance {
            is_background_visible: false,
            ..Default::default()
        };

        text_box.set_appearance(appearance);
        text_box.set_text(text);

        return text_box;
    }

    /// Returns this box's placement and child-arrangement settings.
    pub(crate) fn layout(&self) -> UserInterfaceBoxLayout
    {
        return self.layout;
    }

    /// Replaces this box's placement and child-arrangement settings.
    pub(crate) fn set_layout(&mut self, layout: UserInterfaceBoxLayout)
    {
        self.layout = layout;
    }

    /// Applies an explicit width override to this box.
    pub(crate) fn set_width(&mut self, width: impl Into<UserInterfaceLength>)
    {
        self.layout.width = width.into();
    }

    /// Applies an explicit height override to this box.
    pub(crate) fn set_height(&mut self, height: impl Into<UserInterfaceLength>)
    {
        self.layout.height = height.into();
    }

    /// Changes the amount of glued-layout space this box receives.
    pub(crate) fn set_layout_weight(&mut self, layout_weight: f32)
    {
        self.layout.layout_weight = layout_weight;
    }

    /// Applies outer spacing to this box.
    pub(crate) fn set_margin(&mut self, margin: impl Into<UserInterfaceInsets>)
    {
        self.layout.margin = margin.into();
    }

    /// Applies inner spacing before this box arranges text and children.
    pub(crate) fn set_padding(&mut self, padding: impl Into<UserInterfaceInsets>)
    {
        self.layout.padding = padding.into();
    }

    /// Returns this box's visual properties.
    #[cfg(test)]
    pub(crate) fn appearance(&self) -> UserInterfaceBoxAppearance
    {
        return self.appearance;
    }

    /// Replaces this box's visual properties.
    pub(crate) fn set_appearance(&mut self, appearance: UserInterfaceBoxAppearance)
    {
        self.appearance = appearance;
    }

    /// Returns the layer used to draw this box.
    #[cfg(test)]
    pub(crate) fn set_layer(&mut self, layer: UserInterfaceLayer)
    {
        self.layer = layer;
    }

    /// Returns the text drawn inside this box, when text has been assigned.
    #[cfg(test)]
    pub(crate) fn text(&self) -> Option<&str>
    {
        return self.text.as_deref();
    }

    /// Replaces the text drawn inside this box.
    pub(crate) fn set_text(&mut self, text: impl Into<String>)
    {
        self.text = Some(text.into());
    }

    /// Adds a child that will follow this box when layout is resolved.
    pub(crate) fn add_child(&mut self, child: UserInterfaceBox)
    {
        self.children.push(child);
    }

    /// Returns this box's direct children.
    #[cfg(test)]
    pub(crate) fn children(&self) -> &[UserInterfaceBox]
    {
        return &self.children;
    }
}

impl UserInterfaceLayout
{
    /// Creates an empty layout at normal scale.
    pub(crate) fn new() -> Self
    {
        return Self {
            root_boxes: Vec::new(),
            zoom_factor: 1.0,
            zoom_centre: DEFAULT_ZOOM_CENTRE,
        };
    }

    /// Adds a root box whose bounds are relative to the screen.
    pub(crate) fn add_box(&mut self, user_interface_box: UserInterfaceBox)
    {
        self.root_boxes.push(user_interface_box);
    }

    /// Returns the root boxes that make up this screen layout.
    #[cfg(test)]
    pub(crate) fn boxes(&self) -> &[UserInterfaceBox]
    {
        return &self.root_boxes;
    }

    /// Returns the scale applied when resolving this layout.
    #[cfg(test)]
    pub(crate) fn zoom_factor(&self) -> f32
    {
        return self.zoom_factor;
    }

    /// Changes the scale applied when resolving this layout.
    ///
    /// Non-finite and non-positive values are clamped to a small valid scale.
    #[cfg(test)]
    pub(crate) fn set_zoom_factor(&mut self, zoom_factor: f32)
    {
        self.zoom_factor = valid_zoom_factor(zoom_factor);
    }

    /// Resolves all visible boxes into viewport pixels in drawing order.
    pub(crate) fn resolve(&self, viewport_pixel_size: [u32; 2]) -> Vec<ResolvedUserInterfaceBox>
    {
        let root_rectangle = UserInterfacePixelRectangle {
            left: 0.0,
            top: 0.0,
            width: viewport_pixel_size[0] as f32,
            height: viewport_pixel_size[1] as f32,
        };
        let mut resolved_boxes = Vec::new();

        for user_interface_box in &self.root_boxes
        {
            let allocated_rectangle = rectangle_relative_to_parent(
                user_interface_box.layout.relative_bounds,
                root_rectangle,
            );
            let box_rectangle = apply_box_layout(
                user_interface_box.layout,
                allocated_rectangle,
            );
            resolve_box(
                user_interface_box,
                box_rectangle,
                root_rectangle,
                self.zoom_factor,
                self.zoom_centre,
                &mut resolved_boxes,
            );
        }

        resolved_boxes.sort_by_key(|resolved_box| resolved_box.layer);

        return resolved_boxes;
    }
}

impl Default for UserInterfaceLayout
{
    fn default() -> Self
    {
        return Self::new();
    }
}

fn resolve_box(
    user_interface_box: &UserInterfaceBox,
    box_rectangle: UserInterfacePixelRectangle,
    viewport_rectangle: UserInterfacePixelRectangle,
    zoom_factor: f32,
    zoom_centre: ScreenRelativePosition,
    resolved_boxes: &mut Vec<ResolvedUserInterfaceBox>,
)
{
    if !user_interface_box.appearance.is_visible
    {
        return;
    }

    let content_rectangle = inset_rectangle(box_rectangle, user_interface_box.layout.padding);
    let zoomed_rectangle = apply_zoom(
        box_rectangle,
        viewport_rectangle,
        zoom_factor,
        zoom_centre,
    );
    let zoomed_content_rectangle = apply_zoom(
        content_rectangle,
        viewport_rectangle,
        zoom_factor,
        zoom_centre,
    );
    resolved_boxes.push(ResolvedUserInterfaceBox {
        pixel_rectangle: zoomed_rectangle,
        content_pixel_rectangle: zoomed_content_rectangle,
        background_color: user_interface_box.appearance.background_color,
        is_background_visible: user_interface_box.appearance.is_background_visible,
        layer: user_interface_box.layer,
        text: user_interface_box.text.clone(),
    });

    match user_interface_box.layout.children_layout
    {
        UserInterfaceChildrenLayout::Freeform =>
        {
            for child_box in &user_interface_box.children
            {
                let allocated_rectangle = rectangle_relative_to_parent(
                    child_box.layout.relative_bounds,
                    content_rectangle,
                );
                let child_rectangle = apply_box_layout(
                    child_box.layout,
                    allocated_rectangle,
                );
                resolve_box(
                    child_box,
                    child_rectangle,
                    viewport_rectangle,
                    zoom_factor,
                    zoom_centre,
                    resolved_boxes,
                );
            }
        }
        UserInterfaceChildrenLayout::Horizontal =>
        {
            resolve_glued_children(
                user_interface_box,
                content_rectangle,
                viewport_rectangle,
                zoom_factor,
                zoom_centre,
                resolved_boxes,
                true,
            );
        }
        UserInterfaceChildrenLayout::Vertical =>
        {
            resolve_glued_children(
                user_interface_box,
                content_rectangle,
                viewport_rectangle,
                zoom_factor,
                zoom_centre,
                resolved_boxes,
                false,
            );
        }
    }
}

fn resolve_glued_children(
    parent_box: &UserInterfaceBox,
    parent_rectangle: UserInterfacePixelRectangle,
    viewport_rectangle: UserInterfacePixelRectangle,
    zoom_factor: f32,
    zoom_centre: ScreenRelativePosition,
    resolved_boxes: &mut Vec<ResolvedUserInterfaceBox>,
    is_horizontal: bool,
)
{
    let total_layout_weight = total_layout_weight(&parent_box.children);

    if total_layout_weight == 0.0
    {
        return;
    }

    let mut current_offset = 0.0;

    for child_box in &parent_box.children
    {
        let layout_weight = valid_layout_weight(child_box.layout.layout_weight);
        let proportion = layout_weight / total_layout_weight;
        let child_rectangle = if is_horizontal
        {
            UserInterfacePixelRectangle {
                left: parent_rectangle.left + current_offset,
                top: parent_rectangle.top,
                width: parent_rectangle.width * proportion,
                height: parent_rectangle.height,
            }
        }
        else
        {
            UserInterfacePixelRectangle {
                left: parent_rectangle.left,
                top: parent_rectangle.top + current_offset,
                width: parent_rectangle.width,
                height: parent_rectangle.height * proportion,
            }
        };

        if is_horizontal
        {
            current_offset += child_rectangle.width;
        }
        else
        {
            current_offset += child_rectangle.height;
        }

        let child_rectangle = apply_box_layout(
            child_box.layout,
            child_rectangle,
        );

        resolve_box(
            child_box,
            child_rectangle,
            viewport_rectangle,
            zoom_factor,
            zoom_centre,
            resolved_boxes,
        );
    }
}

fn total_layout_weight(children: &[UserInterfaceBox]) -> f32
{
    let mut total_layout_weight = 0.0;

    for child_box in children
    {
        total_layout_weight += valid_layout_weight(child_box.layout.layout_weight);
    }

    return total_layout_weight;
}

fn rectangle_relative_to_parent(
    relative_rectangle: UserInterfaceRelativeRectangle,
    parent_rectangle: UserInterfacePixelRectangle,
) -> UserInterfacePixelRectangle
{
    return UserInterfacePixelRectangle {
        left: parent_rectangle.left + valid_coordinate(relative_rectangle.left) * parent_rectangle.width,
        top: parent_rectangle.top + valid_coordinate(relative_rectangle.top) * parent_rectangle.height,
        width: valid_extent(relative_rectangle.width) * parent_rectangle.width,
        height: valid_extent(relative_rectangle.height) * parent_rectangle.height,
    };
}

fn apply_box_layout(
    layout: UserInterfaceBoxLayout,
    allocated_rectangle: UserInterfacePixelRectangle,
) -> UserInterfacePixelRectangle
{
    let width = resolved_length(
        layout.width,
        allocated_rectangle.width,
    );
    let height = resolved_length(
        layout.height,
        allocated_rectangle.height,
    );
    let constrained_rectangle = UserInterfacePixelRectangle {
        left: allocated_rectangle.left,
        top: allocated_rectangle.top,
        width,
        height,
    };

    return inset_rectangle(constrained_rectangle, layout.margin);
}

fn resolved_length(
    length: UserInterfaceLength,
    allocated_extent: f32,
) -> f32
{
    match length
    {
        UserInterfaceLength::Fill => return allocated_extent,
        UserInterfaceLength::Pixels(pixel_extent) =>
        {
            return valid_extent(pixel_extent).min(allocated_extent);
        }
    }
}

fn inset_rectangle(
    rectangle: UserInterfacePixelRectangle,
    insets: UserInterfaceInsets,
) -> UserInterfacePixelRectangle
{
    let left_inset = valid_extent(insets.left);
    let top_inset = valid_extent(insets.top);
    let right_inset = valid_extent(insets.right);
    let bottom_inset = valid_extent(insets.bottom);

    return UserInterfacePixelRectangle {
        left: rectangle.left + left_inset,
        top: rectangle.top + top_inset,
        width: (rectangle.width - left_inset - right_inset).max(0.0),
        height: (rectangle.height - top_inset - bottom_inset).max(0.0),
    };
}

fn apply_zoom(
    rectangle: UserInterfacePixelRectangle,
    viewport_rectangle: UserInterfacePixelRectangle,
    zoom_factor: f32,
    zoom_centre: ScreenRelativePosition,
) -> UserInterfacePixelRectangle
{
    let zoom_centre_left = viewport_rectangle.left
        + valid_coordinate(zoom_centre[0]) * viewport_rectangle.width;
    let zoom_centre_top = viewport_rectangle.top
        + valid_coordinate(zoom_centre[1]) * viewport_rectangle.height;

    return UserInterfacePixelRectangle {
        left: zoom_centre_left + (rectangle.left - zoom_centre_left) * zoom_factor,
        top: zoom_centre_top + (rectangle.top - zoom_centre_top) * zoom_factor,
        width: rectangle.width * zoom_factor,
        height: rectangle.height * zoom_factor,
    };
}

fn valid_coordinate(value: f32) -> f32
{
    if value.is_finite()
    {
        return value;
    }

    return 0.0;
}

fn valid_extent(value: f32) -> f32
{
    if !value.is_finite() || value <= 0.0
    {
        return 0.0;
    }

    return value;
}

fn valid_layout_weight(layout_weight: f32) -> f32
{
    return valid_extent(layout_weight);
}

#[cfg(test)]
fn valid_zoom_factor(zoom_factor: f32) -> f32
{
    if !zoom_factor.is_finite() || zoom_factor < MINIMUM_ZOOM_FACTOR
    {
        return MINIMUM_ZOOM_FACTOR;
    }

    return zoom_factor;
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn root_box_uses_screen_relative_bounds()
    {
        let mut layout = UserInterfaceLayout::new();
        let user_interface_box = UserInterfaceBox::new(UserInterfaceBoxLayout {
            relative_bounds: UserInterfaceRelativeRectangle::new(0.1, 0.2, 0.3, 0.4),
            ..Default::default()
        });
        layout.add_box(user_interface_box);

        let resolved_boxes = layout.resolve([1_000, 500]);

        assert_eq!(resolved_boxes.len(), 1);
        assert_eq!(
            resolved_boxes[0].pixel_rectangle,
            UserInterfacePixelRectangle {
                left: 100.0,
                top: 100.0,
                width: 300.0,
                height: 200.0,
            },
        );
    }

    #[test]
    fn child_box_follows_its_parent()
    {
        let mut layout = UserInterfaceLayout::new();
        let mut parent_box = UserInterfaceBox::new(UserInterfaceBoxLayout {
            relative_bounds: UserInterfaceRelativeRectangle::new(0.2, 0.2, 0.4, 0.4),
            ..Default::default()
        });
        let child_box = UserInterfaceBox::new(UserInterfaceBoxLayout {
            relative_bounds: UserInterfaceRelativeRectangle::new(0.25, 0.5, 0.5, 0.25),
            ..Default::default()
        });
        parent_box.add_child(child_box);
        layout.add_box(parent_box);

        let resolved_boxes = layout.resolve([1_000, 1_000]);

        assert_eq!(resolved_boxes.len(), 2);
        assert_eq!(
            resolved_boxes[1].pixel_rectangle,
            UserInterfacePixelRectangle {
                left: 300.0,
                top: 400.0,
                width: 200.0,
                height: 100.0,
            },
        );
    }

    #[test]
    fn horizontal_layout_glues_children_together_using_weights()
    {
        let mut layout = UserInterfaceLayout::new();
        let mut parent_box = UserInterfaceBox::new(UserInterfaceBoxLayout {
            children_layout: UserInterfaceChildrenLayout::Horizontal,
            ..Default::default()
        });
        let first_child = UserInterfaceBox::new(UserInterfaceBoxLayout {
            layout_weight: 1.0,
            ..Default::default()
        });
        let second_child = UserInterfaceBox::new(UserInterfaceBoxLayout {
            layout_weight: 2.0,
            ..Default::default()
        });
        parent_box.add_child(first_child);
        parent_box.add_child(second_child);
        layout.add_box(parent_box);

        let resolved_boxes = layout.resolve([900, 300]);

        assert_eq!(resolved_boxes.len(), 3);
        assert_eq!(
            resolved_boxes[1].pixel_rectangle,
            UserInterfacePixelRectangle {
                left: 0.0,
                top: 0.0,
                width: 300.0,
                height: 300.0,
            },
        );
        assert_eq!(
            resolved_boxes[2].pixel_rectangle,
            UserInterfacePixelRectangle {
                left: 300.0,
                top: 0.0,
                width: 600.0,
                height: 300.0,
            },
        );
    }

    #[test]
    fn higher_layers_draw_after_lower_layers()
    {
        let mut layout = UserInterfaceLayout::new();
        let mut overlay_box = UserInterfaceBox::new(UserInterfaceBoxLayout::default());
        overlay_box.set_layer(UserInterfaceLayer::OVERLAY);
        let mut background_box = UserInterfaceBox::new(UserInterfaceBoxLayout::default());
        background_box.set_layer(UserInterfaceLayer::BACKGROUND);
        layout.add_box(overlay_box);
        layout.add_box(background_box);

        let resolved_boxes = layout.resolve([100, 100]);

        assert_eq!(resolved_boxes[0].layer, UserInterfaceLayer::BACKGROUND);
        assert_eq!(resolved_boxes[1].layer, UserInterfaceLayer::OVERLAY);
    }

    #[test]
    fn zoom_scales_boxes_around_the_default_screen_centre()
    {
        let mut layout = UserInterfaceLayout::new();
        layout.set_zoom_factor(2.0);
        let user_interface_box = UserInterfaceBox::new(UserInterfaceBoxLayout {
            relative_bounds: UserInterfaceRelativeRectangle::new(0.25, 0.25, 0.5, 0.5),
            ..Default::default()
        });
        layout.add_box(user_interface_box);

        let resolved_boxes = layout.resolve([1_000, 1_000]);

        assert_eq!(
            resolved_boxes[0].pixel_rectangle,
            UserInterfacePixelRectangle {
                left: 0.0,
                top: 0.0,
                width: 1_000.0,
                height: 1_000.0,
            },
        );
    }

    #[test]
    fn invalid_zoom_factor_falls_back_to_a_valid_scale()
    {
        let mut layout = UserInterfaceLayout::new();
        layout.set_zoom_factor(0.0);

        assert_eq!(layout.zoom_factor(), MINIMUM_ZOOM_FACTOR);
    }

    #[test]
    fn list_constructor_uses_vertical_child_layout()
    {
        let list = UserInterfaceBox::new_list(UserInterfaceBoxLayout::default());

        assert_eq!(
            list.layout().children_layout,
            UserInterfaceChildrenLayout::Vertical,
        );
    }

    #[test]
    fn text_constructor_creates_a_transparent_text_box()
    {
        let text_box = UserInterfaceBox::new_text(UserInterfaceBoxLayout::default(), "Metrics");
        let mut layout = UserInterfaceLayout::new();
        layout.add_box(text_box);

        let resolved_boxes = layout.resolve([1_000, 1_000]);

        assert_eq!(resolved_boxes[0].text.as_deref(), Some("Metrics"));
        assert!(!resolved_boxes[0].is_background_visible);
    }

    #[test]
    fn container_and_panel_constructors_arrange_children_vertically()
    {
        let container = UserInterfaceBox::new_container(UserInterfaceBoxLayout::default());
        let panel = UserInterfaceBox::new_panel(UserInterfaceBoxLayout::default());

        assert_eq!(
            container.layout().children_layout,
            UserInterfaceChildrenLayout::Vertical,
        );
        assert!(!container.appearance().is_background_visible);
        assert_eq!(
            panel.layout().children_layout,
            UserInterfaceChildrenLayout::Vertical,
        );
        assert!(panel.appearance().is_background_visible);
    }
}
