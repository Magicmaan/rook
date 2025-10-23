use ratatui::{buffer::Buffer, layout::Rect};
use tachyonfx::{Duration, EffectManager, fx, pattern::SweepPattern};

pub fn rainbow(area: Rect, buf: &mut Buffer, tick: u32) {
    let fg_shift = [1440.0, 0.0, 0.0];
    let timer = 2000;
    let mut effects: EffectManager<()> = EffectManager::default();
    let mut fx = fx::hsl_shift_fg(fg_shift, timer)
        .with_pattern(SweepPattern::left_to_right(area.width as u16 * 2))
        .with_area(area);

    fx = fx::repeating(fx::parallel(&[fx::remap_alpha(0.333, 0.666, fx)]));
    effects.add_effect(fx);
    effects.process_effects(Duration::from_millis(((tick) * 10) % (timer)), buf, area);
}
