use rodio::{
    default_output_device, play_raw,
    source::{SineWave, Source},
};
use std::time::Duration;

fn main() {
    let device = default_output_device().unwrap();
    let source = SineWave::new(440);
    let short = source.take_duration(Duration::from_millis(1000));
    play_raw(&device, short.convert_samples());

    loop {}
}
