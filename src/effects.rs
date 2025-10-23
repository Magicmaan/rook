use ratatui::{buffer::Buffer, layout::Rect, style::Color};
use tachyonfx::{Duration, EffectManager, fx, pattern::SweepPattern};

pub fn rainbow(
    start_color: Color,
    timer: u32,
    speed: f32,
    area: Rect,
    buf: &mut Buffer,
    tick: u32,
) {
    let fg_shift = [1440.0, 0.0, 0.0];
    let mut timer = timer;
    let mut effects: EffectManager<()> = EffectManager::default();
    let mut fx_rainbow = fx::hsl_shift_fg(fg_shift, timer)
        .with_pattern(SweepPattern::left_to_right(area.width as u16 * 2))
        .with_area(area);
    // cut start end portion for infinite rainbow
    fx_rainbow = fx::remap_alpha(0.333, 0.666, fx_rainbow);

    let fx_in = fx::repeating(fx_rainbow.clone());

    let t = (tick * (speed * 10.0) as u32) % timer;
    // if fade in and out, adjust timer and create fade effects

    let mut fx = fx_rainbow;

    fx = fx::repeat(fx::sequence(&[fx_in]), fx::RepeatMode::Forever);

    effects.add_effect(fx);
    effects.process_effects(Duration::from_millis((t)), buf, area);
}
