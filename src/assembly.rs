// MIT License

// Copyright (c) 2021 AnonmousDapper

global_asm!(include_str!("asm/boot.s"));
global_asm!(include_str!("asm/trap.s"));
global_asm!(include_str!("asm/mem.s"));
