use super::*;

/// `h`: Add horizontal spacing.
///
/// # Positional parameters
/// - Amount of spacing: of type `linear` relative to current font size.
///
/// # Return value
/// A template that adds horizontal spacing.
pub fn h(ctx: &mut EvalContext, args: &mut ValueArgs) -> Value {
    spacing(ctx, args, SpecAxis::Horizontal)
}

/// `v`: Add vertical spacing.
///
/// # Positional parameters
/// - Amount of spacing: of type `linear` relative to current font size.
///
/// # Return value
/// A template that adds vertical spacing.
pub fn v(ctx: &mut EvalContext, args: &mut ValueArgs) -> Value {
    spacing(ctx, args, SpecAxis::Vertical)
}

/// Apply spacing along a specific axis.
fn spacing(ctx: &mut EvalContext, args: &mut ValueArgs, axis: SpecAxis) -> Value {
    let spacing: Option<Linear> = args.require(ctx, "spacing");
    Value::template("spacing", move |ctx| {
        if let Some(linear) = spacing {
            let amount = linear.resolve(ctx.state.font.font_size());
            let spacing = NodeSpacing { amount, softness: 0 };
            if axis == ctx.state.dirs.main.axis() {
                ctx.push_into_stack(spacing);
            } else {
                ctx.push(spacing);
            }
        }
    })
}
