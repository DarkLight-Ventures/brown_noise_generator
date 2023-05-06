use rand::Rng;
use std::fs::File;
use std::i16;
use wav::{BitDepth, Header};

fn main() {
    let duration_secs = 10;
    let sample_rate = 44100;
    let output_file = "brown_noise.wav";

    generate_brown_noise(duration_secs, sample_rate, output_file);
}

fn generate_brown_noise(duration_secs: u32, sample_rate: u32, output_file: &str) {
    let num_samples = duration_secs * sample_rate;
    let mut rng = rand::thread_rng();

    let mut brown_noise = Vec::with_capacity(num_samples as usize);
    let mut current_sample = 0.0;

    for _ in 0..num_samples {
        let white_noise = rng.gen_range(-1.0..1.0);
        current_sample += white_noise;

        // Normalize the sample to prevent overflow
        current_sample *= 0.5;

        let sample_int = (current_sample * i16::MAX as f32).round() as i16;
        brown_noise.push(sample_int);
    }

    let header = Header::new(1, 1, sample_rate, BitDepth::Sixteen);
    let mut writer = wav::Writer::new(File::create(output_file).unwrap(), header).unwrap();
    writer.write_data(&brown_noise).unwrap();
}
