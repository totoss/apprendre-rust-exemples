#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::pio_programs::ws2812::{PioWs2812, PioWs2812Program};
use embassy_time::Timer;
use panic_halt as _; // en cas de panic : boucle infinie
use smart_leds::RGB8;

// Relie chaque interruption matérielle au gestionnaire du pilote qui l'attend.
bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
    DMA_IRQ_0 => embassy_rp::dma::InterruptHandler<DMA_CH0>;
});

// Intensité 16 sur 255 : à pleine puissance, cette LED éblouit.
const COULEURS: [RGB8; 3] = [RGB8::new(16, 0, 0), RGB8::new(0, 16, 0), RGB8::new(0, 0, 16)];

#[embassy_executor::main(executor = "embassy_rp::executor::Executor", entry = "cortex_m_rt::entry")]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // La LED du RP2040-Zero est une WS2812 sur la broche 16. Son signal, trop rapide
    // pour une broche pilotée à la main, est produit par un bloc PIO alimenté par DMA.
    let Pio { mut common, sm0, .. } = Pio::new(p.PIO0, Irqs);
    let programme = PioWs2812Program::new(&mut common);
    let mut led = PioWs2812::new(&mut common, sm0, p.DMA_CH0, Irqs, p.PIN_16, &programme);

    loop {
        for couleur in COULEURS {
            led.write(&[couleur]).await; // rouge, puis vert, puis bleu
            Timer::after_millis(500).await;
        }
    }
}
