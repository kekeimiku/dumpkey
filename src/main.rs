use dumpkey::{bytes_to_usize, Process};

fn main() -> dumpkey::Result<()> {
    let pid = std::env::args().nth(1).ok_or(())?.parse()?;
    let process = Process::open(pid)?;
    let offset = process.get_offset()? as usize;

    // 105705c90 + 0 > 600001b04190 + 8 > 6000018910e0 + 16 > 600001891120 + 32 > 600003c7d160
    let mut buf = vec![0; 8];
    let offset = 0x105705c90 + offset;
    process.read_at_into(offset, buf.as_mut_slice())?;
    let offset = bytes_to_usize(&buf)? + 8;
    process.read_at_into(offset, buf.as_mut_slice())?;
    let offset = bytes_to_usize(&buf)? + 16;
    process.read_at_into(offset, buf.as_mut_slice())?;
    let offset = bytes_to_usize(&buf)? + 32;
    process.read_at_into(offset, buf.as_mut_slice())?;
    let offset = bytes_to_usize(&buf)?;

    let mut buf = vec![0; 32];
    process.read_at_into(offset, buf.as_mut_slice())?;
    println!("key = 0x{}", buf.iter().map(|x| format!("{x:02x}")).collect::<String>());

    Ok(())
}
