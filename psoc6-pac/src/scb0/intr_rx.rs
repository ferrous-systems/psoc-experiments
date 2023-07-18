#[doc = "Register `INTR_RX` reader"]
pub struct R(crate::R<INTR_RX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_RX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_RX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_RX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_RX` writer"]
pub struct W(crate::W<INTR_RX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_RX_SPEC>;
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
impl From<crate::W<INTR_RX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_RX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIGGER` reader - More entries in the RX FIFO than the value specified by RX_FIFO_CTRL.TRIGGER_LEVEL. Only used in FIFO mode."]
pub type TRIGGER_R = crate::BitReader;
#[doc = "Field `TRIGGER` writer - More entries in the RX FIFO than the value specified by RX_FIFO_CTRL.TRIGGER_LEVEL. Only used in FIFO mode."]
pub type TRIGGER_W<'a, const O: u8> = crate::BitWriter<'a, INTR_RX_SPEC, O>;
#[doc = "Field `NOT_EMPTY` reader - RX FIFO is not empty. Only used in FIFO mode."]
pub type NOT_EMPTY_R = crate::BitReader;
#[doc = "Field `NOT_EMPTY` writer - RX FIFO is not empty. Only used in FIFO mode."]
pub type NOT_EMPTY_W<'a, const O: u8> = crate::BitWriter<'a, INTR_RX_SPEC, O>;
#[doc = "Field `FULL` reader - RX FIFO is full. Note that received data frames are lost when the RX FIFO is full. Dependent on CTRL.BYTE_MODE: (FF_DATA_NR = EZ_DATA_NR/2) BYTE_MODE is '0': # entries == FF_DATA_NR/2. BYTE_MODE is '1': # entries == FF_DATA_NR. Only used in FIFO mode."]
pub type FULL_R = crate::BitReader;
#[doc = "Field `FULL` writer - RX FIFO is full. Note that received data frames are lost when the RX FIFO is full. Dependent on CTRL.BYTE_MODE: (FF_DATA_NR = EZ_DATA_NR/2) BYTE_MODE is '0': # entries == FF_DATA_NR/2. BYTE_MODE is '1': # entries == FF_DATA_NR. Only used in FIFO mode."]
pub type FULL_W<'a, const O: u8> = crate::BitWriter<'a, INTR_RX_SPEC, O>;
#[doc = "Field `OVERFLOW` reader - Attempt to write to a full RX FIFO. Note: in I2C mode, the OVERFLOW is set when a data frame is received and the RX FIFO is full, independent of whether it is ACK'd or NACK'd. Only used in FIFO mode."]
pub type OVERFLOW_R = crate::BitReader;
#[doc = "Field `OVERFLOW` writer - Attempt to write to a full RX FIFO. Note: in I2C mode, the OVERFLOW is set when a data frame is received and the RX FIFO is full, independent of whether it is ACK'd or NACK'd. Only used in FIFO mode."]
pub type OVERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, INTR_RX_SPEC, O>;
#[doc = "Field `UNDERFLOW` reader - Attempt to read from an empty RX FIFO. Only used in FIFO mode."]
pub type UNDERFLOW_R = crate::BitReader;
#[doc = "Field `UNDERFLOW` writer - Attempt to read from an empty RX FIFO. Only used in FIFO mode."]
pub type UNDERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, INTR_RX_SPEC, O>;
#[doc = "Field `BLOCKED` reader - AHB-Lite read transfer can not get access to the EZ memory (EZ_DATA accesses), due to an externally clocked EZ access. This may happen when STATUS.EC_BUSY is '1'."]
pub type BLOCKED_R = crate::BitReader;
#[doc = "Field `BLOCKED` writer - AHB-Lite read transfer can not get access to the EZ memory (EZ_DATA accesses), due to an externally clocked EZ access. This may happen when STATUS.EC_BUSY is '1'."]
pub type BLOCKED_W<'a, const O: u8> = crate::BitWriter<'a, INTR_RX_SPEC, O>;
#[doc = "Field `FRAME_ERROR` reader - Frame error in received data frame. Set to '1', when event is detected. Write with '1' to clear bit. This can be either a start or stop bit(s) error: Start bit error: after the detection of the beginning of a start bit period (RX line changes from '1' to '0'), the middle of the start bit period is sampled erroneously (RX line is '1'). Note: a start bit error is detected BEFORE a data frame is received. Stop bit error: the RX line is sampled as '0', but a '1' was expected. Note: a stop bit error may result in failure to receive successive data frame(s). Note: a stop bit error is detected AFTER a data frame is received. A stop bit error is detected after a data frame is received, and the UART_RX_CTL.DROP_ON_FRAME_ERROR field specifies whether the received frame is dropped or send to the RX FIFO. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '1', the received data frame is dropped. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '0', the received data frame is send to the RX FIFO. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO; i.e. the RX FIFO does not have error flags to tag erroneous data frames."]
pub type FRAME_ERROR_R = crate::BitReader;
#[doc = "Field `FRAME_ERROR` writer - Frame error in received data frame. Set to '1', when event is detected. Write with '1' to clear bit. This can be either a start or stop bit(s) error: Start bit error: after the detection of the beginning of a start bit period (RX line changes from '1' to '0'), the middle of the start bit period is sampled erroneously (RX line is '1'). Note: a start bit error is detected BEFORE a data frame is received. Stop bit error: the RX line is sampled as '0', but a '1' was expected. Note: a stop bit error may result in failure to receive successive data frame(s). Note: a stop bit error is detected AFTER a data frame is received. A stop bit error is detected after a data frame is received, and the UART_RX_CTL.DROP_ON_FRAME_ERROR field specifies whether the received frame is dropped or send to the RX FIFO. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '1', the received data frame is dropped. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '0', the received data frame is send to the RX FIFO. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO; i.e. the RX FIFO does not have error flags to tag erroneous data frames."]
pub type FRAME_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, INTR_RX_SPEC, O>;
#[doc = "Field `PARITY_ERROR` reader - Parity error in received data frame. Set to '1', when event is detected. Write with '1' to clear bit. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '1', the received frame is dropped. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '0', the received frame is send to the RX FIFO. In SmartCard submode, negatively acknowledged data frames generate a parity error. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO."]
pub type PARITY_ERROR_R = crate::BitReader;
#[doc = "Field `PARITY_ERROR` writer - Parity error in received data frame. Set to '1', when event is detected. Write with '1' to clear bit. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '1', the received frame is dropped. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '0', the received frame is send to the RX FIFO. In SmartCard submode, negatively acknowledged data frames generate a parity error. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO."]
pub type PARITY_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, INTR_RX_SPEC, O>;
#[doc = "Field `BAUD_DETECT` reader - LIN baudrate detection is completed. The receiver software uses the UART_RX_STATUS.BR_COUNTER value to set the right IP clock (from the programmable clock IP) to guarantee successful receipt of the first LIN data frame (Protected Identifier Field) after the synchronization byte. Set to '1', when event is detected. Write with '1' to clear bit."]
pub type BAUD_DETECT_R = crate::BitReader;
#[doc = "Field `BAUD_DETECT` writer - LIN baudrate detection is completed. The receiver software uses the UART_RX_STATUS.BR_COUNTER value to set the right IP clock (from the programmable clock IP) to guarantee successful receipt of the first LIN data frame (Protected Identifier Field) after the synchronization byte. Set to '1', when event is detected. Write with '1' to clear bit."]
pub type BAUD_DETECT_W<'a, const O: u8> = crate::BitWriter<'a, INTR_RX_SPEC, O>;
#[doc = "Field `BREAK_DETECT` reader - Break detection is successful: the line is '0' for UART_RX_CTRL.BREAK_WIDTH + 1 bit period. Can occur at any time to address unanticipated break fields; i.e. 'break-in-data' is supported. This feature is supported for the UART standard and LIN submodes. For the UART standard submodes, ongoing receipt of data frames is NOT affected; i.e. Firmware is expected to take the proper action. For the LIN submode, possible ongoing receipt of a data frame is stopped and the (partially) received data frame is dropped and baud rate detection is started. Set to '1', when event is detected. Write with '1' to clear bit."]
pub type BREAK_DETECT_R = crate::BitReader;
#[doc = "Field `BREAK_DETECT` writer - Break detection is successful: the line is '0' for UART_RX_CTRL.BREAK_WIDTH + 1 bit period. Can occur at any time to address unanticipated break fields; i.e. 'break-in-data' is supported. This feature is supported for the UART standard and LIN submodes. For the UART standard submodes, ongoing receipt of data frames is NOT affected; i.e. Firmware is expected to take the proper action. For the LIN submode, possible ongoing receipt of a data frame is stopped and the (partially) received data frame is dropped and baud rate detection is started. Set to '1', when event is detected. Write with '1' to clear bit."]
pub type BREAK_DETECT_W<'a, const O: u8> = crate::BitWriter<'a, INTR_RX_SPEC, O>;
impl R {
    #[doc = "Bit 0 - More entries in the RX FIFO than the value specified by RX_FIFO_CTRL.TRIGGER_LEVEL. Only used in FIFO mode."]
    #[inline(always)]
    pub fn trigger(&self) -> TRIGGER_R {
        TRIGGER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - RX FIFO is not empty. Only used in FIFO mode."]
    #[inline(always)]
    pub fn not_empty(&self) -> NOT_EMPTY_R {
        NOT_EMPTY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX FIFO is full. Note that received data frames are lost when the RX FIFO is full. Dependent on CTRL.BYTE_MODE: (FF_DATA_NR = EZ_DATA_NR/2) BYTE_MODE is '0': # entries == FF_DATA_NR/2. BYTE_MODE is '1': # entries == FF_DATA_NR. Only used in FIFO mode."]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Attempt to write to a full RX FIFO. Note: in I2C mode, the OVERFLOW is set when a data frame is received and the RX FIFO is full, independent of whether it is ACK'd or NACK'd. Only used in FIFO mode."]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Attempt to read from an empty RX FIFO. Only used in FIFO mode."]
    #[inline(always)]
    pub fn underflow(&self) -> UNDERFLOW_R {
        UNDERFLOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AHB-Lite read transfer can not get access to the EZ memory (EZ_DATA accesses), due to an externally clocked EZ access. This may happen when STATUS.EC_BUSY is '1'."]
    #[inline(always)]
    pub fn blocked(&self) -> BLOCKED_R {
        BLOCKED_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Frame error in received data frame. Set to '1', when event is detected. Write with '1' to clear bit. This can be either a start or stop bit(s) error: Start bit error: after the detection of the beginning of a start bit period (RX line changes from '1' to '0'), the middle of the start bit period is sampled erroneously (RX line is '1'). Note: a start bit error is detected BEFORE a data frame is received. Stop bit error: the RX line is sampled as '0', but a '1' was expected. Note: a stop bit error may result in failure to receive successive data frame(s). Note: a stop bit error is detected AFTER a data frame is received. A stop bit error is detected after a data frame is received, and the UART_RX_CTL.DROP_ON_FRAME_ERROR field specifies whether the received frame is dropped or send to the RX FIFO. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '1', the received data frame is dropped. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '0', the received data frame is send to the RX FIFO. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO; i.e. the RX FIFO does not have error flags to tag erroneous data frames."]
    #[inline(always)]
    pub fn frame_error(&self) -> FRAME_ERROR_R {
        FRAME_ERROR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity error in received data frame. Set to '1', when event is detected. Write with '1' to clear bit. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '1', the received frame is dropped. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '0', the received frame is send to the RX FIFO. In SmartCard submode, negatively acknowledged data frames generate a parity error. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO."]
    #[inline(always)]
    pub fn parity_error(&self) -> PARITY_ERROR_R {
        PARITY_ERROR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LIN baudrate detection is completed. The receiver software uses the UART_RX_STATUS.BR_COUNTER value to set the right IP clock (from the programmable clock IP) to guarantee successful receipt of the first LIN data frame (Protected Identifier Field) after the synchronization byte. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn baud_detect(&self) -> BAUD_DETECT_R {
        BAUD_DETECT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Break detection is successful: the line is '0' for UART_RX_CTRL.BREAK_WIDTH + 1 bit period. Can occur at any time to address unanticipated break fields; i.e. 'break-in-data' is supported. This feature is supported for the UART standard and LIN submodes. For the UART standard submodes, ongoing receipt of data frames is NOT affected; i.e. Firmware is expected to take the proper action. For the LIN submode, possible ongoing receipt of a data frame is stopped and the (partially) received data frame is dropped and baud rate detection is started. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn break_detect(&self) -> BREAK_DETECT_R {
        BREAK_DETECT_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - More entries in the RX FIFO than the value specified by RX_FIFO_CTRL.TRIGGER_LEVEL. Only used in FIFO mode."]
    #[inline(always)]
    #[must_use]
    pub fn trigger(&mut self) -> TRIGGER_W<0> {
        TRIGGER_W::new(self)
    }
    #[doc = "Bit 2 - RX FIFO is not empty. Only used in FIFO mode."]
    #[inline(always)]
    #[must_use]
    pub fn not_empty(&mut self) -> NOT_EMPTY_W<2> {
        NOT_EMPTY_W::new(self)
    }
    #[doc = "Bit 3 - RX FIFO is full. Note that received data frames are lost when the RX FIFO is full. Dependent on CTRL.BYTE_MODE: (FF_DATA_NR = EZ_DATA_NR/2) BYTE_MODE is '0': # entries == FF_DATA_NR/2. BYTE_MODE is '1': # entries == FF_DATA_NR. Only used in FIFO mode."]
    #[inline(always)]
    #[must_use]
    pub fn full(&mut self) -> FULL_W<3> {
        FULL_W::new(self)
    }
    #[doc = "Bit 5 - Attempt to write to a full RX FIFO. Note: in I2C mode, the OVERFLOW is set when a data frame is received and the RX FIFO is full, independent of whether it is ACK'd or NACK'd. Only used in FIFO mode."]
    #[inline(always)]
    #[must_use]
    pub fn overflow(&mut self) -> OVERFLOW_W<5> {
        OVERFLOW_W::new(self)
    }
    #[doc = "Bit 6 - Attempt to read from an empty RX FIFO. Only used in FIFO mode."]
    #[inline(always)]
    #[must_use]
    pub fn underflow(&mut self) -> UNDERFLOW_W<6> {
        UNDERFLOW_W::new(self)
    }
    #[doc = "Bit 7 - AHB-Lite read transfer can not get access to the EZ memory (EZ_DATA accesses), due to an externally clocked EZ access. This may happen when STATUS.EC_BUSY is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn blocked(&mut self) -> BLOCKED_W<7> {
        BLOCKED_W::new(self)
    }
    #[doc = "Bit 8 - Frame error in received data frame. Set to '1', when event is detected. Write with '1' to clear bit. This can be either a start or stop bit(s) error: Start bit error: after the detection of the beginning of a start bit period (RX line changes from '1' to '0'), the middle of the start bit period is sampled erroneously (RX line is '1'). Note: a start bit error is detected BEFORE a data frame is received. Stop bit error: the RX line is sampled as '0', but a '1' was expected. Note: a stop bit error may result in failure to receive successive data frame(s). Note: a stop bit error is detected AFTER a data frame is received. A stop bit error is detected after a data frame is received, and the UART_RX_CTL.DROP_ON_FRAME_ERROR field specifies whether the received frame is dropped or send to the RX FIFO. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '1', the received data frame is dropped. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '0', the received data frame is send to the RX FIFO. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO; i.e. the RX FIFO does not have error flags to tag erroneous data frames."]
    #[inline(always)]
    #[must_use]
    pub fn frame_error(&mut self) -> FRAME_ERROR_W<8> {
        FRAME_ERROR_W::new(self)
    }
    #[doc = "Bit 9 - Parity error in received data frame. Set to '1', when event is detected. Write with '1' to clear bit. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '1', the received frame is dropped. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '0', the received frame is send to the RX FIFO. In SmartCard submode, negatively acknowledged data frames generate a parity error. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn parity_error(&mut self) -> PARITY_ERROR_W<9> {
        PARITY_ERROR_W::new(self)
    }
    #[doc = "Bit 10 - LIN baudrate detection is completed. The receiver software uses the UART_RX_STATUS.BR_COUNTER value to set the right IP clock (from the programmable clock IP) to guarantee successful receipt of the first LIN data frame (Protected Identifier Field) after the synchronization byte. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn baud_detect(&mut self) -> BAUD_DETECT_W<10> {
        BAUD_DETECT_W::new(self)
    }
    #[doc = "Bit 11 - Break detection is successful: the line is '0' for UART_RX_CTRL.BREAK_WIDTH + 1 bit period. Can occur at any time to address unanticipated break fields; i.e. 'break-in-data' is supported. This feature is supported for the UART standard and LIN submodes. For the UART standard submodes, ongoing receipt of data frames is NOT affected; i.e. Firmware is expected to take the proper action. For the LIN submode, possible ongoing receipt of a data frame is stopped and the (partially) received data frame is dropped and baud rate detection is started. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn break_detect(&mut self) -> BREAK_DETECT_W<11> {
        BREAK_DETECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receiver interrupt request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_rx](index.html) module"]
pub struct INTR_RX_SPEC;
impl crate::RegisterSpec for INTR_RX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_rx::R](R) reader structure"]
impl crate::Readable for INTR_RX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_rx::W](W) writer structure"]
impl crate::Writable for INTR_RX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_RX to value 0"]
impl crate::Resettable for INTR_RX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
