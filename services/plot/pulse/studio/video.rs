use super::resample::resample_indices;
use image::RgbaImage;
use std::io::Write;
use std::process::{Command, Stdio};

#[derive(Clone, Copy, Debug)]
pub(crate) enum VideoFormat {
    Mp4,
    WebM,
    Mov,
    Mkv,
    Avi,
}

const ALL_FORMAT_NAMES: [&str; 5] = ["mp4", "webm", "mov", "mkv", "avi"];

impl VideoFormat {
    pub(crate) fn parse(s: &str) -> Result<Self, String> {
        match s.to_ascii_lowercase().as_str() {
            "mp4" => Ok(VideoFormat::Mp4),
            "webm" => Ok(VideoFormat::WebM),
            "mov" => Ok(VideoFormat::Mov),
            "mkv" | "matroska" => Ok(VideoFormat::Mkv),
            "avi" => Ok(VideoFormat::Avi),
            other => Err(format!("unknown video format '{other}', expected one of: {}", ALL_FORMAT_NAMES.join(", "))),
        }
    }

    fn ffmpeg_container(self) -> &'static str {
        match self {
            VideoFormat::Mp4 => "mp4",
            VideoFormat::WebM => "webm",
            VideoFormat::Mov => "mov",
            VideoFormat::Mkv => "matroska",
            VideoFormat::Avi => "avi",
        }
    }

    fn ffmpeg_codec_args(self) -> &'static [&'static str] {
        match self {
            VideoFormat::Mp4 => &["-c:v", "libx264", "-pix_fmt", "yuv420p", "-preset", "medium", "-movflags", "+faststart"],
            VideoFormat::WebM => &["-c:v", "libvpx-vp9", "-pix_fmt", "yuv420p", "-b:v", "0", "-crf", "32"],
            VideoFormat::Mov => &["-c:v", "libx264", "-pix_fmt", "yuv420p", "-preset", "medium"],
            VideoFormat::Mkv => &["-c:v", "libx264", "-pix_fmt", "yuv420p", "-preset", "medium"],
            VideoFormat::Avi => &["-c:v", "libx264", "-pix_fmt", "yuv420p", "-preset", "medium"],
        }
    }
}

pub(crate) fn even(n: u32) -> u32 {
    if n % 2 == 1 {
        n + 1
    } else {
        n
    }
}

fn spawn_ffmpeg(width: u32, height: u32, fps: u32, format: VideoFormat, out_path: &str) -> Result<std::process::Child, String> {
    Command::new("ffmpeg")
        .args(["-y", "-f", "rawvideo", "-pixel_format", "rgba"])
        .args(["-video_size", &format!("{width}x{height}")])
        .args(["-framerate", &fps.to_string(), "-i", "-"])
        .args(format.ffmpeg_codec_args())
        .args(["-f", format.ffmpeg_container()])
        .arg(out_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                "ffmpeg was not found on PATH - install it from https://ffmpeg.org/download.html (or via your system package manager) to use video export".to_string()
            } else {
                format!("failed to launch ffmpeg: {e}")
            }
        })
}

