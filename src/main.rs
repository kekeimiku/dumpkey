use dumpkey::{bytes_to_usize, Process};

fn main() -> dumpkey::Result<()> {
    let pid = std::env::args().nth(1).ok_or(())?.parse()?;
    let proc = Process::open(pid)?;
    let offset = proc.get_offset()? as usize;

    // 0x105705c90 + 0 > 0x600001460ae0 + 8 > 0x60000170c560 + 16 > 0x60000170c5a0 + 64 > 6000026ac5e0
    let mut buf = vec![0; 8];
    let offset = 0x105705c90 + offset;
    proc.read_at_into(offset, buf.as_mut_slice())?;
    let offset = bytes_to_usize(&buf)? + 8;
    proc.read_at_into(offset, buf.as_mut_slice())?;
    let offset = bytes_to_usize(&buf)? + 16;
    proc.read_at_into(offset, buf.as_mut_slice())?;
    let offset = bytes_to_usize(&buf)? + 64;

    println!("{:#x}",offset);
    let mut buf = vec![0; 32];
    proc.read_at_into(offset, buf.as_mut_slice())?;

    println!("key = 0x{}", buf.iter().map(|x| format!("{x:02x}")).collect::<String>());

    Ok(())
}
