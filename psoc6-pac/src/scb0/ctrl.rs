#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVS` reader - N/A"]
pub type OVS_R = crate::FieldReader;
#[doc = "Field `OVS` writer - N/A"]
pub type OVS_W<'a, const O: u8> = crate::FieldWriter<'a, CTRL_SPEC, 4, O>;
#[doc = "Field `EC_AM_MODE` reader - Internally clocked mode ('0') or externally clocked mode ('1') address matching (I2C) or selection (SPI). In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. In UART mode this field should be '0'."]
pub type EC_AM_MODE_R = crate::BitReader;
#[doc = "Field `EC_AM_MODE` writer - Internally clocked mode ('0') or externally clocked mode ('1') address matching (I2C) or selection (SPI). In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. In UART mode this field should be '0'."]
pub type EC_AM_MODE_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `EC_OP_MODE` reader - Internally clocked mode ('0') or externally clocked mode ('1') operation. In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked operation mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode AND EZ mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. The maximum SPI slave, EZ mode bitrate is 48 Mbps (transmission and IO delays outside the IP will degrade the effective bitrate). In UART mode this field should be '0'."]
pub type EC_OP_MODE_R = crate::BitReader;
#[doc = "Field `EC_OP_MODE` writer - Internally clocked mode ('0') or externally clocked mode ('1') operation. In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked operation mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode AND EZ mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. The maximum SPI slave, EZ mode bitrate is 48 Mbps (transmission and IO delays outside the IP will degrade the effective bitrate). In UART mode this field should be '0'."]
pub type EC_OP_MODE_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `EZ_MODE` reader - Non EZ mode ('0') or EZ mode ('1'). In EZ mode, a meta protocol is applied to the serial interface protocol. This meta protocol adds meaning to the data frames transferred by the serial interface protocol: a data frame can represent a memory address, a write memory data element or a read memory data element. EZ mode is only used for synchronous serial interface protocols: SPI and I2C. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported and the transmitter should use continuous data frames; i.e. data frames mot separated by slave deselection. This mode is only applicable to slave functionality. In EZ mode, the slave can read from and write to an addressable memory structure of up to 256 bytes. In EZ mode, data frames should 8-bit in size and should be transmitted and received with the Most Significant Bit (MSB) first. In UART mode this field should be '0'."]
pub type EZ_MODE_R = crate::BitReader;
#[doc = "Field `EZ_MODE` writer - Non EZ mode ('0') or EZ mode ('1'). In EZ mode, a meta protocol is applied to the serial interface protocol. This meta protocol adds meaning to the data frames transferred by the serial interface protocol: a data frame can represent a memory address, a write memory data element or a read memory data element. EZ mode is only used for synchronous serial interface protocols: SPI and I2C. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported and the transmitter should use continuous data frames; i.e. data frames mot separated by slave deselection. This mode is only applicable to slave functionality. In EZ mode, the slave can read from and write to an addressable memory structure of up to 256 bytes. In EZ mode, data frames should 8-bit in size and should be transmitted and received with the Most Significant Bit (MSB) first. In UART mode this field should be '0'."]
pub type EZ_MODE_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `BYTE_MODE` reader - Determines the number of bits per FIFO data element: '0': 16-bit FIFO data elements. '1': 8-bit FIFO data elements. This mode doubles the amount of FIFO entries, but TX_CTRL.DATA_WIDTH and RX_CTRL.DATA_WIDTH are restricted to \\[0, 7\\]."]
pub type BYTE_MODE_R = crate::BitReader;
#[doc = "Field `BYTE_MODE` writer - Determines the number of bits per FIFO data element: '0': 16-bit FIFO data elements. '1': 8-bit FIFO data elements. This mode doubles the amount of FIFO entries, but TX_CTRL.DATA_WIDTH and RX_CTRL.DATA_WIDTH are restricted to \\[0, 7\\]."]
pub type BYTE_MODE_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `CMD_RESP_MODE` reader - Determines CMD_RESP mode of operation: '0': CMD_RESP mode disabled. '1': CMD_RESP mode enabled (also requires EC_AM_MODE and EC_OP_MODE to be set to '1')."]
pub type CMD_RESP_MODE_R = crate::BitReader;
#[doc = "Field `CMD_RESP_MODE` writer - Determines CMD_RESP mode of operation: '0': CMD_RESP mode disabled. '1': CMD_RESP mode enabled (also requires EC_AM_MODE and EC_OP_MODE to be set to '1')."]
pub type CMD_RESP_MODE_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `ADDR_ACCEPT` reader - Determines whether a received matching address is accepted in the RX FIFO ('1') or not ('0'). In I2C mode, this field is used to allow the slave to put the received slave address or general call address in the RX FIFO. Note that a received matching address is put in the RX FIFO when ADDR_ACCEPT is '1' for both I2C read and write transfers. In multi-processor UART receiver mode, this field is used to allow the receiver to put the received address in the RX FIFO. Note: non-matching addresses are never put in the RX FIFO."]
pub type ADDR_ACCEPT_R = crate::BitReader;
#[doc = "Field `ADDR_ACCEPT` writer - Determines whether a received matching address is accepted in the RX FIFO ('1') or not ('0'). In I2C mode, this field is used to allow the slave to put the received slave address or general call address in the RX FIFO. Note that a received matching address is put in the RX FIFO when ADDR_ACCEPT is '1' for both I2C read and write transfers. In multi-processor UART receiver mode, this field is used to allow the receiver to put the received address in the RX FIFO. Note: non-matching addresses are never put in the RX FIFO."]
pub type ADDR_ACCEPT_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `BLOCK` reader - Only used in externally clocked mode. If the externally clocked logic and the MMIO SW accesses to EZ memory coincide/collide, this bit determines whether a SW access should block and result in bus wait states ('BLOCK is 1') or not (BLOCK is '0'). IF BLOCK is '0' and the accesses collide, MMIO read operations return 0xffff:ffff and MMIO write operations are ignored. Colliding accesses are registered as interrupt causes: field BLOCKED of MMIO registers INTR_TX and INTR_RX."]
pub type BLOCK_R = crate::BitReader;
#[doc = "Field `BLOCK` writer - Only used in externally clocked mode. If the externally clocked logic and the MMIO SW accesses to EZ memory coincide/collide, this bit determines whether a SW access should block and result in bus wait states ('BLOCK is 1') or not (BLOCK is '0'). IF BLOCK is '0' and the accesses collide, MMIO read operations return 0xffff:ffff and MMIO write operations are ignored. Colliding accesses are registered as interrupt causes: field BLOCKED of MMIO registers INTR_TX and INTR_RX."]
pub type BLOCK_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `MODE` reader - N/A"]
pub type MODE_R = crate::FieldReader<MODE_A>;
#[doc = "N/A\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Inter-Integrated Circuits (I2C) mode."]
    I2C = 0,
    #[doc = "1: Serial Peripheral Interface (SPI) mode."]
    SPI = 1,
    #[doc = "2: Universal Asynchronous Receiver/Transmitter (UART) mode."]
    UART = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE_A {
    type Ux = u8;
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::I2C),
            1 => Some(MODE_A::SPI),
            2 => Some(MODE_A::UART),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `I2C`"]
    #[inline(always)]
    pub fn is_i2c(&self) -> bool {
        *self == MODE_A::I2C
    }
    #[doc = "Checks if the value of the field is `SPI`"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == MODE_A::SPI
    }
    #[doc = "Checks if the value of the field is `UART`"]
    #[inline(always)]
    pub fn is_uart(&self) -> bool {
        *self == MODE_A::UART
    }
}
#[doc = "Field `MODE` writer - N/A"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, CTRL_SPEC, 2, O, MODE_A>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Inter-Integrated Circuits (I2C) mode."]
    #[inline(always)]
    pub fn i2c(self) -> &'a mut W {
        self.variant(MODE_A::I2C)
    }
    #[doc = "Serial Peripheral Interface (SPI) mode."]
    #[inline(always)]
    pub fn spi(self) -> &'a mut W {
        self.variant(MODE_A::SPI)
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter (UART) mode."]
    #[inline(always)]
    pub fn uart(self) -> &'a mut W {
        self.variant(MODE_A::UART)
    }
}
#[doc = "Field `ENABLED` reader - IP enabled ('1') or not ('0'). The proper order in which to initialize the IP is as follows: - Program protocol specific information using SPI_CTRL, UART_CTRL (and UART_TX_CTRL and UART_RX_CTRL) or I2C_CTRL. This includes selection of a submode, master/slave functionality and transmitter/receiver functionality when applicable. - Program generic transmitter (TX_CTRL) and receiver (RX_CTRL) information. This includes enabling of the transmitter and receiver functionality. - Program transmitter FIFO (TX_FIFO_CTRL) and receiver FIFO (RX_FIFO_CTRL) information. - Program CTRL to enable IP, select the specific operation mode and oversampling factor. When the IP is enabled, no control information should be changed. Changes should be made AFTER disabling the IP, e.g. to modify the operation mode (from I2C to SPI) or to go from externally to internally clocked. The change takes effect after the IP is re-enabled. Note that disabling the IP will cause re-initialization of the design and associated state is lost (e.g. FIFO content)."]
pub type ENABLED_R = crate::BitReader;
#[doc = "Field `ENABLED` writer - IP enabled ('1') or not ('0'). The proper order in which to initialize the IP is as follows: - Program protocol specific information using SPI_CTRL, UART_CTRL (and UART_TX_CTRL and UART_RX_CTRL) or I2C_CTRL. This includes selection of a submode, master/slave functionality and transmitter/receiver functionality when applicable. - Program generic transmitter (TX_CTRL) and receiver (RX_CTRL) information. This includes enabling of the transmitter and receiver functionality. - Program transmitter FIFO (TX_FIFO_CTRL) and receiver FIFO (RX_FIFO_CTRL) information. - Program CTRL to enable IP, select the specific operation mode and oversampling factor. When the IP is enabled, no control information should be changed. Changes should be made AFTER disabling the IP, e.g. to modify the operation mode (from I2C to SPI) or to go from externally to internally clocked. The change takes effect after the IP is re-enabled. Note that disabling the IP will cause re-initialization of the design and associated state is lost (e.g. FIFO content)."]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
impl R {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    pub fn ovs(&self) -> OVS_R {
        OVS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Internally clocked mode ('0') or externally clocked mode ('1') address matching (I2C) or selection (SPI). In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. In UART mode this field should be '0'."]
    #[inline(always)]
    pub fn ec_am_mode(&self) -> EC_AM_MODE_R {
        EC_AM_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Internally clocked mode ('0') or externally clocked mode ('1') operation. In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked operation mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode AND EZ mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. The maximum SPI slave, EZ mode bitrate is 48 Mbps (transmission and IO delays outside the IP will degrade the effective bitrate). In UART mode this field should be '0'."]
    #[inline(always)]
    pub fn ec_op_mode(&self) -> EC_OP_MODE_R {
        EC_OP_MODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Non EZ mode ('0') or EZ mode ('1'). In EZ mode, a meta protocol is applied to the serial interface protocol. This meta protocol adds meaning to the data frames transferred by the serial interface protocol: a data frame can represent a memory address, a write memory data element or a read memory data element. EZ mode is only used for synchronous serial interface protocols: SPI and I2C. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported and the transmitter should use continuous data frames; i.e. data frames mot separated by slave deselection. This mode is only applicable to slave functionality. In EZ mode, the slave can read from and write to an addressable memory structure of up to 256 bytes. In EZ mode, data frames should 8-bit in size and should be transmitted and received with the Most Significant Bit (MSB) first. In UART mode this field should be '0'."]
    #[inline(always)]
    pub fn ez_mode(&self) -> EZ_MODE_R {
        EZ_MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Determines the number of bits per FIFO data element: '0': 16-bit FIFO data elements. '1': 8-bit FIFO data elements. This mode doubles the amount of FIFO entries, but TX_CTRL.DATA_WIDTH and RX_CTRL.DATA_WIDTH are restricted to \\[0, 7\\]."]
    #[inline(always)]
    pub fn byte_mode(&self) -> BYTE_MODE_R {
        BYTE_MODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Determines CMD_RESP mode of operation: '0': CMD_RESP mode disabled. '1': CMD_RESP mode enabled (also requires EC_AM_MODE and EC_OP_MODE to be set to '1')."]
    #[inline(always)]
    pub fn cmd_resp_mode(&self) -> CMD_RESP_MODE_R {
        CMD_RESP_MODE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Determines whether a received matching address is accepted in the RX FIFO ('1') or not ('0'). In I2C mode, this field is used to allow the slave to put the received slave address or general call address in the RX FIFO. Note that a received matching address is put in the RX FIFO when ADDR_ACCEPT is '1' for both I2C read and write transfers. In multi-processor UART receiver mode, this field is used to allow the receiver to put the received address in the RX FIFO. Note: non-matching addresses are never put in the RX FIFO."]
    #[inline(always)]
    pub fn addr_accept(&self) -> ADDR_ACCEPT_R {
        ADDR_ACCEPT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Only used in externally clocked mode. If the externally clocked logic and the MMIO SW accesses to EZ memory coincide/collide, this bit determines whether a SW access should block and result in bus wait states ('BLOCK is 1') or not (BLOCK is '0'). IF BLOCK is '0' and the accesses collide, MMIO read operations return 0xffff:ffff and MMIO write operations are ignored. Colliding accesses are registered as interrupt causes: field BLOCKED of MMIO registers INTR_TX and INTR_RX."]
    #[inline(always)]
    pub fn block(&self) -> BLOCK_R {
        BLOCK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:25 - N/A"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 31 - IP enabled ('1') or not ('0'). The proper order in which to initialize the IP is as follows: - Program protocol specific information using SPI_CTRL, UART_CTRL (and UART_TX_CTRL and UART_RX_CTRL) or I2C_CTRL. This includes selection of a submode, master/slave functionality and transmitter/receiver functionality when applicable. - Program generic transmitter (TX_CTRL) and receiver (RX_CTRL) information. This includes enabling of the transmitter and receiver functionality. - Program transmitter FIFO (TX_FIFO_CTRL) and receiver FIFO (RX_FIFO_CTRL) information. - Program CTRL to enable IP, select the specific operation mode and oversampling factor. When the IP is enabled, no control information should be changed. Changes should be made AFTER disabling the IP, e.g. to modify the operation mode (from I2C to SPI) or to go from externally to internally clocked. The change takes effect after the IP is re-enabled. Note that disabling the IP will cause re-initialization of the design and associated state is lost (e.g. FIFO content)."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn ovs(&mut self) -> OVS_W<0> {
        OVS_W::new(self)
    }
    #[doc = "Bit 8 - Internally clocked mode ('0') or externally clocked mode ('1') address matching (I2C) or selection (SPI). In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. In UART mode this field should be '0'."]
    #[inline(always)]
    #[must_use]
    pub fn ec_am_mode(&mut self) -> EC_AM_MODE_W<8> {
        EC_AM_MODE_W::new(self)
    }
    #[doc = "Bit 9 - Internally clocked mode ('0') or externally clocked mode ('1') operation. In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked operation mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode AND EZ mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. The maximum SPI slave, EZ mode bitrate is 48 Mbps (transmission and IO delays outside the IP will degrade the effective bitrate). In UART mode this field should be '0'."]
    #[inline(always)]
    #[must_use]
    pub fn ec_op_mode(&mut self) -> EC_OP_MODE_W<9> {
        EC_OP_MODE_W::new(self)
    }
    #[doc = "Bit 10 - Non EZ mode ('0') or EZ mode ('1'). In EZ mode, a meta protocol is applied to the serial interface protocol. This meta protocol adds meaning to the data frames transferred by the serial interface protocol: a data frame can represent a memory address, a write memory data element or a read memory data element. EZ mode is only used for synchronous serial interface protocols: SPI and I2C. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported and the transmitter should use continuous data frames; i.e. data frames mot separated by slave deselection. This mode is only applicable to slave functionality. In EZ mode, the slave can read from and write to an addressable memory structure of up to 256 bytes. In EZ mode, data frames should 8-bit in size and should be transmitted and received with the Most Significant Bit (MSB) first. In UART mode this field should be '0'."]
    #[inline(always)]
    #[must_use]
    pub fn ez_mode(&mut self) -> EZ_MODE_W<10> {
        EZ_MODE_W::new(self)
    }
    #[doc = "Bit 11 - Determines the number of bits per FIFO data element: '0': 16-bit FIFO data elements. '1': 8-bit FIFO data elements. This mode doubles the amount of FIFO entries, but TX_CTRL.DATA_WIDTH and RX_CTRL.DATA_WIDTH are restricted to \\[0, 7\\]."]
    #[inline(always)]
    #[must_use]
    pub fn byte_mode(&mut self) -> BYTE_MODE_W<11> {
        BYTE_MODE_W::new(self)
    }
    #[doc = "Bit 12 - Determines CMD_RESP mode of operation: '0': CMD_RESP mode disabled. '1': CMD_RESP mode enabled (also requires EC_AM_MODE and EC_OP_MODE to be set to '1')."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_resp_mode(&mut self) -> CMD_RESP_MODE_W<12> {
        CMD_RESP_MODE_W::new(self)
    }
    #[doc = "Bit 16 - Determines whether a received matching address is accepted in the RX FIFO ('1') or not ('0'). In I2C mode, this field is used to allow the slave to put the received slave address or general call address in the RX FIFO. Note that a received matching address is put in the RX FIFO when ADDR_ACCEPT is '1' for both I2C read and write transfers. In multi-processor UART receiver mode, this field is used to allow the receiver to put the received address in the RX FIFO. Note: non-matching addresses are never put in the RX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn addr_accept(&mut self) -> ADDR_ACCEPT_W<16> {
        ADDR_ACCEPT_W::new(self)
    }
    #[doc = "Bit 17 - Only used in externally clocked mode. If the externally clocked logic and the MMIO SW accesses to EZ memory coincide/collide, this bit determines whether a SW access should block and result in bus wait states ('BLOCK is 1') or not (BLOCK is '0'). IF BLOCK is '0' and the accesses collide, MMIO read operations return 0xffff:ffff and MMIO write operations are ignored. Colliding accesses are registered as interrupt causes: field BLOCKED of MMIO registers INTR_TX and INTR_RX."]
    #[inline(always)]
    #[must_use]
    pub fn block(&mut self) -> BLOCK_W<17> {
        BLOCK_W::new(self)
    }
    #[doc = "Bits 24:25 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<24> {
        MODE_W::new(self)
    }
    #[doc = "Bit 31 - IP enabled ('1') or not ('0'). The proper order in which to initialize the IP is as follows: - Program protocol specific information using SPI_CTRL, UART_CTRL (and UART_TX_CTRL and UART_RX_CTRL) or I2C_CTRL. This includes selection of a submode, master/slave functionality and transmitter/receiver functionality when applicable. - Program generic transmitter (TX_CTRL) and receiver (RX_CTRL) information. This includes enabling of the transmitter and receiver functionality. - Program transmitter FIFO (TX_FIFO_CTRL) and receiver FIFO (RX_FIFO_CTRL) information. - Program CTRL to enable IP, select the specific operation mode and oversampling factor. When the IP is enabled, no control information should be changed. Changes should be made AFTER disabling the IP, e.g. to modify the operation mode (from I2C to SPI) or to go from externally to internally clocked. The change takes effect after the IP is re-enabled. Note that disabling the IP will cause re-initialization of the design and associated state is lost (e.g. FIFO content)."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> ENABLED_W<31> {
        ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Generic control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0300_000f"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0300_000f;
}
