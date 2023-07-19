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
#[doc = "Field `AUTO_RELOAD_CC` reader - Specifies switching of the CC and buffered CC values. This field has a function in TIMER, PWM, PWM_DT and PWM_PR modes. Timer mode: '0': never switch. '1': switch on a compare match event. PWM, PWM_DT, PWM_PR modes: '0: never switch. '1': switch on a terminal count event with an actively pending switch event."]
pub type AUTO_RELOAD_CC_R = crate::BitReader;
#[doc = "Field `AUTO_RELOAD_CC` writer - Specifies switching of the CC and buffered CC values. This field has a function in TIMER, PWM, PWM_DT and PWM_PR modes. Timer mode: '0': never switch. '1': switch on a compare match event. PWM, PWM_DT, PWM_PR modes: '0: never switch. '1': switch on a terminal count event with an actively pending switch event."]
pub type AUTO_RELOAD_CC_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `AUTO_RELOAD_PERIOD` reader - Specifies switching of the PERIOD and buffered PERIOD values. This field has a function in PWM, PWM_DT and PWM_PR modes. '0': never switch. '1': switch on a terminal count event with and actively pending switch event."]
pub type AUTO_RELOAD_PERIOD_R = crate::BitReader;
#[doc = "Field `AUTO_RELOAD_PERIOD` writer - Specifies switching of the PERIOD and buffered PERIOD values. This field has a function in PWM, PWM_DT and PWM_PR modes. '0': never switch. '1': switch on a terminal count event with and actively pending switch event."]
pub type AUTO_RELOAD_PERIOD_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `PWM_SYNC_KILL` reader - Specifies asynchronous/synchronous kill behavior: '1': synchronous kill mode: the kill event disables the 'dt_line_out' and 'dt_line_compl_out' signals till the next terminal count event (synchronous kill). In synchronous kill mode, STOP_EDGE should be RISING_EDGE. '0': asynchronous kill mode: the kill event only disables the 'dt_line_out' and 'dt_line_compl_out' signals when present. In asynchronous kill mode, STOP_EDGE should be NO_EDGE_DET. This field has a function in PWM and PWM_DT modes only. This field is only used when PWM_STOP_ON_KILL is '0'."]
pub type PWM_SYNC_KILL_R = crate::BitReader;
#[doc = "Field `PWM_SYNC_KILL` writer - Specifies asynchronous/synchronous kill behavior: '1': synchronous kill mode: the kill event disables the 'dt_line_out' and 'dt_line_compl_out' signals till the next terminal count event (synchronous kill). In synchronous kill mode, STOP_EDGE should be RISING_EDGE. '0': asynchronous kill mode: the kill event only disables the 'dt_line_out' and 'dt_line_compl_out' signals when present. In asynchronous kill mode, STOP_EDGE should be NO_EDGE_DET. This field has a function in PWM and PWM_DT modes only. This field is only used when PWM_STOP_ON_KILL is '0'."]
pub type PWM_SYNC_KILL_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `PWM_STOP_ON_KILL` reader - Specifies whether the counter stops on a kill events: '0': kill event does NOT stop counter. '1': kill event stops counter. This field has a function in PWM, PWM_DT and PWM_PR modes only."]
pub type PWM_STOP_ON_KILL_R = crate::BitReader;
#[doc = "Field `PWM_STOP_ON_KILL` writer - Specifies whether the counter stops on a kill events: '0': kill event does NOT stop counter. '1': kill event stops counter. This field has a function in PWM, PWM_DT and PWM_PR modes only."]
pub type PWM_STOP_ON_KILL_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `GENERIC` reader - Generic 8-bit control field. In PWM_DT mode, this field is used to determine the dead time: amount of dead time cycles in the counter clock domain. In all other modes, the lower 3 bits of this field determine pre-scaling of the selected counter clock."]
pub type GENERIC_R = crate::FieldReader;
#[doc = "Field `GENERIC` writer - Generic 8-bit control field. In PWM_DT mode, this field is used to determine the dead time: amount of dead time cycles in the counter clock domain. In all other modes, the lower 3 bits of this field determine pre-scaling of the selected counter clock."]
pub type GENERIC_W<'a, const O: u8> = crate::FieldWriter<'a, CTRL_SPEC, 8, O>;
#[doc = "Field `UP_DOWN_MODE` reader - Determines counter direction."]
pub type UP_DOWN_MODE_R = crate::FieldReader<UP_DOWN_MODE_A>;
#[doc = "Determines counter direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UP_DOWN_MODE_A {
    #[doc = "0: Count up (to PERIOD). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. A terminal count event is generated when the counter changes from a state in which COUNTER equals PERIOD."]
    COUNT_UP = 0,
    #[doc = "1: Count down (to '0'). An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0'."]
    COUNT_DOWN = 1,
    #[doc = "2: Count up (to PERIOD), then count down (to '0'). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0'."]
    COUNT_UPDN1 = 2,
    #[doc = "3: Count up (to PERIOD), then count down (to '0'). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0' AND when the counter changes from a state in which COUNTER equals PERIOD (this counter direction can be used for PWM functionality with asymmetrical updates)."]
    COUNT_UPDN2 = 3,
}
impl From<UP_DOWN_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: UP_DOWN_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UP_DOWN_MODE_A {
    type Ux = u8;
}
impl UP_DOWN_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UP_DOWN_MODE_A {
        match self.bits {
            0 => UP_DOWN_MODE_A::COUNT_UP,
            1 => UP_DOWN_MODE_A::COUNT_DOWN,
            2 => UP_DOWN_MODE_A::COUNT_UPDN1,
            3 => UP_DOWN_MODE_A::COUNT_UPDN2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `COUNT_UP`"]
    #[inline(always)]
    pub fn is_count_up(&self) -> bool {
        *self == UP_DOWN_MODE_A::COUNT_UP
    }
    #[doc = "Checks if the value of the field is `COUNT_DOWN`"]
    #[inline(always)]
    pub fn is_count_down(&self) -> bool {
        *self == UP_DOWN_MODE_A::COUNT_DOWN
    }
    #[doc = "Checks if the value of the field is `COUNT_UPDN1`"]
    #[inline(always)]
    pub fn is_count_updn1(&self) -> bool {
        *self == UP_DOWN_MODE_A::COUNT_UPDN1
    }
    #[doc = "Checks if the value of the field is `COUNT_UPDN2`"]
    #[inline(always)]
    pub fn is_count_updn2(&self) -> bool {
        *self == UP_DOWN_MODE_A::COUNT_UPDN2
    }
}
#[doc = "Field `UP_DOWN_MODE` writer - Determines counter direction."]
pub type UP_DOWN_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, CTRL_SPEC, 2, O, UP_DOWN_MODE_A>;
impl<'a, const O: u8> UP_DOWN_MODE_W<'a, O> {
    #[doc = "Count up (to PERIOD). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. A terminal count event is generated when the counter changes from a state in which COUNTER equals PERIOD."]
    #[inline(always)]
    pub fn count_up(self) -> &'a mut W {
        self.variant(UP_DOWN_MODE_A::COUNT_UP)
    }
    #[doc = "Count down (to '0'). An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0'."]
    #[inline(always)]
    pub fn count_down(self) -> &'a mut W {
        self.variant(UP_DOWN_MODE_A::COUNT_DOWN)
    }
    #[doc = "Count up (to PERIOD), then count down (to '0'). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0'."]
    #[inline(always)]
    pub fn count_updn1(self) -> &'a mut W {
        self.variant(UP_DOWN_MODE_A::COUNT_UPDN1)
    }
    #[doc = "Count up (to PERIOD), then count down (to '0'). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0' AND when the counter changes from a state in which COUNTER equals PERIOD (this counter direction can be used for PWM functionality with asymmetrical updates)."]
    #[inline(always)]
    pub fn count_updn2(self) -> &'a mut W {
        self.variant(UP_DOWN_MODE_A::COUNT_UPDN2)
    }
}
#[doc = "Field `ONE_SHOT` reader - When '0', counter runs continuous. When '1', counter is turned off by hardware when a terminal count event is generated."]
pub type ONE_SHOT_R = crate::BitReader;
#[doc = "Field `ONE_SHOT` writer - When '0', counter runs continuous. When '1', counter is turned off by hardware when a terminal count event is generated."]
pub type ONE_SHOT_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `QUADRATURE_MODE` reader - In QUAD mode selects quadrature encoding mode (X1/X2/X4). In PWM, PWM_DT and PWM_PR modes, these two bits can be used to invert 'dt_line_out' and 'dt_line_compl_out'. Inversion is the last step in generation of 'dt_line_out' and 'dt_line_compl_out'; i.e. a disabled output line 'dt_line_out' has the value QUADRATURE_MODE\\[0\\]
and a disabled output line 'dt_line_compl_out' has the value QUADRATURE_MODE\\[1\\]."]
pub type QUADRATURE_MODE_R = crate::FieldReader<QUADRATURE_MODE_A>;
#[doc = "In QUAD mode selects quadrature encoding mode (X1/X2/X4). In PWM, PWM_DT and PWM_PR modes, these two bits can be used to invert 'dt_line_out' and 'dt_line_compl_out'. Inversion is the last step in generation of 'dt_line_out' and 'dt_line_compl_out'; i.e. a disabled output line 'dt_line_out' has the value QUADRATURE_MODE\\[0\\]
and a disabled output line 'dt_line_compl_out' has the value QUADRATURE_MODE\\[1\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum QUADRATURE_MODE_A {
    #[doc = "0: X1 encoding (QUAD mode)"]
    X1 = 0,
    #[doc = "1: X2 encoding (QUAD mode)"]
    X2 = 1,
    #[doc = "2: X4 encoding (QUAD mode)"]
    X4 = 2,
}
impl From<QUADRATURE_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: QUADRATURE_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for QUADRATURE_MODE_A {
    type Ux = u8;
}
impl QUADRATURE_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<QUADRATURE_MODE_A> {
        match self.bits {
            0 => Some(QUADRATURE_MODE_A::X1),
            1 => Some(QUADRATURE_MODE_A::X2),
            2 => Some(QUADRATURE_MODE_A::X4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `X1`"]
    #[inline(always)]
    pub fn is_x1(&self) -> bool {
        *self == QUADRATURE_MODE_A::X1
    }
    #[doc = "Checks if the value of the field is `X2`"]
    #[inline(always)]
    pub fn is_x2(&self) -> bool {
        *self == QUADRATURE_MODE_A::X2
    }
    #[doc = "Checks if the value of the field is `X4`"]
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == QUADRATURE_MODE_A::X4
    }
}
#[doc = "Field `QUADRATURE_MODE` writer - In QUAD mode selects quadrature encoding mode (X1/X2/X4). In PWM, PWM_DT and PWM_PR modes, these two bits can be used to invert 'dt_line_out' and 'dt_line_compl_out'. Inversion is the last step in generation of 'dt_line_out' and 'dt_line_compl_out'; i.e. a disabled output line 'dt_line_out' has the value QUADRATURE_MODE\\[0\\]
and a disabled output line 'dt_line_compl_out' has the value QUADRATURE_MODE\\[1\\]."]
pub type QUADRATURE_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, CTRL_SPEC, 2, O, QUADRATURE_MODE_A>;
impl<'a, const O: u8> QUADRATURE_MODE_W<'a, O> {
    #[doc = "X1 encoding (QUAD mode)"]
    #[inline(always)]
    pub fn x1(self) -> &'a mut W {
        self.variant(QUADRATURE_MODE_A::X1)
    }
    #[doc = "X2 encoding (QUAD mode)"]
    #[inline(always)]
    pub fn x2(self) -> &'a mut W {
        self.variant(QUADRATURE_MODE_A::X2)
    }
    #[doc = "X4 encoding (QUAD mode)"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut W {
        self.variant(QUADRATURE_MODE_A::X4)
    }
}
#[doc = "Field `MODE` reader - Counter mode."]
pub type MODE_R = crate::FieldReader<MODE_A>;
#[doc = "Counter mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Timer mode"]
    TIMER = 0,
    #[doc = "2: Capture mode"]
    CAPTURE = 2,
    #[doc = "3: Quadrature encoding mode"]
    QUAD = 3,
    #[doc = "4: Pulse width modulation (PWM) mode"]
    PWM = 4,
    #[doc = "5: PWM with deadtime insertion mode"]
    PWM_DT = 5,
    #[doc = "6: Pseudo random pulse width modulation"]
    PWM_PR = 6,
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
            0 => Some(MODE_A::TIMER),
            2 => Some(MODE_A::CAPTURE),
            3 => Some(MODE_A::QUAD),
            4 => Some(MODE_A::PWM),
            5 => Some(MODE_A::PWM_DT),
            6 => Some(MODE_A::PWM_PR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == MODE_A::TIMER
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == MODE_A::CAPTURE
    }
    #[doc = "Checks if the value of the field is `QUAD`"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == MODE_A::QUAD
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == MODE_A::PWM
    }
    #[doc = "Checks if the value of the field is `PWM_DT`"]
    #[inline(always)]
    pub fn is_pwm_dt(&self) -> bool {
        *self == MODE_A::PWM_DT
    }
    #[doc = "Checks if the value of the field is `PWM_PR`"]
    #[inline(always)]
    pub fn is_pwm_pr(&self) -> bool {
        *self == MODE_A::PWM_PR
    }
}
#[doc = "Field `MODE` writer - Counter mode."]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, CTRL_SPEC, 3, O, MODE_A>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Timer mode"]
    #[inline(always)]
    pub fn timer(self) -> &'a mut W {
        self.variant(MODE_A::TIMER)
    }
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(MODE_A::CAPTURE)
    }
    #[doc = "Quadrature encoding mode"]
    #[inline(always)]
    pub fn quad(self) -> &'a mut W {
        self.variant(MODE_A::QUAD)
    }
    #[doc = "Pulse width modulation (PWM) mode"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut W {
        self.variant(MODE_A::PWM)
    }
    #[doc = "PWM with deadtime insertion mode"]
    #[inline(always)]
    pub fn pwm_dt(self) -> &'a mut W {
        self.variant(MODE_A::PWM_DT)
    }
    #[doc = "Pseudo random pulse width modulation"]
    #[inline(always)]
    pub fn pwm_pr(self) -> &'a mut W {
        self.variant(MODE_A::PWM_PR)
    }
}
impl R {
    #[doc = "Bit 0 - Specifies switching of the CC and buffered CC values. This field has a function in TIMER, PWM, PWM_DT and PWM_PR modes. Timer mode: '0': never switch. '1': switch on a compare match event. PWM, PWM_DT, PWM_PR modes: '0: never switch. '1': switch on a terminal count event with an actively pending switch event."]
    #[inline(always)]
    pub fn auto_reload_cc(&self) -> AUTO_RELOAD_CC_R {
        AUTO_RELOAD_CC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Specifies switching of the PERIOD and buffered PERIOD values. This field has a function in PWM, PWM_DT and PWM_PR modes. '0': never switch. '1': switch on a terminal count event with and actively pending switch event."]
    #[inline(always)]
    pub fn auto_reload_period(&self) -> AUTO_RELOAD_PERIOD_R {
        AUTO_RELOAD_PERIOD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Specifies asynchronous/synchronous kill behavior: '1': synchronous kill mode: the kill event disables the 'dt_line_out' and 'dt_line_compl_out' signals till the next terminal count event (synchronous kill). In synchronous kill mode, STOP_EDGE should be RISING_EDGE. '0': asynchronous kill mode: the kill event only disables the 'dt_line_out' and 'dt_line_compl_out' signals when present. In asynchronous kill mode, STOP_EDGE should be NO_EDGE_DET. This field has a function in PWM and PWM_DT modes only. This field is only used when PWM_STOP_ON_KILL is '0'."]
    #[inline(always)]
    pub fn pwm_sync_kill(&self) -> PWM_SYNC_KILL_R {
        PWM_SYNC_KILL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Specifies whether the counter stops on a kill events: '0': kill event does NOT stop counter. '1': kill event stops counter. This field has a function in PWM, PWM_DT and PWM_PR modes only."]
    #[inline(always)]
    pub fn pwm_stop_on_kill(&self) -> PWM_STOP_ON_KILL_R {
        PWM_STOP_ON_KILL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Generic 8-bit control field. In PWM_DT mode, this field is used to determine the dead time: amount of dead time cycles in the counter clock domain. In all other modes, the lower 3 bits of this field determine pre-scaling of the selected counter clock."]
    #[inline(always)]
    pub fn generic(&self) -> GENERIC_R {
        GENERIC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Determines counter direction."]
    #[inline(always)]
    pub fn up_down_mode(&self) -> UP_DOWN_MODE_R {
        UP_DOWN_MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - When '0', counter runs continuous. When '1', counter is turned off by hardware when a terminal count event is generated."]
    #[inline(always)]
    pub fn one_shot(&self) -> ONE_SHOT_R {
        ONE_SHOT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:21 - In QUAD mode selects quadrature encoding mode (X1/X2/X4). In PWM, PWM_DT and PWM_PR modes, these two bits can be used to invert 'dt_line_out' and 'dt_line_compl_out'. Inversion is the last step in generation of 'dt_line_out' and 'dt_line_compl_out'; i.e. a disabled output line 'dt_line_out' has the value QUADRATURE_MODE\\[0\\]
and a disabled output line 'dt_line_compl_out' has the value QUADRATURE_MODE\\[1\\]."]
    #[inline(always)]
    pub fn quadrature_mode(&self) -> QUADRATURE_MODE_R {
        QUADRATURE_MODE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Counter mode."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies switching of the CC and buffered CC values. This field has a function in TIMER, PWM, PWM_DT and PWM_PR modes. Timer mode: '0': never switch. '1': switch on a compare match event. PWM, PWM_DT, PWM_PR modes: '0: never switch. '1': switch on a terminal count event with an actively pending switch event."]
    #[inline(always)]
    #[must_use]
    pub fn auto_reload_cc(&mut self) -> AUTO_RELOAD_CC_W<0> {
        AUTO_RELOAD_CC_W::new(self)
    }
    #[doc = "Bit 1 - Specifies switching of the PERIOD and buffered PERIOD values. This field has a function in PWM, PWM_DT and PWM_PR modes. '0': never switch. '1': switch on a terminal count event with and actively pending switch event."]
    #[inline(always)]
    #[must_use]
    pub fn auto_reload_period(&mut self) -> AUTO_RELOAD_PERIOD_W<1> {
        AUTO_RELOAD_PERIOD_W::new(self)
    }
    #[doc = "Bit 2 - Specifies asynchronous/synchronous kill behavior: '1': synchronous kill mode: the kill event disables the 'dt_line_out' and 'dt_line_compl_out' signals till the next terminal count event (synchronous kill). In synchronous kill mode, STOP_EDGE should be RISING_EDGE. '0': asynchronous kill mode: the kill event only disables the 'dt_line_out' and 'dt_line_compl_out' signals when present. In asynchronous kill mode, STOP_EDGE should be NO_EDGE_DET. This field has a function in PWM and PWM_DT modes only. This field is only used when PWM_STOP_ON_KILL is '0'."]
    #[inline(always)]
    #[must_use]
    pub fn pwm_sync_kill(&mut self) -> PWM_SYNC_KILL_W<2> {
        PWM_SYNC_KILL_W::new(self)
    }
    #[doc = "Bit 3 - Specifies whether the counter stops on a kill events: '0': kill event does NOT stop counter. '1': kill event stops counter. This field has a function in PWM, PWM_DT and PWM_PR modes only."]
    #[inline(always)]
    #[must_use]
    pub fn pwm_stop_on_kill(&mut self) -> PWM_STOP_ON_KILL_W<3> {
        PWM_STOP_ON_KILL_W::new(self)
    }
    #[doc = "Bits 8:15 - Generic 8-bit control field. In PWM_DT mode, this field is used to determine the dead time: amount of dead time cycles in the counter clock domain. In all other modes, the lower 3 bits of this field determine pre-scaling of the selected counter clock."]
    #[inline(always)]
    #[must_use]
    pub fn generic(&mut self) -> GENERIC_W<8> {
        GENERIC_W::new(self)
    }
    #[doc = "Bits 16:17 - Determines counter direction."]
    #[inline(always)]
    #[must_use]
    pub fn up_down_mode(&mut self) -> UP_DOWN_MODE_W<16> {
        UP_DOWN_MODE_W::new(self)
    }
    #[doc = "Bit 18 - When '0', counter runs continuous. When '1', counter is turned off by hardware when a terminal count event is generated."]
    #[inline(always)]
    #[must_use]
    pub fn one_shot(&mut self) -> ONE_SHOT_W<18> {
        ONE_SHOT_W::new(self)
    }
    #[doc = "Bits 20:21 - In QUAD mode selects quadrature encoding mode (X1/X2/X4). In PWM, PWM_DT and PWM_PR modes, these two bits can be used to invert 'dt_line_out' and 'dt_line_compl_out'. Inversion is the last step in generation of 'dt_line_out' and 'dt_line_compl_out'; i.e. a disabled output line 'dt_line_out' has the value QUADRATURE_MODE\\[0\\]
and a disabled output line 'dt_line_compl_out' has the value QUADRATURE_MODE\\[1\\]."]
    #[inline(always)]
    #[must_use]
    pub fn quadrature_mode(&mut self) -> QUADRATURE_MODE_W<20> {
        QUADRATURE_MODE_W::new(self)
    }
    #[doc = "Bits 24:26 - Counter mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<24> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
