use rodio::{
    default_output_device, play_raw,
    source::{SineWave, Source},
};
use std::time::Duration;

fn main() {
    let device = default_output_device().unwrap();
    let source1 = SineWave::new(300);
    let source2 = SineWave::new(600);
    let short = source1.take_duration(Duration::from_millis(1000));
    play_raw(&device, short.convert_samples());
    let short = source2.take_duration(Duration::from_millis(1000));
    play_raw(&device, short.convert_samples());

    loop {}
}
