use std::fs::File;
use std::io::BufReader;
use std::time::{Duration, Instant};

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};

pub struct AudioPlayer {
    _stream: Option<OutputStream>,
    stream_handle: Option<OutputStreamHandle>,
    sink: Option<Sink>,
    start_instant: Option<Instant>,
    total_elapsed: Duration,
    paused: bool,
    loaded_path: Option<String>,
    duration: Option<Duration>,
}

impl AudioPlayer {
    pub fn new() -> Result<Self, String> {
        match OutputStream::try_default() {
            Ok((stream, handle)) => match Sink::try_new(&handle) {
                Ok(sink) => Ok(AudioPlayer {
                    _stream: Some(stream),
                    stream_handle: Some(handle),
                    sink: Some(sink),
                    start_instant: None,
                    total_elapsed: Duration::ZERO,
                    paused: true,
                    loaded_path: None,
                    duration: None,
                }),
                Err(e) => Err(format!("Failed to create sink: {}", e)),
            },
            Err(_) => Ok(AudioPlayer::dummy()),
        }
    }

    fn dummy() -> Self {
        AudioPlayer {
            _stream: None,
            stream_handle: None,
            sink: None,
            start_instant: None,
            total_elapsed: Duration::ZERO,
            paused: true,
            loaded_path: None,
            duration: None,
        }
    }

    pub fn load(&mut self, path: &str) -> Result<(), String> {
        if self.sink.is_none() {
            self.loaded_path = Some(path.to_string());
            return Ok(());
        }
        if let Some(sink) = &self.sink {
            sink.stop();
        }
        self.loaded_path = Some(path.to_string());
        self.start_instant = None;
        self.total_elapsed = Duration::ZERO;
        self.paused = true;
        self.duration = None;
        let file = File::open(path).map_err(|e| format!("open file error: {}", e))?;
        let reader = BufReader::new(file);
        let decoder = Decoder::new(reader).map_err(|e| format!("decoder error: {:?}", e))?;
        if let Some(dur) = decoder.total_duration() {
            self.duration = Some(dur);
        }
        if let Some(sink) = &self.sink {
            let file = File::open(path).map_err(|e| format!("open file error: {}", e))?;
            let reader = BufReader::new(file);
            let decoder = Decoder::new(reader).map_err(|e| format!("decoder error: {:?}", e))?;
            sink.append(decoder);
        }
        Ok(())
    }

    pub fn play(&mut self) {
        if self.loaded_path.is_none() {
            return;
        }
        if self.paused {
            self.start_instant = Some(Instant::now());
            self.paused = false;
        }
        if let Some(sink) = &self.sink {
            sink.play();
        }
    }

    pub fn pause(&mut self) {
        if !self.paused {
            if let Some(start) = self.start_instant {
                self.total_elapsed += start.elapsed();
            }
            self.start_instant = None;
            self.paused = true;
            if let Some(sink) = &self.sink {
                sink.pause();
            }
        }
    }

    pub fn resume(&mut self) {
        if self.paused {
            self.start_instant = Some(Instant::now());
            self.paused = false;
            if let Some(sink) = &self.sink {
                sink.play();
            }
        }
    }

    pub fn is_playing(&self) -> bool {
        !self.paused && self.start_instant.is_some()
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded_path.is_some()
    }

    pub fn elapsed_ms(&self) -> u64 {
        let mut elapsed = self.total_elapsed;
        if let Some(start) = self.start_instant {
            elapsed += start.elapsed();
        }
        elapsed.as_millis() as u64
    }
}