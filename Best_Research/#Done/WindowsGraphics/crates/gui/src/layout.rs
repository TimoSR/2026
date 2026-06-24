// public types
/// A position expressed as a fraction of the screen or an owning box.
pub type ScreenRelativePosition = [f32; 2];

/// A colour expressed as red, green, blue, and alpha components.
pub type UserInterfaceColor = [f32; 4];

/// A rectangle whose values are fractions of its parent rectangle.
///
/// Root boxes use the screen as their parent. Child boxes use their immediate
/// parent. A value of `1.0` represents the full corresponding parent dimension.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UserInterfaceRelativeRectangle
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
pub struct UserInterfacePixelRectangle
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
pub enum UserInterfaceChildrenLayout
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
pub struct UserInterfaceLayer
{
    order: i32,
}

/// The layout and child-arrangement settings of one user-interface box.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UserInterfaceBoxLayout
{
    /// Bounds relative to the screen for a root box or its parent for a child box.
    pub relative_bounds: UserInterfaceRelativeRectangle,
    /// Determines how direct child boxes are placed.
    pub children_layout: UserInterfaceChildrenLayout,
    /// Share of available space when this box belongs to a glued layout.
    pub layout_weight: f32,
}

/// The visual properties of one user-interface box.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UserInterfaceBoxAppearance
{
    /// Fill colour used to draw the box.
    pub background_color: UserInterfaceColor,
    /// Whether this box and all its children should be drawn.
    pub is_visible: bool,
}

/// A retained box that can own child boxes.
pub struct UserInterfaceBox
{
    layout: UserInterfaceBoxLayout,
    appearance: UserInterfaceBoxAppearance,
    layer: UserInterfaceLayer,
    children: Vec<UserInterfaceBox>,
}

