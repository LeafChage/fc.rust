extern crate anyhow;
extern crate binary;

mod cpu;
mod ines;
mod memory;
mod ppu;
mod program;
mod result;

use std::fs;
use std::io::Read;

use std::cell::RefCell;
use std::rc::Rc;

use result::Result;

fn main() -> Result<()> {
    let mut f = fs::File::open("./resource/sample1.nes")?;
    let length = f.metadata()?.len() as usize;
    let mut data = vec![0; length];
    let n = f.read(&mut data)?;
    let data = &data[..n];

    let ines = ines::INes::parse(data)?;
    // let sprites = parser.sprites(&header)?;
    // dbg!(sprites.len());
    // ines::debug::create(sprites);

    let mut ppu_register = Rc::new(RefCell::new(ppu::Register::default()));
    let mut ppu_memory = ppu::MemoryMap::default();
    let mut ppu = ppu::PPU::new(Rc::clone(&ppu_register), ppu_memory);
    let mut cpu_register = cpu::Register::default();
    let mut cpu_memory = cpu::MemoryMap::new(Rc::clone(&ppu_register), ines.program());
    let mut cpu = cpu::CPU::new(cpu_register, cpu_memory);

    println!("{}", cpu);
    for i in 0..1000 {
        cpu.exec()?;
        ppu.exec()?;
        println!("{}", cpu);
        let _ = std::io::stdin().read_line(&mut String::new());
    }

    // show_binary_with(&data[..n], 16);
    Ok(())
}
