use crate::background_thread::DecodedAudio;
use std::path::PathBuf;

pub(super) fn run_decode_wav(path: PathBuf) -> hound::Result<DecodedAudio> {
    let reader = hound::WavReader::open(&path)?;
    let spec = reader.spec();
    let frames = reader.len() as usize / spec.channels as usize;

    Ok(DecodedAudio {
        path,
        frames,
        sample_rate: spec.sample_rate as u64,
    })
}