pub(crate) fn encode_video(timed_frames: &[(u32, RgbaImage)], fps: u32, format: VideoFormat, out_path: &str) -> Result<(), String> {
    let (width, height) = timed_frames.first().map(|(_, img)| (img.width(), img.height())).ok_or_else(|| "no frames to encode".to_string())?;
    let timestamps: Vec<u32> = timed_frames.iter().map(|(dt, _)| *dt).collect();
    let order = resample_indices(&timestamps, fps);
    if order.is_empty() {
        return Err("no frames to encode".to_string());
    }

    let mut child = spawn_ffmpeg(width, height, fps, format, out_path)?;
    let mut stdin = child.stdin.take().expect("stdin was requested as piped");

    let output = std::thread::scope(|scope| -> Result<std::process::Output, String> {
        let writer = scope.spawn(move || -> std::io::Result<()> {
            for &idx in &order {
                stdin.write_all(timed_frames[idx].1.as_raw())?;
            }
            drop(stdin);
            Ok(())
        });
        let output = child.wait_with_output().map_err(|e| format!("failed while waiting for ffmpeg: {e}"))?;
        match writer.join() {
            Ok(Ok(())) => {}
            Ok(Err(e)) => return Err(format!("failed writing frames to ffmpeg: {e}")),
            Err(_) => return Err("the ffmpeg stdin writer thread panicked".to_string()),
        }
        Ok(output)
    })?;

    if !output.status.success() {
        return Err(format!("ffmpeg exited with an error:\n{}", String::from_utf8_lossy(&output.stderr)));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_accepts_every_known_format_case_insensitively() {
        assert!(matches!(VideoFormat::parse("mp4"), Ok(VideoFormat::Mp4)));
        assert!(matches!(VideoFormat::parse("MP4"), Ok(VideoFormat::Mp4)));
        assert!(matches!(VideoFormat::parse("webm"), Ok(VideoFormat::WebM)));
        assert!(matches!(VideoFormat::parse("WebM"), Ok(VideoFormat::WebM)));
        assert!(matches!(VideoFormat::parse("mov"), Ok(VideoFormat::Mov)));
        assert!(matches!(VideoFormat::parse("MOV"), Ok(VideoFormat::Mov)));
        assert!(matches!(VideoFormat::parse("mkv"), Ok(VideoFormat::Mkv)));
        assert!(matches!(VideoFormat::parse("matroska"), Ok(VideoFormat::Mkv)));
        assert!(matches!(VideoFormat::parse("avi"), Ok(VideoFormat::Avi)));
        assert!(matches!(VideoFormat::parse("AVI"), Ok(VideoFormat::Avi)));
    }

    #[test]
    fn parse_rejects_unknown_formats_with_a_clear_message_listing_every_valid_one() {
        let err = VideoFormat::parse("flv").unwrap_err();
        assert!(err.contains("flv"));
        for name in ALL_FORMAT_NAMES {
            assert!(err.contains(name), "error message should list '{name}' as a valid option, got: {err}");
        }
    }

    #[test]
    fn every_format_maps_to_a_distinct_ffmpeg_container() {
        let containers: std::collections::HashSet<&str> =
            [VideoFormat::Mp4, VideoFormat::WebM, VideoFormat::Mov, VideoFormat::Mkv, VideoFormat::Avi].iter().map(|f| f.ffmpeg_container()).collect();
        assert_eq!(containers.len(), 5, "every format should map to its own distinct ffmpeg container name");
    }

    #[test]
    fn mov_mkv_and_avi_all_use_h264_like_mp4_only_the_container_differs() {
        assert_eq!(VideoFormat::Mov.ffmpeg_codec_args(), VideoFormat::Mkv.ffmpeg_codec_args());
        assert_eq!(VideoFormat::Mkv.ffmpeg_codec_args(), VideoFormat::Avi.ffmpeg_codec_args());
        assert!(VideoFormat::Mov.ffmpeg_codec_args().contains(&"libx264"));
    }

    #[test]
    fn even_rounds_odd_numbers_up_and_leaves_even_numbers_alone() {
        assert_eq!(even(800), 800);
        assert_eq!(even(801), 802);
        assert_eq!(even(1), 2);
        assert_eq!(even(0), 0);
    }

    #[test]
    fn encode_video_rejects_an_empty_frame_list() {
        let err = encode_video(&[], 30, VideoFormat::Mp4, "unused.mp4").unwrap_err();
        assert!(err.contains("no frames"));
    }

    #[test]
    fn mp4_and_webm_use_different_codec_arguments() {
        assert_ne!(VideoFormat::Mp4.ffmpeg_codec_args(), VideoFormat::WebM.ffmpeg_codec_args());
    }
}
