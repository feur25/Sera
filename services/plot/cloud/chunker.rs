use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct CsvChunkReader {
    reader: BufReader<File>,
    pub chunk_rows: usize,
    pub header: Option<Vec<String>>,
    pub delimiter: u8,
    eof: bool,
}

impl CsvChunkReader {
    pub fn open<P: AsRef<Path>>(
        path: P,
        chunk_rows: usize,
        has_header: bool,
        delimiter: u8,
    ) -> std::io::Result<Self> {
        let f = File::open(path)?;
        let mut r = BufReader::with_capacity(1 << 20, f);
        let header = if has_header {
            let mut s = String::new();
            r.read_line(&mut s)?;
            Some(
                s.trim_end_matches(&['\n', '\r'][..])
                    .split(delimiter as char)
                    .map(|x| x.to_string())
                    .collect(),
            )
        } else {
            None
        };
        Ok(Self {
            reader: r,
            chunk_rows: chunk_rows.max(1),
            header,
            delimiter,
            eof: false,
        })
    }

    pub fn next_chunk(&mut self) -> std::io::Result<Option<Vec<Vec<f64>>>> {
        if self.eof {
            return Ok(None);
        }
        let mut out: Vec<Vec<f64>> = Vec::with_capacity(self.chunk_rows);
        let mut line = String::new();
        while out.len() < self.chunk_rows {
            line.clear();
            let n = self.reader.read_line(&mut line)?;
            if n == 0 {
                self.eof = true;
                break;
            }
            let trimmed = line.trim_end_matches(&['\n', '\r'][..]);
            if trimmed.is_empty() {
                continue;
            }
            let row: Vec<f64> = trimmed
                .split(self.delimiter as char)
                .map(|s| s.trim().parse::<f64>().unwrap_or(f64::NAN))
                .collect();
            out.push(row);
        }
        if out.is_empty() {
            Ok(None)
        } else {
            Ok(Some(out))
        }
    }
}

pub fn count_rows<P: AsRef<Path>>(path: P, has_header: bool) -> std::io::Result<usize> {
    let f = File::open(path)?;
    let r = BufReader::with_capacity(1 << 20, f);
    let mut n = 0usize;
    for _ in r.lines() {
        n += 1;
    }
    if has_header && n > 0 {
        Ok(n - 1)
    } else {
        Ok(n)
    }
}
