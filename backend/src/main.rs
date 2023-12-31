use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::str::FromStr;

#[allow(non_upper_case_globals)]

#[cfg(target_os = "linux")]
static newline: &str = "\r\n";

#[cfg(target_os = "windows")]
static newline: &str = "\n";

fn preprocess(message: &[u8]) -> Vec<u8> {
    let message_length: u64 = message.len() as u64 * 8;
    let mut result = message.to_owned();

    // atleast one value must be added
    result.push(0x80);

    while ((result.len() * 8) + 64) % 512 != 0 {
        result.push(0);
    }

    // u64 -> little endian bytes
    for b in 0..8 {
        result.push((message_length >> (b * 8)) as u8);
    }

    result
}

fn md4(input: &[u8]) -> String {
    let mut a0 = 0x67452301u32;
    let mut b0 = 0xefcdab89u32;
    let mut c0 = 0x98badcfeu32;
    let mut d0 = 0x10325476u32;

    let preprocessed_message = preprocess(input);

    for chunk in preprocessed_message.chunks(64) {
        // Little endian here too
        let m: Vec<u32> = chunk
            .chunks(4)
            .map(|int32_bytes| {
                ((int32_bytes[3] as u32) << 24)
                    | ((int32_bytes[2] as u32) << 16)
                    | ((int32_bytes[1] as u32) << 8)
                    | ((int32_bytes[0] as u32) << 0)
            })
            .collect();

        let mut a = a0;
        let mut b = b0;
        let mut c = c0;
        let mut d = d0;

        a = (a.wrapping_add((b & c) | (!b) & d).wrapping_add(m[0])).rotate_left(3);
        d = (d.wrapping_add((a & b) | (!a) & c).wrapping_add(m[1])).rotate_left(7);
        c = (c.wrapping_add((d & a) | (!d) & b).wrapping_add(m[2])).rotate_left(11);
        b = (b.wrapping_add((c & d) | (!c) & a).wrapping_add(m[3])).rotate_left(19);
        a = (a.wrapping_add((b & c) | (!b) & d).wrapping_add(m[4])).rotate_left(3);
        d = (d.wrapping_add((a & b) | (!a) & c).wrapping_add(m[5])).rotate_left(7);
        c = (c.wrapping_add((d & a) | (!d) & b).wrapping_add(m[6])).rotate_left(11);
        b = (b.wrapping_add((c & d) | (!c) & a).wrapping_add(m[7])).rotate_left(19);
        a = (a.wrapping_add((b & c) | (!b) & d).wrapping_add(m[8])).rotate_left(3);
        d = (d.wrapping_add((a & b) | (!a) & c).wrapping_add(m[9])).rotate_left(7);
        c = (c.wrapping_add((d & a) | (!d) & b).wrapping_add(m[10])).rotate_left(11);
        b = (b.wrapping_add((c & d) | (!c) & a).wrapping_add(m[11])).rotate_left(19);
        a = (a.wrapping_add((b & c) | (!b) & d).wrapping_add(m[12])).rotate_left(3);
        d = (d.wrapping_add((a & b) | (!a) & c).wrapping_add(m[13])).rotate_left(7);
        c = (c.wrapping_add((d & a) | (!d) & b).wrapping_add(m[14])).rotate_left(11);
        b = (b.wrapping_add((c & d) | (!c) & a).wrapping_add(m[15])).rotate_left(19);

        a = (a
            .wrapping_add((b & c) | (b & d) | (c & d))
            .wrapping_add(m[0].wrapping_add(0x5a827999)))
        .rotate_left(3);
        d = (d
            .wrapping_add((a & b) | (a & c) | (b & c))
            .wrapping_add(m[4].wrapping_add(0x5a827999)))
        .rotate_left(5);
        c = (c
            .wrapping_add((d & a) | (d & b) | (a & b))
            .wrapping_add(m[8].wrapping_add(0x5a827999)))
        .rotate_left(9);
        b = (b
            .wrapping_add((c & d) | (c & a) | (d & a))
            .wrapping_add(m[12].wrapping_add(0x5a827999)))
        .rotate_left(13);
        a = (a
            .wrapping_add((b & c) | (b & d) | (c & d))
            .wrapping_add(m[1].wrapping_add(0x5a827999)))
        .rotate_left(3);
        d = (d
            .wrapping_add((a & b) | (a & c) | (b & c))
            .wrapping_add(m[5].wrapping_add(0x5a827999)))
        .rotate_left(5);
        c = (c
            .wrapping_add((d & a) | (d & b) | (a & b))
            .wrapping_add(m[9].wrapping_add(0x5a827999)))
        .rotate_left(9);
        b = (b
            .wrapping_add((c & d) | (c & a) | (d & a))
            .wrapping_add(m[13].wrapping_add(0x5a827999)))
        .rotate_left(13);
        a = (a
            .wrapping_add((b & c) | (b & d) | (c & d))
            .wrapping_add(m[2].wrapping_add(0x5a827999)))
        .rotate_left(3);
        d = (d
            .wrapping_add((a & b) | (a & c) | (b & c))
            .wrapping_add(m[6].wrapping_add(0x5a827999)))
        .rotate_left(5);
        c = (c
            .wrapping_add((d & a) | (d & b) | (a & b))
            .wrapping_add(m[10].wrapping_add(0x5a827999)))
        .rotate_left(9);
        b = (b
            .wrapping_add((c & d) | (c & a) | (d & a))
            .wrapping_add(m[14].wrapping_add(0x5a827999)))
        .rotate_left(13);
        a = (a
            .wrapping_add((b & c) | (b & d) | (c & d))
            .wrapping_add(m[3].wrapping_add(0x5a827999)))
        .rotate_left(3);
        d = (d
            .wrapping_add((a & b) | (a & c) | (b & c))
            .wrapping_add(m[7].wrapping_add(0x5a827999)))
        .rotate_left(5);
        c = (c
            .wrapping_add((d & a) | (d & b) | (a & b))
            .wrapping_add(m[11].wrapping_add(0x5a827999)))
        .rotate_left(9);
        b = (b
            .wrapping_add((c & d) | (c & a) | (d & a))
            .wrapping_add(m[15].wrapping_add(0x5a827999)))
        .rotate_left(13);

        a = (a
            .wrapping_add(b ^ c ^ d)
            .wrapping_add(m[0].wrapping_add(0x6ed9eba1)))
        .rotate_left(3);
        d = (d
            .wrapping_add(a ^ b ^ c)
            .wrapping_add(m[8].wrapping_add(0x6ed9eba1)))
        .rotate_left(9);
        c = (c
            .wrapping_add(d ^ a ^ b)
            .wrapping_add(m[4].wrapping_add(0x6ed9eba1)))
        .rotate_left(11);
        b = (b
            .wrapping_add(c ^ d ^ a)
            .wrapping_add(m[12].wrapping_add(0x6ed9eba1)))
        .rotate_left(15);
        a = (a
            .wrapping_add(b ^ c ^ d)
            .wrapping_add(m[2].wrapping_add(0x6ed9eba1)))
        .rotate_left(3);
        d = (d
            .wrapping_add(a ^ b ^ c)
            .wrapping_add(m[10].wrapping_add(0x6ed9eba1)))
        .rotate_left(9);
        c = (c
            .wrapping_add(d ^ a ^ b)
            .wrapping_add(m[6].wrapping_add(0x6ed9eba1)))
        .rotate_left(11);
        b = (b
            .wrapping_add(c ^ d ^ a)
            .wrapping_add(m[14].wrapping_add(0x6ed9eba1)))
        .rotate_left(15);
        a = (a
            .wrapping_add(b ^ c ^ d)
            .wrapping_add(m[1].wrapping_add(0x6ed9eba1)))
        .rotate_left(3);
        d = (d
            .wrapping_add(a ^ b ^ c)
            .wrapping_add(m[9].wrapping_add(0x6ed9eba1)))
        .rotate_left(9);
        c = (c
            .wrapping_add(d ^ a ^ b)
            .wrapping_add(m[5].wrapping_add(0x6ed9eba1)))
        .rotate_left(11);
        b = (b
            .wrapping_add(c ^ d ^ a)
            .wrapping_add(m[13].wrapping_add(0x6ed9eba1)))
        .rotate_left(15);
        a = (a
            .wrapping_add(b ^ c ^ d)
            .wrapping_add(m[3].wrapping_add(0x6ed9eba1)))
        .rotate_left(3);
        d = (d
            .wrapping_add(a ^ b ^ c)
            .wrapping_add(m[11].wrapping_add(0x6ed9eba1)))
        .rotate_left(9);
        c = (c
            .wrapping_add(d ^ a ^ b)
            .wrapping_add(m[7].wrapping_add(0x6ed9eba1)))
        .rotate_left(11);
        b = (b
            .wrapping_add(c ^ d ^ a)
            .wrapping_add(m[15].wrapping_add(0x6ed9eba1)))
        .rotate_left(15);

        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
    }

    let mut result = String::new();

    for v in &[a0, b0, c0, d0] {
        result.push_str(&format!(
            "{:02x}{:02x}{:02x}{:02x}",
            (v >> 0) as u8,
            (v >> 8) as u8,
            (v >> 16) as u8,
            (v >> 24) as u8
        ));
    }

    result
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("One of input/output file is missing in arguments");
    } else if args.len() > 3 {
        panic!("More number of arguments passed");
    }
    let input_file = &args[1];
    let output_file = &args[2];

    let file = File::open(input_file.clone()).expect("Failed to open input file");

    let outfile = OpenOptions::new()
        .read(false)
        .create(true)
        .write(true)
        .open(output_file.clone())
        .expect("Failed to open output file");

    let mut bufout = BufWriter::with_capacity(1024 * 8, outfile);
    let bufreader = BufReader::new(file);

    for line in bufreader.lines() {
        let con = line.expect("Failed to read string");
        let output = md4(con.as_bytes());

        let mut msg = String::from_str(&format!("{} : ", con)).expect("Failed to write output");
        msg.push_str(&output);
        msg.push_str(&newline);

        let _res = bufout.write_all(msg.as_bytes());
        // don't handle Result
        bufout.flush().expect("Failed to write into output file");
    }
}
