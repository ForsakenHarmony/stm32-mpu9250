#![no_std]
#![no_main]

extern crate panic_semihosting as _;

use stm32l4xx_hal as hal;

use crate::hal::{delay::Delay, i2c::I2c, prelude::*, spi::Spi};
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use mpu9250::MODE;

#[entry]
fn main() -> ! {
	let cp = cortex_m::Peripherals::take().unwrap();
	let dp = hal::stm32::Peripherals::take().unwrap();

	let mut flash = dp.FLASH.constrain();
	let mut rcc = dp.RCC.constrain();
	let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

	// TRY the other clock configuration
	// let clocks = rcc.cfgr.freeze(&mut flash.acr);
	let clocks = rcc
		.cfgr
		.sysclk(80.mhz())
		.pclk1(80.mhz())
		.pclk2(80.mhz())
		.freeze(&mut flash.acr, &mut pwr);

	hprintln!("boot");

	// let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
	let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
	// let mut gpioc = dp.GPIOC.split(&mut rcc.ahb2);

	// let sck = gpiob.pb13.into_af5(&mut gpiob.moder, &mut gpiob.afrh);
	// let miso = gpiob.pb14.into_af5(&mut gpiob.moder, &mut gpiob.afrh);
	// let mosi = gpiob.pb15.into_af5(&mut gpiob.moder, &mut gpiob.afrh);

	let scl = gpiob
		.pb10
		.into_open_drain_output(&mut gpiob.moder, &mut gpiob.otyper)
		.into_af4(&mut gpiob.moder, &mut gpiob.afrh);
	let sda = gpiob
		.pb11
		.into_open_drain_output(&mut gpiob.moder, &mut gpiob.otyper)
		.into_af4(&mut gpiob.moder, &mut gpiob.afrh);

	let mut i2c = I2c::i2c2(dp.I2C2, (scl, sda), 100.khz(), clocks, &mut rcc.apb1r1);

	// let mut spi = Spi::spi2(
	// 	dp.SPI2,
	// 	(sck, miso, mosi),
	// 	MODE,
	// 	3.mhz(),
	// 	clocks,
	// 	&mut rcc.apb1r1,
	// );

	// let ncs = gpiob.pb12.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

	let mut delay = Delay::new(cp.SYST, clocks);
	let mut mpu = mpu9250::Mpu9250::marg_default(i2c, &mut delay).unwrap();

	let all = mpu.all::<[f32; 3]>().unwrap();
	hprintln!("{:?}", all).unwrap();

	loop {}
}
