const PERIPH_BASE:      u32 =            0x4000_0000;
const PERIPH_BASE_APB1: u32 = PERIPH_BASE;
const PERIPH_BASE_APB2: u32 = PERIPH_BASE + 0x1_0000;
const PERIPH_BASE_AHB:  u32 = PERIPH_BASE + 0x1_8000;

const RCC_BASE: u32 = PERIPH_BASE_AHB + 0x0_9000;

/// AHB peripheral clock enable register
///should = hex(0x4000_0000 + 0x1_8000 + 0x0_9000)
///       = 0x40021000
const RCC_AHBENR: *mut u32 = (RCC_BASE + 0x14) as *mut u32;

/// APB2 peripheral clock enable register (RCC_APB2ENR)
/// Has a bunch of flags described below
/// should = hex(0x4000_0000 + 0x1_8000 + 0x0_9000 + (0x304 >> 5))
///        = 0x40021018
pub const RCC_APB2ENR: *mut u32 = (RCC_BASE + 0x18) as *mut u32;
pub const RCC_APB1ENR: *mut u32 = (RCC_BASE + 0x1c) as *mut u32;

const GPIO_PORT_A_BASE: u32 = PERIPH_BASE_APB2 + 0x0800;
const GPIO_PORT_B_BASE: u32 = PERIPH_BASE_APB2 + 0x0c00;
const GPIO_PORT_C_BASE: u32 = PERIPH_BASE_APB2 + 0x1000;
const GPIO_PORT_D_BASE: u32 = PERIPH_BASE_APB2 + 0x1400;
const GPIO_PORT_E_BASE: u32 = PERIPH_BASE_APB2 + 0x1800;
const GPIO_PORT_F_BASE: u32 = PERIPH_BASE_APB2 + 0x1c00;
const GPIO_PORT_G_BASE: u32 = PERIPH_BASE_APB2 + 0x2000;

/// Bit set/reset register address
/// Write only
/// To set GPIO pin we write into the first 16 bits of BSRR
/// To reset it we write to the next 16 bits of BSRR
pub const GPIOA_BSRR: u32 = GPIO_PORT_A_BASE + 0x10;
pub const GPIOB_BSRR: u32 = GPIO_PORT_B_BASE + 0x10;
pub const GPIOC_BSRR: u32 = GPIO_PORT_C_BASE + 0x10;
pub const GPIOD_BSRR: u32 = GPIO_PORT_D_BASE + 0x10;
pub const GPIOE_BSRR: u32 = GPIO_PORT_E_BASE + 0x10;
pub const GPIOF_BSRR: u32 = GPIO_PORT_F_BASE + 0x10;
pub const GPIOG_BSRR: u32 = GPIO_PORT_G_BASE + 0x10;

/// Output data register address
/// Returns current state
/// Read/Write
pub const GPIOA_ODR: *mut u32 = (GPIO_PORT_A_BASE + 0x0C) as *mut u32;
pub const GPIOB_ODR: *mut u32 = (GPIO_PORT_B_BASE + 0x0C) as *mut u32;
pub const GPIOC_ODR: *mut u32 = (GPIO_PORT_C_BASE + 0x0C) as *mut u32;
pub const GPIOD_ODR: *mut u32 = (GPIO_PORT_D_BASE + 0x0C) as *mut u32;
pub const GPIOE_ODR: *mut u32 = (GPIO_PORT_E_BASE + 0x0C) as *mut u32;
pub const GPIOF_ODR: *mut u32 = (GPIO_PORT_F_BASE + 0x0C) as *mut u32;
pub const GPIOG_ODR: *mut u32 = (GPIO_PORT_G_BASE + 0x0C) as *mut u32;

