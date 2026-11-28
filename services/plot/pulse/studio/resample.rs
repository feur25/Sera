pub(crate) fn resample_indices(timestamps_ms: &[u32], fps: u32) -> Vec<usize> {
    if timestamps_ms.is_empty() {
        return Vec::new();
    }
    let fps = fps.max(1);
    let duration_ms = *timestamps_ms.last().unwrap();
    let frame_interval_ms = 1000.0 / fps as f64;
    let n_out = ((duration_ms as f64 / frame_interval_ms).floor() as usize) + 1;

    let mut out = Vec::with_capacity(n_out);
    let mut src_idx = 0usize;
    for i in 0..n_out {
        let t = i as f64 * frame_interval_ms;
        while src_idx + 1 < timestamps_ms.len() && (timestamps_ms[src_idx + 1] as f64) <= t {
            src_idx += 1;
        }
        out.push(src_idx);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_produces_no_output_frames() {
        assert_eq!(resample_indices(&[], 30), Vec::<usize>::new());
    }

    #[test]
    fn a_single_source_frame_always_produces_at_least_one_output_frame() {
        assert_eq!(resample_indices(&[0], 30), vec![0]);
        assert_eq!(resample_indices(&[0], 1), vec![0]);
    }

    #[test]
    fn output_frame_count_matches_duration_times_fps() {
        let out = resample_indices(&[0, 1000], 10);
        assert_eq!(out.len(), 11, "1 second at 10fps should produce 11 samples (t=0,100,...,1000ms)");
    }

    #[test]
    fn each_output_frame_maps_to_the_latest_source_frame_at_or_before_its_time() {
        let timestamps = [0, 500, 1000];
        let out = resample_indices(&timestamps, 2);
        assert_eq!(out, vec![0, 1, 2], "at 2fps (500ms spacing), each output lands exactly on a source timestamp");
    }

    #[test]
    fn a_burst_of_updates_within_one_output_frame_interval_only_shows_the_latest() {
        let timestamps = [0, 10, 20, 30, 3000];
        let out = resample_indices(&timestamps, 1);
        assert_eq!(out, vec![0, 3, 3, 4], "the burst (indices 1-3, all under 30ms) must compress into a single sample showing the latest of the burst (index 3), repeat through the quiet gap, then advance to the final update");
    }

    #[test]
    fn a_gap_with_no_updates_repeats_the_last_known_frame() {
        let timestamps = [0, 3000];
        let out = resample_indices(&timestamps, 1);
        assert_eq!(out, vec![0, 0, 0, 1], "the source frame at t=0 should repeat for every output sample until the next real update at t=3000ms");
    }

    #[test]
    fn resampled_indices_are_always_non_decreasing() {
        let timestamps = [0, 15, 40, 41, 42, 900, 2000];
        let out = resample_indices(&timestamps, 24);
        for w in out.windows(2) {
            assert!(w[1] >= w[0], "resampled source indices must never go backwards in time");
        }
    }

    #[test]
    fn higher_fps_never_produces_fewer_output_frames_than_lower_fps() {
        let timestamps = [0, 200, 700, 1500];
        let low = resample_indices(&timestamps, 5);
        let high = resample_indices(&timestamps, 60);
        assert!(high.len() >= low.len());
    }
}
