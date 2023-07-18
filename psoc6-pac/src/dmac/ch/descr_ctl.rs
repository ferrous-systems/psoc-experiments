#[doc = "Register `DESCR_CTL` reader"]
pub struct R(crate::R<DESCR_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESCR_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESCR_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESCR_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WAIT_FOR_DEACT` reader - Specifies whether the controller should wait for the input trigger to be deactivated; i.e. the selected system trigger is not active. This field is used to synchronize the controller with the agent that generated the trigger. This field is ONLY used at the completion of the transfer as specified by TR_IN. E.g., a TX FIFO indicates that it is empty and it needs a new data sample. The agent removes the trigger ONLY when the data sample has been written by the controller AND received by the agent. Furthermore, the agent's trigger may be delayed by a few cycles before it reaches the controller. This field is used for level sensitive trigger, which reflect state (pulse sensitive triggers should have this field set to '0'). The wait cycles incurred by this field reduce DW controller performance. '0': Do not wait for trigger de-activation (for pulse sensitive triggers). '1': Wait for up to 4 cycles. '2': Wait for up to 16 cycles. '3': Wait indefinitely. This option may result in controller lockup if the trigger is not de-activated."]
pub type WAIT_FOR_DEACT_R = crate::FieldReader;
#[doc = "Field `INTR_TYPE` reader - Specifies when a completion interrupt is generated (CH_STATUS.INTR_CAUSE is set to COMPLETION): '0': An interrupt is generated after a single transfer. '1': An interrupt is generated after a single 1D transfer or a memory copy transfer - If the descriptor type is 'single', the interrupt is generated after a single transfer. - If the descriptor type is '1D' or '2D', the interrupt is generated after the execution of a 1D transfer. - If the descriptor type is 'memory copy', the interrupt is generated after the execution of a memory copy transfer. - If the descriptor type is 'scatter' the interrupt is generated after the execution of a scatter transfer. '2': An interrupt is generated after the execution of the current descriptor (independent of the value of DESCR_NEXT_PTR.ADDR of the current descriptor). '3': An interrupt is generated after the execution of the current descriptor and the current descriptor's DESCR_NEXT_PTR.ADDR is '0'."]
pub type INTR_TYPE_R = crate::FieldReader;
#[doc = "Field `TR_OUT_TYPE` reader - Specifies when an output trigger is generated: '0': An output trigger is generated after a single transfer. '1': An output trigger is generated after a single 1D transfer or a memory copy transfer. - If the descriptor type is 'single', the output trigger is generated after a single transfer. - If the descriptor type is '1D' or '2D', the output trigger is generated after the execution of a 1D transfer. - If the descriptor type is 'memory copy', the output trigger is generated after the execution of a memory copy transfer. - If the descriptor type is 'scatter', the output trigger is generated after the execution of a scatter transfer. '2': An output trigger is generated after the execution of the current descriptor. '3': An output trigger is generated after the execution of a descriptor list: after the execution of the current descriptor AND the current descriptor's DESCR_NEXT_PTR.ADDR is '0'."]
pub type TR_OUT_TYPE_R = crate::FieldReader;
#[doc = "Field `TR_IN_TYPE` reader - Specifies the input trigger type (not to be confused with the descriptor type): '0': A trigger results in the execution of a single transfer. The descriptor type can be single, 1D or 2D. '1': A trigger results in the execution of a single 1D transfer. - If the descriptor type is 'single', the trigger results in the execution of a single transfer. - If the descriptor type is '1D' or '2D', the trigger results in the execution of a 1D transfer. - If the descriptor type is 'memory copy', the trigger results in the execution of a memory copy transfer. - If the descriptor type is 'scatter', the trigger results in the execution of an scatter transfer. '2': A trigger results in the execution of the current descriptor. '3': A trigger results in the execution of the current descriptor and continues (without requiring another input trigger) with the execution of the next descriptor using the next descriptor's information."]
pub type TR_IN_TYPE_R = crate::FieldReader;
#[doc = "Field `DATA_PREFETCH` reader - Source data prefetch: '0': No source data prefetch. Source data transfers are only initiated AFTER the input trigger is activated. '1': Source data prefetch. Source data transfers are initiated as soon as the channel is enabled, the current descriptor pointer is NOT '0' and there is space available in the channel's data FIFO. When the input trigger is activated, the trigger can initiate destination data transfers with data that is already in the channel's data FIFO. This effectively shortens the initial delay of the data transfer. Note: data prefetch should be used with care, to ensure that data coherency is guaranteed and that prefetches do not cause undesired side effects."]
pub type DATA_PREFETCH_R = crate::BitReader;
#[doc = "Field `DATA_SIZE` reader - Specifies the data element size: '0': Byte (8 bits). '1': Halfword (16 bits). '2': Word (32 bits). DATA_SIZE, SRC_TRANSFER_SIZE and DST_TRANSFER_SIZE together determine how data elements are transferred. The following are the 9 legal settings: - DATA is 8 bit, SRC is 8 bit, DST is 8 bit. - DATA is 8 bit, SRC is 32 bit (higher 24 bits are dropped), DST is 8 bit. - DATA is 8 bit, SRC is 8 bit, DST is 32 bit (higher 24 bits are made '0'). - DATA is 8 bit, SRC is 32 bit (higher 24 bits are dropped), DST is 32 bit (higher 24 bits are made '0'). - DATA is 16 bit, SRC is 16 bit, DST is 16 bit. - DATA is 16 bit, SRC is 32 bit (higher 16 bits are dropped), DST is 16 bit. - DATA is 16 bit, SRC is 16 bit, DST is 32 bit (higher 16 bits are made '0'). - DATA is 16 bit, SRC is 32 bit (higher 16 bits are dropped), DST is 32 bit (higher 16 bits are made '0'). - DATA is 32 bit, SRC is 32 bit, DST is 32 bit. Note: this field is not used for a 'memory copy' descriptor type. Note: this field must be set to '2' for a 'initialization' descriptor type."]
pub type DATA_SIZE_R = crate::FieldReader;
#[doc = "Field `CH_DISABLE` reader - Specifies whether the channel is disabled or not after completion of the current descriptor (independent of the value of the DESCR_NEXT_PTR value): '0': Channel is not disabled. '1': Channel is disabled."]
pub type CH_DISABLE_R = crate::BitReader;
#[doc = "Field `SRC_TRANSFER_SIZE` reader - Specifies the bus transfer size to the source location: '0': As specified by DATA_SIZE. '1': Word (32 bits). Distinguishing bus transfer size from data element size allows for source components with data elements that are smaller than their 32-bit bus interface width. E.g., an ADC source has a 32-bit bus transfer size, but only provides a 16-bit data element. Note: this field is not used for a 'memory copy' descriptor type. Note: this field must be set to '1' for a 'scatter' descriptor type."]
pub type SRC_TRANSFER_SIZE_R = crate::BitReader;
#[doc = "Field `DST_TRANSFER_SIZE` reader - Specifies the bus transfer size to the destination location: '0': As specified by DATA_SIZE. '1': Word (32 bits). Distinguishing bus transfer size from data element size allows for destination components with data elements that are smaller than their 32-bit bus interface width. E.g., a DAC destination has a 32-bit bus transfer size, but only requires a 16-bit data element. Note: this field is not used for a 'memory copy' descriptor type. Note: this field must be set to '1' for a 'scatter' descriptor type."]
pub type DST_TRANSFER_SIZE_R = crate::BitReader;
#[doc = "Field `DESCR_TYPE` reader - Specifies the descriptor type (not to be confused with the trigger type): '0': Single transfer. The DESCR_X_SIZE, DESCR_X_INCR, DESCR_Y_SIZE and DESCR_Y_INCR registers are NOT present. The DESCR_NEXT_PTR is at offset 0x0c. '1': 1D transfer. The DESCR_X_SIZE and DESCR_X_INCR registers are present, the DESCR_Y_SIZE and DESCR_Y_INCR are NOT present. A 1D transfer consists out of DESCR_X_SIZE.X_COUNT+1 single transfers. The DESCR_NEXT_PTR is at offset 0x14. '2': 2D transfer. The DESCR_X_SIZE, DESCR_X_INCR, DESCR_Y_SIZE and DESCR_Y_INCR registers are present. A 2D transfer consists of (DESCR_X_SIZE.X_COUNT+1)*(DESCR_Y_SIZE.Y_COUNT+1) single transfers. The DESCR_NEXT_PTR is at offset 0x1c. '3': Memory copy. The DESCR_X_SIZE register is present, the DESCR_X_INCR, DESCR_Y_SIZE and DESCR_Y_INCR are NOT present. A memory copy transfer copies DESCR_X_SIZE.X_COUNT+1 Bytes and may use Byte, halfword and word transfers. The DESCR_NEXT_PTR is at offset 0x10. '4': Scatter transfer. The DESCR_X_SIZE register is present, the DESCR_DST, DESCR_X_INCR, DESCR_Y_SIZE and DESCR_Y_INCR are NOT present. '5'-'7': Undefined. After the execution of the current descriptor, the DESCR_NEXT_PTR address is copied to the channel's CH_CURR_PTR address and CH_STATUS.X_IDX and CH_STATUS.Y_IDX are set to '0'."]
pub type DESCR_TYPE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Specifies whether the controller should wait for the input trigger to be deactivated; i.e. the selected system trigger is not active. This field is used to synchronize the controller with the agent that generated the trigger. This field is ONLY used at the completion of the transfer as specified by TR_IN. E.g., a TX FIFO indicates that it is empty and it needs a new data sample. The agent removes the trigger ONLY when the data sample has been written by the controller AND received by the agent. Furthermore, the agent's trigger may be delayed by a few cycles before it reaches the controller. This field is used for level sensitive trigger, which reflect state (pulse sensitive triggers should have this field set to '0'). The wait cycles incurred by this field reduce DW controller performance. '0': Do not wait for trigger de-activation (for pulse sensitive triggers). '1': Wait for up to 4 cycles. '2': Wait for up to 16 cycles. '3': Wait indefinitely. This option may result in controller lockup if the trigger is not de-activated."]
    #[inline(always)]
    pub fn wait_for_deact(&self) -> WAIT_FOR_DEACT_R {
        WAIT_FOR_DEACT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Specifies when a completion interrupt is generated (CH_STATUS.INTR_CAUSE is set to COMPLETION): '0': An interrupt is generated after a single transfer. '1': An interrupt is generated after a single 1D transfer or a memory copy transfer - If the descriptor type is 'single', the interrupt is generated after a single transfer. - If the descriptor type is '1D' or '2D', the interrupt is generated after the execution of a 1D transfer. - If the descriptor type is 'memory copy', the interrupt is generated after the execution of a memory copy transfer. - If the descriptor type is 'scatter' the interrupt is generated after the execution of a scatter transfer. '2': An interrupt is generated after the execution of the current descriptor (independent of the value of DESCR_NEXT_PTR.ADDR of the current descriptor). '3': An interrupt is generated after the execution of the current descriptor and the current descriptor's DESCR_NEXT_PTR.ADDR is '0'."]
    #[inline(always)]
    pub fn intr_type(&self) -> INTR_TYPE_R {
        INTR_TYPE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Specifies when an output trigger is generated: '0': An output trigger is generated after a single transfer. '1': An output trigger is generated after a single 1D transfer or a memory copy transfer. - If the descriptor type is 'single', the output trigger is generated after a single transfer. - If the descriptor type is '1D' or '2D', the output trigger is generated after the execution of a 1D transfer. - If the descriptor type is 'memory copy', the output trigger is generated after the execution of a memory copy transfer. - If the descriptor type is 'scatter', the output trigger is generated after the execution of a scatter transfer. '2': An output trigger is generated after the execution of the current descriptor. '3': An output trigger is generated after the execution of a descriptor list: after the execution of the current descriptor AND the current descriptor's DESCR_NEXT_PTR.ADDR is '0'."]
    #[inline(always)]
    pub fn tr_out_type(&self) -> TR_OUT_TYPE_R {
        TR_OUT_TYPE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Specifies the input trigger type (not to be confused with the descriptor type): '0': A trigger results in the execution of a single transfer. The descriptor type can be single, 1D or 2D. '1': A trigger results in the execution of a single 1D transfer. - If the descriptor type is 'single', the trigger results in the execution of a single transfer. - If the descriptor type is '1D' or '2D', the trigger results in the execution of a 1D transfer. - If the descriptor type is 'memory copy', the trigger results in the execution of a memory copy transfer. - If the descriptor type is 'scatter', the trigger results in the execution of an scatter transfer. '2': A trigger results in the execution of the current descriptor. '3': A trigger results in the execution of the current descriptor and continues (without requiring another input trigger) with the execution of the next descriptor using the next descriptor's information."]
    #[inline(always)]
    pub fn tr_in_type(&self) -> TR_IN_TYPE_R {
        TR_IN_TYPE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Source data prefetch: '0': No source data prefetch. Source data transfers are only initiated AFTER the input trigger is activated. '1': Source data prefetch. Source data transfers are initiated as soon as the channel is enabled, the current descriptor pointer is NOT '0' and there is space available in the channel's data FIFO. When the input trigger is activated, the trigger can initiate destination data transfers with data that is already in the channel's data FIFO. This effectively shortens the initial delay of the data transfer. Note: data prefetch should be used with care, to ensure that data coherency is guaranteed and that prefetches do not cause undesired side effects."]
    #[inline(always)]
    pub fn data_prefetch(&self) -> DATA_PREFETCH_R {
        DATA_PREFETCH_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Specifies the data element size: '0': Byte (8 bits). '1': Halfword (16 bits). '2': Word (32 bits). DATA_SIZE, SRC_TRANSFER_SIZE and DST_TRANSFER_SIZE together determine how data elements are transferred. The following are the 9 legal settings: - DATA is 8 bit, SRC is 8 bit, DST is 8 bit. - DATA is 8 bit, SRC is 32 bit (higher 24 bits are dropped), DST is 8 bit. - DATA is 8 bit, SRC is 8 bit, DST is 32 bit (higher 24 bits are made '0'). - DATA is 8 bit, SRC is 32 bit (higher 24 bits are dropped), DST is 32 bit (higher 24 bits are made '0'). - DATA is 16 bit, SRC is 16 bit, DST is 16 bit. - DATA is 16 bit, SRC is 32 bit (higher 16 bits are dropped), DST is 16 bit. - DATA is 16 bit, SRC is 16 bit, DST is 32 bit (higher 16 bits are made '0'). - DATA is 16 bit, SRC is 32 bit (higher 16 bits are dropped), DST is 32 bit (higher 16 bits are made '0'). - DATA is 32 bit, SRC is 32 bit, DST is 32 bit. Note: this field is not used for a 'memory copy' descriptor type. Note: this field must be set to '2' for a 'initialization' descriptor type."]
    #[inline(always)]
    pub fn data_size(&self) -> DATA_SIZE_R {
        DATA_SIZE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - Specifies whether the channel is disabled or not after completion of the current descriptor (independent of the value of the DESCR_NEXT_PTR value): '0': Channel is not disabled. '1': Channel is disabled."]
    #[inline(always)]
    pub fn ch_disable(&self) -> CH_DISABLE_R {
        CH_DISABLE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Specifies the bus transfer size to the source location: '0': As specified by DATA_SIZE. '1': Word (32 bits). Distinguishing bus transfer size from data element size allows for source components with data elements that are smaller than their 32-bit bus interface width. E.g., an ADC source has a 32-bit bus transfer size, but only provides a 16-bit data element. Note: this field is not used for a 'memory copy' descriptor type. Note: this field must be set to '1' for a 'scatter' descriptor type."]
    #[inline(always)]
    pub fn src_transfer_size(&self) -> SRC_TRANSFER_SIZE_R {
        SRC_TRANSFER_SIZE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Specifies the bus transfer size to the destination location: '0': As specified by DATA_SIZE. '1': Word (32 bits). Distinguishing bus transfer size from data element size allows for destination components with data elements that are smaller than their 32-bit bus interface width. E.g., a DAC destination has a 32-bit bus transfer size, but only requires a 16-bit data element. Note: this field is not used for a 'memory copy' descriptor type. Note: this field must be set to '1' for a 'scatter' descriptor type."]
    #[inline(always)]
    pub fn dst_transfer_size(&self) -> DST_TRANSFER_SIZE_R {
        DST_TRANSFER_SIZE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Specifies the descriptor type (not to be confused with the trigger type): '0': Single transfer. The DESCR_X_SIZE, DESCR_X_INCR, DESCR_Y_SIZE and DESCR_Y_INCR registers are NOT present. The DESCR_NEXT_PTR is at offset 0x0c. '1': 1D transfer. The DESCR_X_SIZE and DESCR_X_INCR registers are present, the DESCR_Y_SIZE and DESCR_Y_INCR are NOT present. A 1D transfer consists out of DESCR_X_SIZE.X_COUNT+1 single transfers. The DESCR_NEXT_PTR is at offset 0x14. '2': 2D transfer. The DESCR_X_SIZE, DESCR_X_INCR, DESCR_Y_SIZE and DESCR_Y_INCR registers are present. A 2D transfer consists of (DESCR_X_SIZE.X_COUNT+1)*(DESCR_Y_SIZE.Y_COUNT+1) single transfers. The DESCR_NEXT_PTR is at offset 0x1c. '3': Memory copy. The DESCR_X_SIZE register is present, the DESCR_X_INCR, DESCR_Y_SIZE and DESCR_Y_INCR are NOT present. A memory copy transfer copies DESCR_X_SIZE.X_COUNT+1 Bytes and may use Byte, halfword and word transfers. The DESCR_NEXT_PTR is at offset 0x10. '4': Scatter transfer. The DESCR_X_SIZE register is present, the DESCR_DST, DESCR_X_INCR, DESCR_Y_SIZE and DESCR_Y_INCR are NOT present. '5'-'7': Undefined. After the execution of the current descriptor, the DESCR_NEXT_PTR address is copied to the channel's CH_CURR_PTR address and CH_STATUS.X_IDX and CH_STATUS.Y_IDX are set to '0'."]
    #[inline(always)]
    pub fn descr_type(&self) -> DESCR_TYPE_R {
        DESCR_TYPE_R::new(((self.bits >> 28) & 7) as u8)
    }
}
#[doc = "Channel descriptor control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [descr_ctl](index.html) module"]
pub struct DESCR_CTL_SPEC;
impl crate::RegisterSpec for DESCR_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [descr_ctl::R](R) reader structure"]
impl crate::Readable for DESCR_CTL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DESCR_CTL to value 0"]
impl crate::Resettable for DESCR_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
