use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use synthrs::synthesizer::{make_samples_from_midi_file, quantize};

use cli::audio_env::get_audio_env;

fn main() -> anyhow::Result<()> {
    let (host, device, config) = get_audio_env()?;
    dbg!(&config);
    match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into()),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into()),
    }
}

pub fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
where
    T: cpal::Sample,
{
    let channels = config.channels as usize;
    let sample_rate = config.sample_rate.0 as f32;
    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let samples = synthrs::synthesizer::make_samples(
        1.0,
        config.sample_rate.0 as usize,
        synthrs::wave::sine_wave(440.0),
    )
    .into_iter()
    .map(|f| f as f32)
    .collect::<Vec<_>>();

    let mut sample_clock = 0f32;
    let mut next_sample = samples.into_iter();
    // let mut next_value = move || next_sample.next().unwrap();
    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
    };

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &mut next_value)
        },
        err_fn,
    )?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(1000));

    Ok(())
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: cpal::Sample,
{
    for frame in output.chunks_mut(channels) {
        let value: T = cpal::Sample::from::<f32>(&next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
