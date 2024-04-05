use bitflags::bitflags;

#[repr(C)]
pub struct GpioxT {
    pub moder: u32,
    pub otyper: u32,
    pub ospeedr: u32,
    pub pupdr: u32,
    pub idr: u32,
    pub odr: u32,
    pub bsrr: u32,
    pub lckr: u32,
    pub afrl: u32,
    pub afrh: u32,
}


impl GpioxT {
    pub fn set_mode(&mut self, mode: GpioMode, pin: GpioPin) {

        let _pin = pin as u8;

        //Reset Moder register
        self.moder &= !(0b11 << (2 * (_pin as u8)));
        //Set Mode
        self.moder |= (mode.bits() & 0b11) << (2 * (_pin as u8))
    }

    pub fn toggle_pin(&mut self, pin: GpioPin){

        let _pin = pin as u8;

        self.moder ^= 1 << (2 * (_pin as u8));
    }

    pub fn set_pin(&mut self, pin: GpioPin){

        let _pin = pin as u8;

        self.moder |= 1 << (2 * (_pin as u8));
    }

    pub fn reset_pin(&mut self, pin: GpioPin){

        let _pin = pin as u8;

        self.moder &= !(1 << (2 * (_pin as u8)));
    }
}


bitflags! {
    pub struct GpioMode: u32{
        const INPUT   = 0b00;
        const OUTPUT  = 0b01;
        const ALT     = 0b10;
        const ANALOG  = 0b11;
    }
}

pub enum GpioPin{
    PIN0 = 0,
    PIN1 = 1,
    PIN2 = 2,
    PIN3 = 3,
    PIN4 = 4,
    PIN5 = 5,
    PIN6 = 6,
    PIN7 = 7,
    PIN8 = 8,
    PIN9 = 9,
    PIN10 = 10,
    PIN11 = 11,
    PIN12 = 12,
    PIN13 = 13,
    PIN14 = 14,
    PIN15 = 15,
}


