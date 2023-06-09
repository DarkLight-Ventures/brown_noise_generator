extern crate rand;
extern crate hound;
extern crate biquad;


use biquad::*;
use clap::Parser;
use hound::WavWriter;
use rand::Rng;
use std::i16;
use std::f32::consts::PI;

#[derive(Clone, Debug, Parser)]
#[clap(name = "Brown Noise Generator", version = "1.0", author = "Kelsea Blackwell")]
struct Opts {
    #[clap(short, long, default_value = "60")]
    duration_secs: u32,

    #[clap(short, long, default_value = "44100")]
    sample_rate: u32,

    #[clap(short, long, default_value = "brown_noise.wav")]
    output_file: String,

    #[clap(short, long, default_value = "900")]
    cutoff_frequency: f32,
}

fn main() {
    let opts = Opts::parse();

    let brown_noise_samples = generate_white_noise(opts.duration_secs, opts.sample_rate);
    let filtered_samples = apply_low_pass_filter(&brown_noise_samples, opts.cutoff_frequency, opts.sample_rate);
    let warbled_samples_15 = apply_warble_effect(&filtered_samples, 0.15, opts.sample_rate, 0.5, 0);
    let warbled_samples_30 = apply_warble_effect(&warbled_samples_15, 0.15, opts.sample_rate, 0.5, (opts.sample_rate / 2).try_into().unwrap());

    let mixed_warble = mix_wav_samples(&warbled_samples_15, &warbled_samples_30, 0.5);
    let mixed_sample = mix_wav_samples(&mixed_warble, &filtered_samples, 0.5);

    write_wav_samples(&opts.output_file, opts.sample_rate, &mixed_sample);
}


fn apply_low_pass_filter(samples: &[i16], cutoff_frequency: f32, sample_rate: u32) -> Vec<i16> {
    let mut filtered_samples = vec![0i16; samples.len()];

    let f0 = cutoff_frequency.hz();
    let fs = sample_rate.hz();
    let coeffs = Coefficients::<f32>::from_params(Type::LowPass, fs, f0, Q_BUTTERWORTH_F32).unwrap();

    let mut low_pass_filter = DirectForm1::<f32>::new(coeffs);

    for (input_sample, output_sample) in samples.iter().zip(filtered_samples.iter_mut()) {
        *output_sample = low_pass_filter.run(*input_sample as f32) as i16;
    }

    filtered_samples
}


fn apply_warble_effect(samples: &[i16], lfo_frequency: f32, sample_rate: u32, depth: f32, offset: usize) -> Vec<i16> {
    let mut warbled_samples = vec![0i16; samples.len()];
    let lfo_increment = 2.0 * PI * lfo_frequency / (sample_rate as f32);

    let mut lfo_phase: f32 = 0.0;

    for (i, (input_sample, output_sample)) in samples.iter().zip(warbled_samples.iter_mut()).enumerate() {
        if i >= offset {
            let lfo = (lfo_phase.sin() + 1.0) * 0.5 * depth + (1.0 - depth);
            *output_sample = (*input_sample as f32 * lfo) as i16;
            lfo_phase += lfo_increment;
        } else {
            *output_sample = *input_sample;
        }
    }

    warbled_samples
}


fn generate_white_noise(duration_secs: u32, sample_rate: u32) -> Vec<i16> {
    let num_samples = duration_secs * sample_rate;
    let mut rng = rand::thread_rng();

    let mut white_noise = Vec::with_capacity(num_samples as usize);
    let mut current_sample = 0.0;

    for _ in 0..num_samples {
        let noise = rng.gen_range(-1.0..1.0);
        current_sample += noise;

        // Normalize the sample to prevent overflow
        current_sample *= 0.5;

        let sample_int = (current_sample * i16::MAX as f32).round() as i16;
        white_noise.push(sample_int);
    }

    white_noise
}


fn mix_wav_samples(samples1: &[i16], samples2: &[i16], blend_factor: f32) -> Vec<i16> {
    let length = samples1.len().max(samples2.len());
    let mut mixed_samples = vec![0i16; length];

    for i in 0..length {
        let sample1 = if i < samples1.len() {
            samples1[i] as f32
        } else {
            0.0
        };

        let sample2 = if i < samples2.len() {
            samples2[i] as f32
        } else {
            0.0
        };

        mixed_samples[i] = (sample1 * (1.0 - blend_factor) + sample2 * blend_factor) as i16;
    }

    mixed_samples
}


fn write_wav_samples(output_file: &str, sample_rate: u32, samples: &[i16]) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = WavWriter::create(output_file, spec).unwrap();

    for &sample in samples {
        writer.write_sample(sample).unwrap();
    }

    writer.finalize().unwrap();
}
