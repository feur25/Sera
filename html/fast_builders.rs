pub struct SvgBuilder {
    buf: Vec<u8>,
    width: f32,
    height: f32,
    max_bars: usize,
}

impl SvgBuilder {
    #[inline(always)]
    pub fn new(width: f32, height: f32, capacity: usize) -> Self {
        let mut buf = Vec::with_capacity(capacity * 90 + 256);
        buf.extend_from_slice(b"<svg viewBox=\"0 0 1200 600\" xmlns=\"http://www.w3.org/2000/svg\"><rect width=\"1200\" height=\"600\" fill=\"#fff\"/>");
        
        Self {
            buf,
            width,
            height,
            max_bars: 0,
        }
    }

    #[inline(always)]
    pub fn add_bar_fast(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32) {
        let _r = ((color >> 16) & 0xFF) as u8;
        let _g = ((color >> 8) & 0xFF) as u8;
        let _b = (color & 0xFF) as u8;
        
        let mut hex = [b'0'; 6];
        Self::encode_hex(color, &mut hex);
        
        self.buf.extend_from_slice(b"<rect x=\"");
        Self::encode_int(x, &mut self.buf);
        self.buf.extend_from_slice(b"\" y=\"");
        Self::encode_int(y, &mut self.buf);
        self.buf.extend_from_slice(b"\" width=\"");
        Self::encode_int(w, &mut self.buf);
        self.buf.extend_from_slice(b"\" height=\"");
        Self::encode_int(h, &mut self.buf);
        self.buf.extend_from_slice(b"\" fill=\"#");
        self.buf.extend_from_slice(&hex);
        self.buf.extend_from_slice(b"\"/>");
        
        self.max_bars += 1;
    }

    #[inline(always)]
    pub fn add_bar(&mut self, x: f32, y: f32, w: f32, h: f32, color: u32) {
        self.add_bar_fast(x as i32, y as i32, w as i32, h as i32, color);
    }

    #[inline(always)]
    pub fn add_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, color: u32, width: f32) {
        let mut hex = [b'0'; 6];
        Self::encode_hex(color, &mut hex);
        
        self.buf.extend_from_slice(b"<line x1=\"");
        Self::encode_int(x1 as i32, &mut self.buf);
        self.buf.extend_from_slice(b"\" y1=\"");
        Self::encode_int(y1 as i32, &mut self.buf);
        self.buf.extend_from_slice(b"\" x2=\"");
        Self::encode_int(x2 as i32, &mut self.buf);
        self.buf.extend_from_slice(b"\" y2=\"");
        Self::encode_int(y2 as i32, &mut self.buf);
        self.buf.extend_from_slice(b"\" stroke=\"#");
        self.buf.extend_from_slice(&hex);
        self.buf.extend_from_slice(b"\" stroke-width=\"");
        Self::encode_int(width as i32, &mut self.buf);
        self.buf.extend_from_slice(b"\"/>");
        
        self.max_bars += 1;
    }

    #[inline(always)]
    fn encode_hex(color: u32, hex: &mut [u8; 6]) {
        let h = b"0123456789abcdef";
        hex[0] = h[((color >> 20) & 0xF) as usize];
        hex[1] = h[((color >> 16) & 0xF) as usize];
        hex[2] = h[((color >> 12) & 0xF) as usize];
        hex[3] = h[((color >> 8) & 0xF) as usize];
        hex[4] = h[((color >> 4) & 0xF) as usize];
        hex[5] = h[(color & 0xF) as usize];
    }

    #[inline(always)]
    fn encode_int(mut num: i32, buf: &mut Vec<u8>) {
        if num < 0 {
            buf.push(b'-');
            num = -num;
        }
        
        if num == 0 {
            buf.push(b'0');
            return;
        }
        
        let mut digits = [0u8; 10];
        let mut len = 0;
        let mut n = num;
        while n > 0 {
            digits[len] = (n % 10) as u8 + b'0';
            n /= 10;
            len += 1;
        }
        
        for &d in digits[..len].iter().rev() {
            buf.push(d);
        }
    }

    #[inline(always)]
    pub fn finish(mut self) -> String {
        self.buf.extend_from_slice(b"</svg>");
        String::from_utf8(self.buf).unwrap_or_default()
    }

    #[inline(always)]
    pub fn get_svg(&self) -> String {
        let mut result = String::from_utf8_lossy(&self.buf).to_string();
        result.push_str("</svg>");
        result
    }
}

pub struct HtmlBuilder {
    buf: Vec<u8>,
    capacity: usize,
}

impl HtmlBuilder {
    #[inline(always)]
    pub fn new(title: &str, svg: &str, json: &str) -> Self {
        let capacity = title.len() + svg.len() + json.len() + 3000;
        let mut buf = Vec::with_capacity(capacity);
        
        buf.extend_from_slice(b"<!DOCTYPE html><html><head><meta charset=UTF-8><title>");
        buf.extend_from_slice(title.as_bytes());
        buf.extend_from_slice(b"</title><style>*{margin:0;padding:0;box-sizing:border-box}body{background:#f5f5f5;font:12px sans-serif}.chart-container{width:100%;height:100vh;display:flex;align-items:center;justify-content:center;background:#fff;position:relative;overflow:hidden}svg{width:90%;height:90%;max-width:1200px;max-height:600px;display:block}</style></head><body><div class=chart-container>");
        buf.extend_from_slice(svg.as_bytes());
        buf.extend_from_slice(b"</div><script>window.__SERAPLOT_STATE__=");
        buf.extend_from_slice(json.as_bytes());
        buf.extend_from_slice(b"</script></body></html>");
        
        Self { buf, capacity }
    }

    #[inline(always)]
    pub fn build(self) -> String {
        String::from_utf8(self.buf).unwrap_or_default()
    }
}
