mod cpu;
mod display;
mod ines;
mod memory;
mod ppu;
mod program;
mod result;
mod sprite;
mod x;

use clap::Parser;
use result::Result;

use std::fs;
use std::io::Read;

use std::cell::RefCell;
use std::rc::Rc;

use macroquad::prelude as quad;
use macroquad::prelude::Conf;

fn window_conf() -> Conf {
    Conf {
        window_title: "FC".to_owned(),
        fullscreen: false,
        window_height: display::H as i32,
        window_width: display::W as i32,
        ..Default::default()
    }
}

#[derive(Parser)]
#[command()]
struct CLI {
    #[arg(long)]
    debug: bool,

    #[arg(short)]
    nes: std::path::PathBuf,
}

#[macroquad::main(window_conf)]
async fn main() -> Result<()> {
    let cli = CLI::parse();

    let mut f = fs::File::open(&cli.nes)?;
    let length = f.metadata()?.len() as usize;
    let mut data = vec![0; length];
    let n = f.read(&mut data)?;
    let data = &data[..n];

    let ines = ines::INes::parse(data)?;
    let display = Rc::new(RefCell::new(display::Display::default()));

    let ppu_register = RefCell::new(ppu::Register::default());
    let ppu_memory = ppu::MemoryMap::default();
    let ppu = Rc::new(RefCell::new(ppu::PPU::new(
        ppu_register,
        ppu_memory,
        Rc::clone(&display),
        ines.sprites(),
    )));

    let wram = vec![0; 0x2000];

    let cpu_register = cpu::Register::default();
    let cpu_memory = cpu::MemoryMap::new(Rc::clone(&ppu), ines.program(), wram);
    let mut cpu = cpu::CPU::new(cpu_register, cpu_memory);

    cpu.reset()?;
    app(&cli, &mut cpu, ppu, display).await?;

    Ok(())
}

async fn app<
    CPUM: memory::RAM<usize> + memory::ROM<[usize; 2], Output = u16> + std::fmt::Display,
    VRAM: memory::RAM<usize>,
    CHROM: memory::ROM<std::ops::Range<usize>, Output = Vec<u8>>,
>(
    cli: &CLI,
    cpu: &mut cpu::CPU<CPUM>,
    ppu: Rc<RefCell<ppu::PPU<VRAM, CHROM>>>,
    display: Rc<RefCell<display::Display>>,
) -> Result<()> {
    loop {
        if macroquad::input::is_key_pressed(macroquad::input::KeyCode::Space) {
            cpu.reset()?;
        } else {
            loop {
                let cycle = cpu.exec(cli.debug)?;
                let cycle = ppu.borrow_mut().exec(cycle * 3)?;
                if cli.debug {
                    println!("{}", cpu);
                }
                if cycle == 0 {
                    break;
                }
            }
        }

        let image = &display.borrow().image;
        let tx = macroquad::texture::Texture2D::from_image(&image);
        quad::draw_texture(&tx, 0f32, 0f32, quad::WHITE);
        quad::next_frame().await;
    }
}
