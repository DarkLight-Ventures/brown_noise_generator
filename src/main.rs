extern crate rand;
extern crate hound;

use rand::Rng;
use std::fs::File;
use std::i16;
use hound::WavWriter;

fn main() {
    let duration_secs = 10;
    let sample_rate = 44100;
    let output_file = "brown_noise.wav";

    generate_brown_noise(duration_secs, sample_rate, output_file);
}

fn generate_brown_noise(duration_secs: u32, sample_rate: u32, output_file: &str) {
    let num_samples = duration_secs * sample_rate;

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = WavWriter::create(output_file, spec).unwrap();
    let mut rng = rand::thread_rng();
    let mut current_sample = 0.0;

    for _ in 0..num_samples {
        let white_noise = rng.gen_range(-1.0..1.0);
        current_sample += white_noise;

        // Normalize the sample to prevent overflow
        current_sample *= 0.5;

        let sample_int = (current_sample * i16::MAX as f32).round() as i16;
        writer.write_sample(sample_int).unwrap();
    }

    writer.finalize().unwrap();
}
