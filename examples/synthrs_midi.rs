use synthrs::midi;
use synthrs::sample;
use synthrs::synthesizer::{make_samples_from_midi, make_samples_from_midi_file, quantize_samples};
use synthrs::wave;
use synthrs::writer::write_wav_file;

fn main() {
    // `make_samples_from_midi_file` is a convenience function that parses and synthesises
    // a MIDI file given a file path
    // Set `use_envelope` to decide whether to use a basic attack/decay envelope when generating samples
    // The envelope will slowly fade each note out over time
    let mm: () = &make_samples_from_midi_file(
        wave::square_wave,
        44_100,
        true,
        "examples/assets/octave.mid",
    )
    .unwrap();
    dbg!(&mm);
    let ss = &quantize_samples::<i16>(mm);

    // dbg!(ss);
}
