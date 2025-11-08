use hound::{SampleFormat, WavReader};
use nih_plug::prelude::*;
use rubato::{FftFixedIn, Resampler};
use std::path::PathBuf;

use super::Result;

#[derive(Debug, Default)]
pub struct ImpulseResponse {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub file: Option<PathBuf>,
}

impl ImpulseResponse {
    const RESAMPLER_CHUNKS: usize = 16;

    pub fn load<F>(&mut self, file: F) -> Result<()>
    where
        F: Into<PathBuf> + Clone,
    {
        let mut reader = WavReader::open(file.clone().into().as_path())?;

        let normalizer = 1.0 / (1_u64 << (reader.spec().bits_per_sample - 1)) as f32;

        self.samples = match reader.spec().sample_format {
            SampleFormat::Float => reader.samples::<f32>().map(|s| s.unwrap()).collect(),
            SampleFormat::Int => reader
                .samples::<i32>()
                .map(|s| s.unwrap() as f32 * normalizer)
                .collect(),
        };
        self.sample_rate = reader.spec().sample_rate;
        self.file = Some(file.clone().into());

        nih_log!(
            "IR loaded from {} {}",
            file.into().to_string_lossy(),
            self.sample_rate
        );

        Ok(())
    }

    pub fn resample(&mut self, sample_rate: u32) -> Result<()> {
        if sample_rate == self.sample_rate {
            return Ok(());
        }

        let mut resampler = FftFixedIn::<f32>::new(
            self.sample_rate.try_into()?,
            sample_rate.try_into()?,
            self.samples.len(),
            Self::RESAMPLER_CHUNKS,
            1,
        )?;

        let mut resampled_samples = resampler.process(&vec![self.samples.clone()], None)?;
        resampled_samples[0].drain(0..resampler.output_delay());

        self.samples = resampled_samples[0].clone();
        self.sample_rate = sample_rate;

        nih_log!("IR resampled {} {}", self.samples.len(), self.sample_rate);

        Ok(())
    }
}
