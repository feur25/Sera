use crate::plot::canvas_points::{b64_encode, pack_indices_u32_raw};

pub fn atomic_write(path: &str, contents: impl AsRef<[u8]>) -> std::io::Result<()> {
    let tmp_path = format!("{path}.tmp");
    std::fs::write(&tmp_path, contents)?;
    std::fs::rename(&tmp_path, path)
}

pub fn build_binary_frame(hid: u64, idx_raw: &[u8], pts_raw: &[u8], n: u32, vpp: u32) -> Vec<u8> {
    let mut frame = Vec::with_capacity(12 + idx_raw.len() + pts_raw.len());
    frame.extend_from_slice(&(hid as u32).to_le_bytes());
    frame.extend_from_slice(&n.to_le_bytes());
    frame.extend_from_slice(&vpp.to_le_bytes());
    frame.extend_from_slice(idx_raw);
    frame.extend_from_slice(pts_raw);
    frame
}

pub fn encode_replay_frames(records: &[super::record_registry::Record]) -> String {
    let mut frames = String::with_capacity(records.len() * 96 + 2);
    frames.push('[');
    let mut last_dt: u32 = 0;
    for (i, rec) in records.iter().enumerate() {
        if i > 0 {
            frames.push(',');
        }
        let idx_raw = pack_indices_u32_raw(&rec.indices);
        let mut pts_raw = Vec::with_capacity(rec.values.len() * 2);
        for v in &rec.values {
            pts_raw.extend_from_slice(&v.to_le_bytes());
        }
        let wait_ms = rec.dt_ms.saturating_sub(last_dt);
        last_dt = rec.dt_ms;
        frames.push_str(&format!(
            "{{\"a\":\"{}\",\"b\":\"{}\",\"w\":{}}}",
            b64_encode(&idx_raw),
            b64_encode(&pts_raw),
            wait_ms,
        ));
    }
    frames.push(']');
    frames
}

pub fn build_replay_script(hid: u64, records: &[super::record_registry::Record]) -> String {
    let frames = encode_replay_frames(records);
    format!(
        "<script>(function(){{var F={frames};var k=0;function step(){{if(k>=F.length)return;var f=F[k];var fn=window['sp_push_{hid}'];if(fn)fn(f.a,f.b);k++;if(k<F.length)setTimeout(step,F[k].w);}}if(F.length)setTimeout(step,F[0].w);}})();</script></body>"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_binary_frame_lays_out_hid_n_vpp_then_idx_then_pts() {
        let frame = build_binary_frame(7, &[1, 2], &[3, 4, 5], 9, 2);
        assert_eq!(&frame[0..4], &7u32.to_le_bytes());
        assert_eq!(&frame[4..8], &9u32.to_le_bytes());
        assert_eq!(&frame[8..12], &2u32.to_le_bytes());
        assert_eq!(&frame[12..14], &[1, 2]);
        assert_eq!(&frame[14..17], &[3, 4, 5]);
    }

    #[test]
    fn empty_records_encode_to_an_empty_frame_array() {
        assert_eq!(encode_replay_frames(&[]), "[]");
    }

    #[test]
    fn replay_script_references_the_given_hid() {
        let script = build_replay_script(42, &[]);
        assert!(script.contains("sp_push_42"));
    }
}
