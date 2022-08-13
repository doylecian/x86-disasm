use phf::phf_map;

pub static _OPCODES: phf::Map<u8, &'static str> = phf_map! {
    0xE9u8 => "jmp",
    0x90u8 => "nop",
    0x50u8 => "push eax,",
    0x51u8 => "push ecx",
    0x52u8 => "push edx",
    0x53u8 => "push ebx",
    0x54u8 => "push esp",
    0x55u8 => "push ebp",
    0x56u8 => "push esi",
    0x57u8 => "push edi",
    0x58u8 => "pop eax",
    0x59u8 => "push ecx",
    0x5Au8 => "push edx",
    0x5Bu8 => "push ebx",
    0x5Cu8 => "push esp",
    0x5Du8 => "push ebp",
    0x5Eu8 => "push esi",
    0x5Fu8 => "push edi",
};


#[cfg(test)]
mod tests {
    use test::{Bencher, black_box};

    use super::_OPCODES;

    #[bench]
    fn bench_phf_hash_map(b: &mut Bencher) {
        b.iter(|| {
            for _i in  0..0xd9000 {
                black_box(_OPCODES.get(&(0x59 as u8)));
            }
        });
    }
}