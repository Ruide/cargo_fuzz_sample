#![no_main]
libfuzzer_sys::fuzz_target!(|color: color_conversion::Rgb| {
    let hsl = color.to_hsl();
    let rgb = hsl.to_rgb();

    // This should be true for all RGB -> HSL -> RGB conversions!
    assert_eq!(color, rgb);
});
