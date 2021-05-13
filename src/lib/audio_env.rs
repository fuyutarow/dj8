use cpal::platform::{Device, Host};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::SupportedStreamConfig;

#[derive(Debug)]
struct Opt {
    #[cfg(all(
        any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd"),
        feature = "jack"
    ))]
    jack: bool,

    device: String,
}

impl Opt {
    fn from_args() -> Self {
        let device = {
            let app = clap::App::new("beep").arg_from_usage("[DEVICE] 'The audio device to use'");
            #[cfg(all(
                any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd"),
                feature = "jack"
            ))]
            let app = app.arg_from_usage("-j, --jack 'Use the JACK host");
            let matches = app.get_matches();
            let device = matches.value_of("DEVICE").unwrap_or("default").to_string();
            device
        };

        #[cfg(all(
            any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd"),
            feature = "jack"
        ))]
        return Opt {
            jack: matches.is_present("jack"),
            device,
        };

        #[cfg(any(
            not(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd")),
            not(feature = "jack")
        ))]
        Opt { device }
    }
}

pub fn get_audio_env() -> anyhow::Result<((Host, Device, SupportedStreamConfig))> {
    let opt = Opt::from_args();

    let host = {
        // Conditionally compile with jack if the feature is specified.
        #[cfg(all(
            any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd"),
            feature = "jack"
        ))]
        // Manually check for flags. Can be passed through cargo with -- e.g.
        // cargo run --release --example beep --features jack -- --jack
        let host = if opt.jack {
            cpal::host_from_id(cpal::available_hosts()
            .into_iter()
            .find(|id| *id == cpal::HostId::Jack)
            .expect(
                "make sure --features jack is specified. only works on OSes where jack is available",
            )).expect("jack host unavailable")
        } else {
            cpal::default_host()
        };

        #[cfg(any(
            not(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd")),
            not(feature = "jack")
        ))]
        let host = cpal::default_host();
        host
    };

    let device = {
        let device = if opt.device == "default" {
            host.default_output_device()
        } else {
            host.output_devices()?
                .find(|x| x.name().map(|y| y == opt.device).unwrap_or(false))
        }
        .expect("failed to find output device");
        println!("Output device: {}", device.name()?);
        device
    };

    let config = {
        let config = device.default_output_config().unwrap();
        println!("Default output config: {:?}", config);
        config
    };

    Ok((host, device, config))
}

pub fn build_stream(
    device: cpal::Device,
    supported_config: cpal::SupportedStreamConfig,
    samples: Vec<f64>, // next_sample: &mut dyn FnMut() -> f32,
) -> anyhow::Result<cpal::Stream> {
    let stream = match supported_config.sample_format() {
        cpal::SampleFormat::F32 => {
            build_stream_helper::<f32>(&device, &supported_config.into(), samples)
        }
        cpal::SampleFormat::I16 => {
            build_stream_helper::<i16>(&device, &supported_config.into(), samples)
        }
        cpal::SampleFormat::U16 => {
            build_stream_helper::<u16>(&device, &supported_config.into(), samples)
        }
    }?;
    Ok(stream)
}

fn build_stream_helper<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    samples: Vec<f64>,
) -> anyhow::Result<cpal::Stream>
where
    T: cpal::Sample,
{
    let mut next_value = samples.into_iter().map(|f| f as f32);
    let mut next_sample = move || next_value.next().unwrap();

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
    let channels = config.channels as usize;
    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &mut next_sample)
        },
        err_fn,
    )?;
    Ok(stream)
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
