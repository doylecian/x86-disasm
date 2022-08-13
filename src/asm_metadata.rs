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


pub static TARGET_MACHINE_TYPES: phf::Map<u16, &'static str> = phf_map! {
    0x0u16 => "IMAGE_FILE_MACHINE_UNKNOWN",
    0x1d3u16 => "IMAGE_FILE_MACHINE_AM33",
    0x8664u16=> "IMAGE_FILE_MACHINE_AMD64",
    0x1c0u16 => "IMAGE_FILE_MACHINE_ARM",
    0xaa64u16 => "IMAGE_FILE_MACHINE_ARM64",
    0x1c4u16 => "IMAGE_FILE_MACHINE_ARMNT",
    0xebcu16 => "IMAGE_FILE_MACHINE_EBC",
    0x14cu16 => "IMAGE_FILE_MACHINE_I386",
    0x200u16 => "IMAGE_FILE_MACHINE_IA64",
    0x6232u16 => "IMAGE_FILE_MACHINE_LOONGARCH32",
    0x6264u16 => "IMAGE_FILE_MACHINE_LOONGARCH64",
    0x9041u16 => "IMAGE_FILE_MACHINE_M32R",
    0x266u16 => "IMAGE_FILE_MACHINE_MIPS16",
    0x366u16 => "IMAGE_FILE_MACHINE_MIPSFPU",
    0x466u16 => "IMAGE_FILE_MACHINE_MIPSFPU16",
    0x1f0u16 => "IMAGE_FILE_MACHINE_POWERPC",
    0x1f1u16 => "IMAGE_FILE_MACHINE_POWERPCFP",
    0x166u16 => "IMAGE_FILE_MACHINE_R4000",
    0x5032u16 => "IMAGE_FILE_MACHINE_RISCV32",
    0x5064u16 => "IMAGE_FILE_MACHINE_RISCV64",
    0x5128u16 => "IMAGE_FILE_MACHINE_RISCV128",
    0x1a2u16 => "IMAGE_FILE_MACHINE_SH3",
    0x1a3u16 => "IMAGE_FILE_MACHINE_SH3DSP",
    0x1a6u16 => "IMAGE_FILE_MACHINE_SH4",
    0x1a8u16 => "IMAGE_FILE_MACHINE_SH5",
    0x1c2u16 => "IMAGE_FILE_MACHINE_THUMB",
    0x169u16 => "IMAGE_FILE_MACHINE_WCEMIPSV2"
};