/// A resolved, drawable box produced from a [`UserInterfaceLayout`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResolvedUserInterfaceBox
{
    /// The box bounds in the current viewport.
    pub pixel_rectangle: UserInterfacePixelRectangle,
    /// The box fill colour.
    pub background_color: UserInterfaceColor,
    /// The layer used to order drawing.
    pub layer: UserInterfaceLayer,
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
const MINIMUM_ZOOM_FACTOR: f32 = 0.01;
const DEFAULT_ZOOM_CENTRE: ScreenRelativePosition = [0.5, 0.5];
// domain constants

impl UserInterfaceRelativeRectangle
{
    /// The full area of the parent rectangle.
    pub const FULL: Self = Self {
        left: 0.0,
        top: 0.0,
        width: 1.0,
        height: 1.0,
    };

    /// Creates a rectangle relative to its parent.
    pub const fn new(left: f32, top: f32, width: f32, height: f32) -> Self
    {
        return Self {
            left,
            top,
            width,
            height,
        };
    }
}

impl UserInterfaceLayer
{
    /// The layer behind regular content.
    pub const BACKGROUND: Self = Self { order: 0 };
    /// The default content layer.
    pub const CONTENT: Self = Self { order: 100 };
    /// The layer for floating content such as tooltips.
    pub const OVERLAY: Self = Self { order: 200 };
    /// The layer for modal content that must be above other UI.
    pub const MODAL: Self = Self { order: 300 };

    /// Creates a layer with an explicit draw order.
    pub const fn new(order: i32) -> Self
    {
        return Self { order };
    }

    /// Returns the explicit draw order of this layer.
    pub const fn order(&self) -> i32
    {
        return self.order;
    }
}

impl Default for UserInterfaceBoxLayout
{
    fn default() -> Self
    {
        return Self {
            relative_bounds: UserInterfaceRelativeRectangle::FULL,
            children_layout: UserInterfaceChildrenLayout::Freeform,
            layout_weight: 1.0,
        };
    }
}

impl Default for UserInterfaceBoxAppearance
{
    fn default() -> Self
    {
        return Self {
            background_color: DEFAULT_BACKGROUND_COLOR,
            is_visible: true,
        };
    }
}

impl UserInterfaceBox
{
    /// Creates a visible box with default appearance on the content layer.
    pub fn new(layout: UserInterfaceBoxLayout) -> Self
    {
        return Self {
            layout,
            appearance: UserInterfaceBoxAppearance::default(),
            layer: UserInterfaceLayer::CONTENT,
            children: Vec::new(),
        };
    }

    /// Returns this box's placement and child-arrangement settings.
    pub fn layout(&self) -> UserInterfaceBoxLayout
    {
        return self.layout;
    }

    /// Replaces this box's placement and child-arrangement settings.
    pub fn set_layout(&mut self, layout: UserInterfaceBoxLayout)
    {
        self.layout = layout;
    }

    /// Returns this box's visual properties.
    pub fn appearance(&self) -> UserInterfaceBoxAppearance
    {
        return self.appearance;
    }

    /// Replaces this box's visual properties.
    pub fn set_appearance(&mut self, appearance: UserInterfaceBoxAppearance)
    {
        self.appearance = appearance;
    }

    /// Returns the layer used to draw this box.
    pub fn layer(&self) -> UserInterfaceLayer
    {
        return self.layer;
    }

    /// Changes the layer used to draw this box.
    pub fn set_layer(&mut self, layer: UserInterfaceLayer)
    {
        self.layer = layer;
    }

    /// Adds a child that will follow this box when layout is resolved.
    pub fn add_child(&mut self, child: UserInterfaceBox)
    {
        self.children.push(child);
    }

    /// Returns this box's direct children.
    pub fn children(&self) -> &[UserInterfaceBox]
    {
        return &self.children;
    }

    /// Returns mutable access to this box's direct children.
    pub fn children_mut(&mut self) -> &mut [UserInterfaceBox]
    {
        return &mut self.children;
    }
}

impl UserInterfaceLayout
{
    /// Creates an empty layout at normal scale.
    pub fn new() -> Self
    {
        return Self {
            root_boxes: Vec::new(),
            zoom_factor: 1.0,
            zoom_centre: DEFAULT_ZOOM_CENTRE,
        };
    }

    /// Adds a root box whose bounds are relative to the screen.
    pub fn add_box(&mut self, user_interface_box: UserInterfaceBox)
    {
        self.root_boxes.push(user_interface_box);
    }

    /// Returns the root boxes that make up this screen layout.
    pub fn boxes(&self) -> &[UserInterfaceBox]
    {
        return &self.root_boxes;
    }

    /// Returns mutable access to the root boxes that make up this screen layout.
    pub fn boxes_mut(&mut self) -> &mut [UserInterfaceBox]
    {
        return &mut self.root_boxes;
    }

    /// Returns the scale applied when resolving this layout.
    pub fn zoom_factor(&self) -> f32
    {
        return self.zoom_factor;
    }

    /// Changes the scale applied when resolving this layout.
    ///
    /// Non-finite and non-positive values are clamped to a small valid scale.
    pub fn set_zoom_factor(&mut self, zoom_factor: f32)
    {
        self.zoom_factor = valid_zoom_factor(zoom_factor);
    }

    /// Returns the screen-relative point around which zoom is applied.
    pub fn zoom_centre(&self) -> ScreenRelativePosition
    {
        return self.zoom_centre;
    }

    /// Changes the screen-relative point around which zoom is applied.
    pub fn set_zoom_centre(&mut self, zoom_centre: ScreenRelativePosition)
    {
        self.zoom_centre = zoom_centre;
    }

    /// Resolves all visible boxes into viewport pixels in drawing order.
    pub fn resolve(&self, viewport_pixel_size: [u32; 2]) -> Vec<ResolvedUserInterfaceBox>
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
            let box_rectangle = rectangle_relative_to_parent(
                user_interface_box.layout.relative_bounds,
                root_rectangle,
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

    let zoomed_rectangle = apply_zoom(
        box_rectangle,
        viewport_rectangle,
        zoom_factor,
        zoom_centre,
    );
    resolved_boxes.push(ResolvedUserInterfaceBox {
        pixel_rectangle: zoomed_rectangle,
        background_color: user_interface_box.appearance.background_color,
        layer: user_interface_box.layer,
    });

    match user_interface_box.layout.children_layout
    {
        UserInterfaceChildrenLayout::Freeform =>
        {
            for child_box in &user_interface_box.children
            {
                let child_rectangle = rectangle_relative_to_parent(
                    child_box.layout.relative_bounds,
                    box_rectangle,
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
                box_rectangle,
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
                box_rectangle,
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
}
