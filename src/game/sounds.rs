pub fn play_sound(path: &str, volume: f64) {
    let result = web_sys::HtmlAudioElement::new_with_src(path).unwrap();
    result.set_volume(volume.clamp(0.0, 5.0));
    let _ = result.play();
}
