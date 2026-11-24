use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use web_sys::{AnalyserNode, AudioContext, HtmlAudioElement, MediaElementAudioSourceNode};

pub struct AudioController {
    audio: HtmlAudioElement,
    ctx: RefCell<Option<AudioContext>>,
    analyser: RefCell<Option<AnalyserNode>>,
    _src: RefCell<Option<MediaElementAudioSourceNode>>,
    buf: RefCell<Vec<u8>>,
    out: RefCell<Vec<f32>>,
}

impl AudioController {
    pub fn new(audio: HtmlAudioElement) -> Rc<Self> {
        Rc::new(Self {
            audio,
            ctx: RefCell::new(None),
            analyser: RefCell::new(None),
            _src: RefCell::new(None),
            buf: RefCell::new(vec![0u8; 64]),
            out: RefCell::new(Vec::new()),
        })
    }

    pub fn ensure_audio_graph(&self) -> Result<(), JsValue> {
        if self.ctx.borrow().is_some() {
            return Ok(());
        }
        let ctx = AudioContext::new()?;
        let src = ctx.create_media_element_source(&self.audio)?;
        let analyser = ctx.create_analyser()?;
        analyser.set_fft_size(64);
        analyser.set_smoothing_time_constant(0.72);
        src.connect_with_audio_node(&analyser)?;
        analyser.connect_with_audio_node(&ctx.destination())?;
        *self.ctx.borrow_mut() = Some(ctx);
        *self.analyser.borrow_mut() = Some(analyser);
        *self._src.borrow_mut() = Some(src);
        Ok(())
    }

    pub fn play(&self) -> Result<(), JsValue> {
        self.ensure_audio_graph()?;
        if let Some(ctx) = self.ctx.borrow().as_ref() {
            ctx.resume().ok();
        }
        self.audio.play()?;
        Ok(())
    }

    pub fn pause(&self) {
        self.audio.pause().ok();
    }

    pub fn is_playing(&self) -> bool {
        !self.audio.paused()
    }

    pub fn elapsed(&self) -> f64 {
        self.audio.current_time()
    }

    pub fn duration(&self) -> f64 {
        let d = self.audio.duration();
        if d.is_finite() {
            d
        } else {
            0.0
        }
    }

    pub fn sample_spectrum(&self) -> Vec<f32> {
        let mut out = self.out.borrow_mut();
        out.clear();
        let Some(analyser) = self.analyser.borrow().as_ref().cloned() else {
            return Vec::new();
        };
        if !self.is_playing() {
            return Vec::new();
        }
        let n = (analyser.frequency_bin_count() as usize / 2).min(32);
        if n == 0 {
            return Vec::new();
        }
        let mut buf = self.buf.borrow_mut();
        if buf.len() < n {
            buf.resize(n, 0);
        }
        analyser.get_byte_frequency_data(&mut buf[..n]).ok();
        out.reserve(n);
        for &v in &buf[..n] {
            out.push(v as f32 / 255.0);
        }
        out.clone()
    }

    pub fn audio(&self) -> &HtmlAudioElement {
        &self.audio
    }
}
