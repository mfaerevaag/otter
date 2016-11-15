use std::io::Write;

#[macro_export]
macro_rules! log_msg {
    ($fmt:expr) => {{
        print!(concat!("\r", $fmt, "\n> "));
        io::stdout().flush().unwrap();
    }};
    ($fmt:expr, $($arg:tt)*) => {{
        print!(concat!("\r", $fmt, "\n> "), $($arg)*);
        io::stdout().flush().unwrap();
    }};
}

pub fn hexdump<W: Write>(out: &mut W, buf: &[u8]) {
    out.write_all(b"    ").unwrap();

    for (i, &v) in buf.iter().enumerate() {
        let output = format!("{:02X} ", v);
        out.write_all(output.as_bytes()).unwrap();
        match i % 16 {
            15 => { out.write_all(b"\n    ").unwrap(); },
            7 =>  { out.write_all(b"   ").unwrap(); },
            _ => ()
        }

        out.flush().unwrap();
    }

    out.write_all(b"\n").unwrap();
}
