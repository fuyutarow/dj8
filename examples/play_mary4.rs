use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use synthrs::synthesizer::{make_samples_from_midi_file, quantize};

use dj8::audio_env::build_stream;
use dj8::audio_env::get_audio_env;
use dj8::note::{Note, Pitch};
use dj8::score::Stem;

fn main() -> anyhow::Result<()> {
    let (host, device, supported_config) = get_audio_env()?;
    let config = cpal::StreamConfig::from(supported_config.clone());

    let sample_rate = config.sample_rate.0;
    let bpm = 121.;
    let secs_per_beat = 60. / bpm;
    let c_major = Stem::join_from_abc("C,4E,4G,4");
    let f_major = Stem::join_from_abc("F,4A,4C4");

    let stem = Stem::Cat(vec![
        Stem::Join(vec![Stem::cat_from_abc("AGFG"), f_major.clone()]),
        Stem::Join(vec![Stem::cat_from_abc("AAA2"), f_major.clone()]),
        Stem::Join(vec![Stem::cat_from_abc("GGG2"), c_major.clone()]),
        Stem::Join(vec![Stem::cat_from_abc("Acc2"), f_major.clone()]),
        Stem::Join(vec![Stem::cat_from_abc("AGFG"), f_major.clone()]),
        Stem::Join(vec![Stem::cat_from_abc("AAAA"), f_major.clone()]),
        Stem::Join(vec![Stem::cat_from_abc("GGAG"), c_major.clone()]),
        Stem::Join(vec![Stem::cat_from_abc("F4"), f_major.clone()]),
    ]);

    let samples_per_tick = (secs_per_beat * sample_rate as f64 / 256.).floor() as usize;
    let samples = stem.to_samples(samples_per_tick);
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
