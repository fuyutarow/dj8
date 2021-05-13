use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use synthrs::synthesizer::{make_samples_from_midi_file, quantize};

use cli::audio_env::build_stream;
use cli::audio_env::get_audio_env;
use cli::note::Pitch;

fn main() -> anyhow::Result<()> {
    let (host, device, supported_config) = get_audio_env()?;
    let config = cpal::StreamConfig::from(supported_config.clone());

    let sample_sec = 5.0;
    let sample_rate = config.sample_rate.0 as f32;
    let samples = synthrs::synthesizer::make_samples(
        sample_sec,
        config.sample_rate.0 as usize,
        synthrs::wave::sine_wave(Pitch::C4.to_freq()),
    );

    let stream = build_stream(device, supported_config, samples)?;
    stream.play()?;

    timer();
    Ok(())
}

fn timer() {
    let mut time = 0;
    loop {
        println!("hello @ {}", time);
        std::thread::sleep(std::time::Duration::from_millis(1000));
        time += 1
    }
}
