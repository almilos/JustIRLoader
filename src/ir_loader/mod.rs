use fft_convolver::FFTConvolver;
use nih_plug::prelude::*;
use nih_plug_egui::EguiState;
use std::{
    path::PathBuf,
    sync::{
        Arc, RwLock,
        atomic::{AtomicBool, Ordering},
    },
};

mod error;
mod ir;

pub(crate) use error::*;

use ir::ImpulseResponse;

pub struct IRLoader {
    pub params: Arc<IRLoaderParams>,

    pub impulse_response: ImpulseResponse,
    pub convolver: Vec<(FFTConvolver<f32>, Vec<f32>)>,

    pub sample_rate: u32,
    pub nchannels: u8,
}

#[derive(Params)]
pub struct IRLoaderParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<EguiState>,

    pub ir_file: RwLock<Option<PathBuf>>,
    pub need_reload: AtomicBool,
}

impl IRLoaderParams {
    pub fn set_need_reload(&self, val: bool) {
        self.need_reload.store(val, Ordering::Relaxed);
    }

    pub fn get_need_reload(&self) -> bool {
        self.need_reload.load(Ordering::Relaxed)
    }

    pub fn set_ir_file<F>(&self, file: F)
    where
        F: Into<PathBuf> + Clone,
    {
        let mut ir_file = self.ir_file.write().unwrap();
        *ir_file = Some(file.into());
        self.set_need_reload(true);
    }

    pub fn get_ir_file(&self) -> Option<PathBuf> {
        self.ir_file.read().unwrap().clone()
    }
}

impl IRLoader {
    pub const SIZE: usize = 8;

    pub fn init(&mut self, sample_rate: u32, nchannels: u8) {
        self.sample_rate = sample_rate;
        self.nchannels = nchannels;
        self.params.set_need_reload(true);
    }

    pub fn load_ir(&mut self) -> Result<()> {
        let file = self.params.get_ir_file();
        if file.is_none() {
            return Ok(());
        }

        self.impulse_response.load(file.unwrap())?;
        self.impulse_response.resample(self.sample_rate)?;

        self.convolver = vec![];

        for _ in 0..self.nchannels {
            let mut convolver = FFTConvolver::default();
            convolver.init(IRLoader::SIZE, &self.impulse_response.samples)?;

            self.convolver
                .push((convolver, vec![0 as f32; IRLoader::SIZE]));

            nih_log!(
                "Convolver ready {:?} {:?}",
                self.sample_rate,
                self.nchannels
            );
        }

        self.params.set_need_reload(false);

        Ok(())
    }

    pub fn process(&mut self, channels: &mut [&mut [f32]]) -> Result<()> {
        if self.impulse_response.samples.is_empty() {
            return Err(IRLoaderError::SamplesEmpty);
        }

        for (samples, convolver) in channels.iter_mut().zip(self.convolver.iter_mut()) {
            convolver.0.process(samples, &mut convolver.1)?;
            samples.copy_from_slice(&convolver.1);
        }

        Ok(())
    }

    pub fn reset(&mut self) {
        for convolver in self.convolver.iter_mut() {
            convolver.0.reset();
        }
    }
}

impl Default for IRLoader {
    fn default() -> Self {
        Self {
            params: Arc::new(IRLoaderParams::default()),

            impulse_response: ImpulseResponse::default(),
            sample_rate: 0,
            nchannels: 0,
            convolver: vec![],
        }
    }
}

impl Default for IRLoaderParams {
    fn default() -> Self {
        Self {
            editor_state: EguiState::from_size(300, 100),
            ir_file: RwLock::new(None),
            need_reload: AtomicBool::new(false),
        }
    }
}
