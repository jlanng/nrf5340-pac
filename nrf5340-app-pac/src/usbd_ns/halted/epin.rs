#[doc = "Reader of register EPIN[%s]"]
pub type R = crate::R<u32, super::EPIN>;
#[doc = "IN endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GETSTATUS_A {
    #[doc = "0: Endpoint is not halted"]
    NOTHALTED,
    #[doc = "1: Endpoint is halted"]
    HALTED,
}
impl From<GETSTATUS_A> for u16 {
    #[inline(always)]
    fn from(variant: GETSTATUS_A) -> Self {
        match variant {
            GETSTATUS_A::NOTHALTED => 0,
            GETSTATUS_A::HALTED => 1,
        }
    }
}
#[doc = "Reader of field `GETSTATUS`"]
pub type GETSTATUS_R = crate::R<u16, GETSTATUS_A>;
impl GETSTATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, GETSTATUS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GETSTATUS_A::NOTHALTED),
            1 => Val(GETSTATUS_A::HALTED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOTHALTED`"]
    #[inline(always)]
    pub fn is_not_halted(&self) -> bool {
        *self == GETSTATUS_A::NOTHALTED
    }
    #[doc = "Checks if the value of the field is `HALTED`"]
    #[inline(always)]
    pub fn is_halted(&self) -> bool {
        *self == GETSTATUS_A::HALTED
    }
}
impl R {
    #[doc = "Bits 0:15 - IN endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
    #[inline(always)]
    pub fn getstatus(&self) -> GETSTATUS_R {
        GETSTATUS_R::new((self.bits & 0xffff) as u16)
    }
}
