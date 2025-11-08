use std::path::PathBuf;
use std::sync::RwLockReadGuard;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, IRLoaderError>;

#[derive(Error, Debug)]
pub enum IRLoaderError {
    #[error("Wav processing error {0}")]
    WavError(#[from] hound::Error),
    #[error("Try from int error {0}")]
    TryFromIntError(#[from] std::num::TryFromIntError),
    #[error("Resample error {0}")]
    ResampleError(#[from] rubato::ResampleError),
    #[error("Resampler construction error {0}")]
    ResamplerConstructionError(#[from] rubato::ResamplerConstructionError),
    #[error("FFT convolver init error {0}")]
    FFTConvolverInitError(#[from] fft_convolver::FFTConvolverInitError),
    #[error("FFT convolver process error {0}")]
    FFTConvolverProcessError(#[from] fft_convolver::FFTConvolverProcessError),
    #[error("Sync poison error {0}")]
    PoisonError(#[from] std::sync::PoisonError<RwLockReadGuard<'static, Option<PathBuf>>>),
    #[error("IR samples empty")]
    SamplesEmpty,
}
