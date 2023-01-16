use dumpkey::Process;

fn main(){
    let pid = std::env::args().nth(1).unwrap().parse().unwrap();
    let mem = Process::open(pid).unwrap();
    let offset = mem.get_offset().unwrap() as usize;

    // 105705c90 + 0 > 600001b04190 + 8 > 6000018910e0 + 16 > 600001891120 + 32 > 600003c7d160
    let mut buf = vec![0; 8];
    let addr = 0x105705c90 + offset;
    mem.read_at_into(addr, buf.as_mut_slice()).unwrap();
    let addr = usize::from_ne_bytes(buf.clone().try_into().unwrap());
    mem.read_at_into(addr + 8, buf.as_mut_slice()).unwrap();
    let addr = usize::from_ne_bytes(buf.clone().try_into().unwrap());
    mem.read_at_into(addr + 16, buf.as_mut_slice()).unwrap();
    let addr = usize::from_ne_bytes(buf.clone().try_into().unwrap());
    mem.read_at_into(addr + 32, buf.as_mut_slice()).unwrap();
    let addr = usize::from_ne_bytes(buf.clone().try_into().unwrap());

    let mut buf = vec![0; 32];
    mem.read_at_into(addr, buf.as_mut_slice()).unwrap();
    println!("key = 0x{}", buf.iter().map(|x| format!("{x:02x}")).collect::<String>());
}
