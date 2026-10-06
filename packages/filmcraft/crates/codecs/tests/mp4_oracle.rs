//! MP4 audio against ffmpeg: ffmpeg writes an AAC track (with the edit list that skips the encoder
//! priming), and reads at another sample rate must land on the same source samples as the native
//! decode, however the requests are cut.

mod common;

use common::*;

/// A source whose rate differs from the sequence's is resampled per request. An AAC track in MP4
/// carries an edit list that skips the encoder priming (ffmpeg: 1024 samples), and every request
/// must still land on the same source samples: reading at 44.1 kHz in video-frame-sized pieces (as
/// an export does) equals the 48 kHz decode interpolated at the same times.
#[test]
fn aac_mp4_resampled_reads_follow_the_edit_list() {
    let ff = filmcraft_testkit::require_ffmpeg!();
    let Some(f) = fixture(&ff, "tone_aac_48k.mp4", &["-f", "lavfi", "-i", "sine=frequency=440:sample_rate=48000:d=3", "-c:a", "aac", "-b:a", "128k"]) else {
        panic!("could not write the AAC fixture")
    };
    let src = filmcraft_codecs::open_bytes("tone_aac_48k.mp4", bytes(&f)).unwrap();
    let native = src.audio(0, 2 * 48_000, 48_000).unwrap().channels[0].clone();
    let ratio = 48_000.0 / 44_100.0;
    // 30 fps at 44.1 kHz, starting mid-file and off the 48 kHz grid
    let (mut start, piece) = (22_051i64, 1470usize);
    let (mut worst, mut peak) = (0f32, 0f32);
    for _ in 0..20 {
        let got = src.audio(start, piece, 44_100).unwrap();
        for (k, &v) in got.channels[0].iter().enumerate() {
            let pos = (start + k as i64) as f64 * ratio;
            let i0 = pos.floor() as usize;
            let want = native[i0] + (native[i0 + 1] - native[i0]) * (pos - i0 as f64) as f32;
            worst = worst.max((v - want).abs());
            peak = peak.max(v.abs());
        }
        start += piece as i64;
    }
    let level = native.iter().fold(0f32, |m, v| m.max(v.abs()));
    assert!(peak <= level * 1.01, "resampled peak {peak} above the source's {level}");
    assert!(worst < 1e-4, "resampled read differs from the source by {worst}");
}