// why do we need both BSRR (Bit Set/Reset Register)
// and BRR (Bit Reset Register)?
// maybe not
const GPIOA_BRR: *mut u32 = (GPIO_PORT_A_BASE + 0x14) as *mut u32;
const GPIOB_BRR: *mut u32 = (GPIO_PORT_B_BASE + 0x14) as *mut u32;
const GPIOC_BRR: *mut u32 = (GPIO_PORT_C_BASE + 0x14) as *mut u32;
const GPIOD_BRR: *mut u32 = (GPIO_PORT_D_BASE + 0x14) as *mut u32;
const GPIOE_BRR: *mut u32 = (GPIO_PORT_E_BASE + 0x14) as *mut u32;
const GPIOF_BRR: *mut u32 = (GPIO_PORT_F_BASE + 0x14) as *mut u32;
const GPIOG_BRR: *mut u32 = (GPIO_PORT_G_BASE + 0x14) as *mut u32;

const GPIO0:  u16 = 1 <<  0;
const GPIO1:  u16 = 1 <<  1;
const GPIO2:  u16 = 1 <<  2;
const GPIO3:  u16 = 1 <<  3;
const GPIO4:  u16 = 1 <<  4;
const GPIO5:  u16 = 1 <<  5;
const GPIO6:  u16 = 1 <<  6;
const GPIO7:  u16 = 1 <<  7;
const GPIO8:  u16 = 1 <<  8;
const GPIO9:  u16 = 1 <<  9;
const GPIO10: u16 = 1 << 10;
const GPIO11: u16 = 1 << 11;
const GPIO12: u16 = 1 << 12;
const GPIO13: u16 = 1 << 13;
const GPIO14: u16 = 1 << 14;
const GPIO15: u16 = 1 << 15;
const GPIO_ALL: u16 = 0xffff;

// Flags for RCC_APB2ENR
/// Alternate function IO clock enable
pub const RCC_APB2ENR_AFIOEN: u32 = 1;

// bit 1 is reserved

/// IO port A-D clock enable
pub const RCC_APB2ENR_IOPAEN: u32 = 1 << 2;
pub const RCC_APB2ENR_IOPBEN: u32 = 1 << 3;
pub const RCC_APB2ENR_IOPCEN: u32 = 1 << 4;
pub const RCC_APB2ENR_IOPDEN: u32 = 1 << 5;
pub const RCC_APB2ENR_IOPEEN: u32 = 1 << 6;
// there are more flags, I'm just lazy to fill them all in right now

/// Port configuration register low (GPIOx_CRH) (x=A..G)
#[allow(clippy::identity_op)]
pub const GPIOC_CRL: *mut u32 = (GPIO_PORT_C_BASE + 0x00) as *mut u32;

/// Port configuration register high (GPIOx_CRH) (x=A..G)
pub const GPIOC_CRH: *mut u32 = (GPIO_PORT_C_BASE + 0x04) as *mut u32;

/// CRL and CRH have the same structure
/// of alternating MODE (4 bits) and CNF (4 bits) (configuration flags)
/// for each port (i.e. pin).

/// Port Configuration Flags (CNF) for GPIOx_CRL/GPIOx_CRH
/// When port configured as output
///   (they do different things when configured as input)
pub const GPIO_CNF_OUTPUT_PUSHPULL:        u8 = 0b00;
pub const GPIO_CNF_OUTPUT_OPENDRAIN:       u8 = 0b01;
pub const GPIO_CNF_OUTPUT_ALTFN_PUSHPULL:  u8 = 0b10;
pub const GPIO_CNF_OUTPUT_ALTFN_OPENDRAIN: u8 = 0b11;

/// Pin mode (MODE[1:0]) bits for GPIOx_CRL/GPIOx_CRH
pub const GPIO_MODE_INPUT:         u8 = 0b00;
pub const GPIO_MODE_OUTPUT_10_MHZ: u8 = 0b01;
pub const GPIO_MODE_OUTPUT_2_MHZ:  u8 = 0b10;
pub const GPIO_MODE_OUTPUT_50_MHZ: u8 = 0b11;
