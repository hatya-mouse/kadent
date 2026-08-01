use crate::background_thread::DecodedAudio;
use std::path::Path;

pub(super) fn run_decode_wav(path: &Path) -> hound::Result<DecodedAudio> {
    let mut reader = hound::WavReader::open(path)?;
    let spec = reader.spec();

    let data: Vec<f32> = match spec.sample_format {
        hound::SampleFormat::Float => reader.samples::<f32>().collect::<hound::Result<_>>()?,
        hound::SampleFormat::Int => {
            let max_amplitude = (1i64 << (spec.bits_per_sample - 1)) as f32;
            reader
                .samples::<i32>()
                .map(|s| s.map(|v| v as f32 / max_amplitude))
                .collect::<hound::Result<_>>()?
        }
    };

    let frames = data.len() / spec.channels as usize;
    Ok(DecodedAudio {
        data,
        frames,
        sample_rate: spec.sample_rate,
        channels: spec.channels,
    })
}
