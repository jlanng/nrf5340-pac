#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start comparator"]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stop comparator"]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x08 - Sample comparator value"]
    pub tasks_sample: TASKS_SAMPLE,
    _reserved3: [u8; 116usize],
    #[doc = "0x80 - Subscribe configuration for task START"]
    pub subscribe_start: SUBSCRIBE_START,
    #[doc = "0x84 - Subscribe configuration for task STOP"]
    pub subscribe_stop: SUBSCRIBE_STOP,
    #[doc = "0x88 - Subscribe configuration for task SAMPLE"]
    pub subscribe_sample: SUBSCRIBE_SAMPLE,
    _reserved6: [u8; 116usize],
    #[doc = "0x100 - LPCOMP is ready and output is valid"]
    pub events_ready: EVENTS_READY,
    #[doc = "0x104 - Downward crossing"]
    pub events_down: EVENTS_DOWN,
    #[doc = "0x108 - Upward crossing"]
    pub events_up: EVENTS_UP,
    #[doc = "0x10c - Downward or upward crossing"]
    pub events_cross: EVENTS_CROSS,
    _reserved10: [u8; 112usize],
    #[doc = "0x180 - Publish configuration for event READY"]
    pub publish_ready: PUBLISH_READY,
    #[doc = "0x184 - Publish configuration for event DOWN"]
    pub publish_down: PUBLISH_DOWN,
    #[doc = "0x188 - Publish configuration for event UP"]
    pub publish_up: PUBLISH_UP,
    #[doc = "0x18c - Publish configuration for event CROSS"]
    pub publish_cross: PUBLISH_CROSS,
    _reserved14: [u8; 112usize],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: SHORTS,
    _reserved15: [u8; 256usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved17: [u8; 244usize],
    #[doc = "0x400 - Compare result"]
    pub result: RESULT,
    _reserved18: [u8; 252usize],
    #[doc = "0x500 - Enable LPCOMP"]
    pub enable: ENABLE,
    #[doc = "0x504 - Input pin select"]
    pub psel: PSEL,
    #[doc = "0x508 - Reference select"]
    pub refsel: REFSEL,
    #[doc = "0x50c - External reference select"]
    pub extrefsel: EXTREFSEL,
    _reserved22: [u8; 16usize],
    #[doc = "0x520 - Analog detect configuration"]
    pub anadetect: ANADETECT,
    _reserved23: [u8; 20usize],
    #[doc = "0x538 - Comparator hysteresis enable"]
    pub hyst: HYST,
}
#[doc = "Start comparator\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_start](tasks_start) module"]
pub type TASKS_START = crate::Reg<u32, _TASKS_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_START;
#[doc = "`write(|w| ..)` method takes [tasks_start::W](tasks_start::W) writer structure"]
impl crate::Writable for TASKS_START {}
#[doc = "Start comparator"]
pub mod tasks_start;
#[doc = "Stop comparator\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_stop](tasks_stop) module"]
pub type TASKS_STOP = crate::Reg<u32, _TASKS_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STOP;
#[doc = "`write(|w| ..)` method takes [tasks_stop::W](tasks_stop::W) writer structure"]
impl crate::Writable for TASKS_STOP {}
#[doc = "Stop comparator"]
pub mod tasks_stop;
#[doc = "Sample comparator value\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_sample](tasks_sample) module"]
pub type TASKS_SAMPLE = crate::Reg<u32, _TASKS_SAMPLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_SAMPLE;
#[doc = "`write(|w| ..)` method takes [tasks_sample::W](tasks_sample::W) writer structure"]
impl crate::Writable for TASKS_SAMPLE {}
#[doc = "Sample comparator value"]
pub mod tasks_sample;
#[doc = "Subscribe configuration for task START\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_start](subscribe_start) module"]
pub type SUBSCRIBE_START = crate::Reg<u32, _SUBSCRIBE_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_START;
#[doc = "`read()` method returns [subscribe_start::R](subscribe_start::R) reader structure"]
impl crate::Readable for SUBSCRIBE_START {}
#[doc = "`write(|w| ..)` method takes [subscribe_start::W](subscribe_start::W) writer structure"]
impl crate::Writable for SUBSCRIBE_START {}
#[doc = "Subscribe configuration for task START"]
pub mod subscribe_start;
#[doc = "Subscribe configuration for task STOP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_stop](subscribe_stop) module"]
pub type SUBSCRIBE_STOP = crate::Reg<u32, _SUBSCRIBE_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_STOP;
#[doc = "`read()` method returns [subscribe_stop::R](subscribe_stop::R) reader structure"]
impl crate::Readable for SUBSCRIBE_STOP {}
#[doc = "`write(|w| ..)` method takes [subscribe_stop::W](subscribe_stop::W) writer structure"]
impl crate::Writable for SUBSCRIBE_STOP {}
#[doc = "Subscribe configuration for task STOP"]
pub mod subscribe_stop;
#[doc = "Subscribe configuration for task SAMPLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_sample](subscribe_sample) module"]
pub type SUBSCRIBE_SAMPLE = crate::Reg<u32, _SUBSCRIBE_SAMPLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_SAMPLE;
#[doc = "`read()` method returns [subscribe_sample::R](subscribe_sample::R) reader structure"]
impl crate::Readable for SUBSCRIBE_SAMPLE {}
#[doc = "`write(|w| ..)` method takes [subscribe_sample::W](subscribe_sample::W) writer structure"]
impl crate::Writable for SUBSCRIBE_SAMPLE {}
#[doc = "Subscribe configuration for task SAMPLE"]
pub mod subscribe_sample;
#[doc = "LPCOMP is ready and output is valid\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_ready](events_ready) module"]
pub type EVENTS_READY = crate::Reg<u32, _EVENTS_READY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_READY;
#[doc = "`read()` method returns [events_ready::R](events_ready::R) reader structure"]
impl crate::Readable for EVENTS_READY {}
#[doc = "`write(|w| ..)` method takes [events_ready::W](events_ready::W) writer structure"]
impl crate::Writable for EVENTS_READY {}
#[doc = "LPCOMP is ready and output is valid"]
pub mod events_ready;
#[doc = "Downward crossing\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_down](events_down) module"]
pub type EVENTS_DOWN = crate::Reg<u32, _EVENTS_DOWN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_DOWN;
#[doc = "`read()` method returns [events_down::R](events_down::R) reader structure"]
impl crate::Readable for EVENTS_DOWN {}
#[doc = "`write(|w| ..)` method takes [events_down::W](events_down::W) writer structure"]
impl crate::Writable for EVENTS_DOWN {}
#[doc = "Downward crossing"]
pub mod events_down;
#[doc = "Upward crossing\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_up](events_up) module"]
pub type EVENTS_UP = crate::Reg<u32, _EVENTS_UP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_UP;
#[doc = "`read()` method returns [events_up::R](events_up::R) reader structure"]
impl crate::Readable for EVENTS_UP {}
#[doc = "`write(|w| ..)` method takes [events_up::W](events_up::W) writer structure"]
impl crate::Writable for EVENTS_UP {}
#[doc = "Upward crossing"]
pub mod events_up;
#[doc = "Downward or upward crossing\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_cross](events_cross) module"]
pub type EVENTS_CROSS = crate::Reg<u32, _EVENTS_CROSS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_CROSS;
#[doc = "`read()` method returns [events_cross::R](events_cross::R) reader structure"]
impl crate::Readable for EVENTS_CROSS {}
#[doc = "`write(|w| ..)` method takes [events_cross::W](events_cross::W) writer structure"]
impl crate::Writable for EVENTS_CROSS {}
#[doc = "Downward or upward crossing"]
pub mod events_cross;
#[doc = "Publish configuration for event READY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_ready](publish_ready) module"]
pub type PUBLISH_READY = crate::Reg<u32, _PUBLISH_READY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_READY;
#[doc = "`read()` method returns [publish_ready::R](publish_ready::R) reader structure"]
impl crate::Readable for PUBLISH_READY {}
#[doc = "`write(|w| ..)` method takes [publish_ready::W](publish_ready::W) writer structure"]
impl crate::Writable for PUBLISH_READY {}
#[doc = "Publish configuration for event READY"]
pub mod publish_ready;
#[doc = "Publish configuration for event DOWN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_down](publish_down) module"]
pub type PUBLISH_DOWN = crate::Reg<u32, _PUBLISH_DOWN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_DOWN;
#[doc = "`read()` method returns [publish_down::R](publish_down::R) reader structure"]
impl crate::Readable for PUBLISH_DOWN {}
#[doc = "`write(|w| ..)` method takes [publish_down::W](publish_down::W) writer structure"]
impl crate::Writable for PUBLISH_DOWN {}
#[doc = "Publish configuration for event DOWN"]
pub mod publish_down;
#[doc = "Publish configuration for event UP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_up](publish_up) module"]
pub type PUBLISH_UP = crate::Reg<u32, _PUBLISH_UP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_UP;
#[doc = "`read()` method returns [publish_up::R](publish_up::R) reader structure"]
impl crate::Readable for PUBLISH_UP {}
#[doc = "`write(|w| ..)` method takes [publish_up::W](publish_up::W) writer structure"]
impl crate::Writable for PUBLISH_UP {}
#[doc = "Publish configuration for event UP"]
pub mod publish_up;
#[doc = "Publish configuration for event CROSS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_cross](publish_cross) module"]
pub type PUBLISH_CROSS = crate::Reg<u32, _PUBLISH_CROSS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_CROSS;
#[doc = "`read()` method returns [publish_cross::R](publish_cross::R) reader structure"]
impl crate::Readable for PUBLISH_CROSS {}
#[doc = "`write(|w| ..)` method takes [publish_cross::W](publish_cross::W) writer structure"]
impl crate::Writable for PUBLISH_CROSS {}
#[doc = "Publish configuration for event CROSS"]
pub mod publish_cross;
#[doc = "Shortcuts between local events and tasks\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shorts](shorts) module"]
pub type SHORTS = crate::Reg<u32, _SHORTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHORTS;
#[doc = "`read()` method returns [shorts::R](shorts::R) reader structure"]
impl crate::Readable for SHORTS {}
#[doc = "`write(|w| ..)` method takes [shorts::W](shorts::W) writer structure"]
impl crate::Writable for SHORTS {}
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "Enable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u32, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "Disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u32, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "Compare result\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [result](result) module"]
pub type RESULT = crate::Reg<u32, _RESULT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESULT;
#[doc = "`read()` method returns [result::R](result::R) reader structure"]
impl crate::Readable for RESULT {}
#[doc = "Compare result"]
pub mod result;
#[doc = "Enable LPCOMP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "Enable LPCOMP"]
pub mod enable;
#[doc = "Input pin select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [psel](psel) module"]
pub type PSEL = crate::Reg<u32, _PSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSEL;
#[doc = "`read()` method returns [psel::R](psel::R) reader structure"]
impl crate::Readable for PSEL {}
#[doc = "`write(|w| ..)` method takes [psel::W](psel::W) writer structure"]
impl crate::Writable for PSEL {}
#[doc = "Input pin select"]
pub mod psel;
#[doc = "Reference select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [refsel](refsel) module"]
pub type REFSEL = crate::Reg<u32, _REFSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFSEL;
#[doc = "`read()` method returns [refsel::R](refsel::R) reader structure"]
impl crate::Readable for REFSEL {}
#[doc = "`write(|w| ..)` method takes [refsel::W](refsel::W) writer structure"]
impl crate::Writable for REFSEL {}
#[doc = "Reference select"]
pub mod refsel;
#[doc = "External reference select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [extrefsel](extrefsel) module"]
pub type EXTREFSEL = crate::Reg<u32, _EXTREFSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTREFSEL;
#[doc = "`read()` method returns [extrefsel::R](extrefsel::R) reader structure"]
impl crate::Readable for EXTREFSEL {}
#[doc = "`write(|w| ..)` method takes [extrefsel::W](extrefsel::W) writer structure"]
impl crate::Writable for EXTREFSEL {}
#[doc = "External reference select"]
pub mod extrefsel;
#[doc = "Analog detect configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [anadetect](anadetect) module"]
pub type ANADETECT = crate::Reg<u32, _ANADETECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANADETECT;
#[doc = "`read()` method returns [anadetect::R](anadetect::R) reader structure"]
impl crate::Readable for ANADETECT {}
#[doc = "`write(|w| ..)` method takes [anadetect::W](anadetect::W) writer structure"]
impl crate::Writable for ANADETECT {}
#[doc = "Analog detect configuration"]
pub mod anadetect;
#[doc = "Comparator hysteresis enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hyst](hyst) module"]
pub type HYST = crate::Reg<u32, _HYST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HYST;
#[doc = "`read()` method returns [hyst::R](hyst::R) reader structure"]
impl crate::Readable for HYST {}
#[doc = "`write(|w| ..)` method takes [hyst::W](hyst::W) writer structure"]
impl crate::Writable for HYST {}
#[doc = "Comparator hysteresis enable"]
pub mod hyst;