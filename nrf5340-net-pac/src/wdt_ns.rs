#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start the watchdog"]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stop the watchdog timer."]
    pub tasks_stop: TASKS_STOP,
    _reserved2: [u8; 120usize],
    #[doc = "0x80 - Subscribe configuration for task START"]
    pub subscribe_start: SUBSCRIBE_START,
    #[doc = "0x84 - Subscribe configuration for task STOP"]
    pub subscribe_stop: SUBSCRIBE_STOP,
    _reserved4: [u8; 120usize],
    #[doc = "0x100 - Watchdog timeout"]
    pub events_timeout: EVENTS_TIMEOUT,
    #[doc = "0x104 - Watchdog stopped"]
    pub events_stopped: EVENTS_STOPPED,
    _reserved6: [u8; 120usize],
    #[doc = "0x180 - Publish configuration for event TIMEOUT"]
    pub publish_timeout: PUBLISH_TIMEOUT,
    #[doc = "0x184 - Publish configuration for event STOPPED"]
    pub publish_stopped: PUBLISH_STOPPED,
    _reserved8: [u8; 380usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved10: [u8; 24usize],
    #[doc = "0x324 - Enable interrupt"]
    pub nmienset: NMIENSET,
    #[doc = "0x328 - Disable interrupt"]
    pub nmienclr: NMIENCLR,
    _reserved12: [u8; 212usize],
    #[doc = "0x400 - Run status"]
    pub runstatus: RUNSTATUS,
    #[doc = "0x404 - Request status"]
    pub reqstatus: REQSTATUS,
    _reserved14: [u8; 252usize],
    #[doc = "0x504 - Counter reload value"]
    pub crv: CRV,
    #[doc = "0x508 - Enable register for reload request registers"]
    pub rren: RREN,
    #[doc = "0x50c - Configuration register"]
    pub config: CONFIG,
    _reserved17: [u8; 16usize],
    #[doc = "0x520 - Task Stop Enable"]
    pub tsen: TSEN,
    _reserved18: [u8; 220usize],
    #[doc = "0x600 - Description collection: Reload request n"]
    pub rr: [RR; 8],
}
#[doc = "Start the watchdog\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_start](tasks_start) module"]
pub type TASKS_START = crate::Reg<u32, _TASKS_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_START;
#[doc = "`write(|w| ..)` method takes [tasks_start::W](tasks_start::W) writer structure"]
impl crate::Writable for TASKS_START {}
#[doc = "Start the watchdog"]
pub mod tasks_start;
#[doc = "Stop the watchdog timer.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_stop](tasks_stop) module"]
pub type TASKS_STOP = crate::Reg<u32, _TASKS_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STOP;
#[doc = "`write(|w| ..)` method takes [tasks_stop::W](tasks_stop::W) writer structure"]
impl crate::Writable for TASKS_STOP {}
#[doc = "Stop the watchdog timer."]
pub mod tasks_stop;
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
#[doc = "Watchdog timeout\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_timeout](events_timeout) module"]
pub type EVENTS_TIMEOUT = crate::Reg<u32, _EVENTS_TIMEOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_TIMEOUT;
#[doc = "`read()` method returns [events_timeout::R](events_timeout::R) reader structure"]
impl crate::Readable for EVENTS_TIMEOUT {}
#[doc = "`write(|w| ..)` method takes [events_timeout::W](events_timeout::W) writer structure"]
impl crate::Writable for EVENTS_TIMEOUT {}
#[doc = "Watchdog timeout"]
pub mod events_timeout;
#[doc = "Watchdog stopped\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_stopped](events_stopped) module"]
pub type EVENTS_STOPPED = crate::Reg<u32, _EVENTS_STOPPED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_STOPPED;
#[doc = "`read()` method returns [events_stopped::R](events_stopped::R) reader structure"]
impl crate::Readable for EVENTS_STOPPED {}
#[doc = "`write(|w| ..)` method takes [events_stopped::W](events_stopped::W) writer structure"]
impl crate::Writable for EVENTS_STOPPED {}
#[doc = "Watchdog stopped"]
pub mod events_stopped;
#[doc = "Publish configuration for event TIMEOUT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_timeout](publish_timeout) module"]
pub type PUBLISH_TIMEOUT = crate::Reg<u32, _PUBLISH_TIMEOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_TIMEOUT;
#[doc = "`read()` method returns [publish_timeout::R](publish_timeout::R) reader structure"]
impl crate::Readable for PUBLISH_TIMEOUT {}
#[doc = "`write(|w| ..)` method takes [publish_timeout::W](publish_timeout::W) writer structure"]
impl crate::Writable for PUBLISH_TIMEOUT {}
#[doc = "Publish configuration for event TIMEOUT"]
pub mod publish_timeout;
#[doc = "Publish configuration for event STOPPED\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_stopped](publish_stopped) module"]
pub type PUBLISH_STOPPED = crate::Reg<u32, _PUBLISH_STOPPED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_STOPPED;
#[doc = "`read()` method returns [publish_stopped::R](publish_stopped::R) reader structure"]
impl crate::Readable for PUBLISH_STOPPED {}
#[doc = "`write(|w| ..)` method takes [publish_stopped::W](publish_stopped::W) writer structure"]
impl crate::Writable for PUBLISH_STOPPED {}
#[doc = "Publish configuration for event STOPPED"]
pub mod publish_stopped;
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
#[doc = "Enable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nmienset](nmienset) module"]
pub type NMIENSET = crate::Reg<u32, _NMIENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NMIENSET;
#[doc = "`read()` method returns [nmienset::R](nmienset::R) reader structure"]
impl crate::Readable for NMIENSET {}
#[doc = "`write(|w| ..)` method takes [nmienset::W](nmienset::W) writer structure"]
impl crate::Writable for NMIENSET {}
#[doc = "Enable interrupt"]
pub mod nmienset;
#[doc = "Disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nmienclr](nmienclr) module"]
pub type NMIENCLR = crate::Reg<u32, _NMIENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NMIENCLR;
#[doc = "`read()` method returns [nmienclr::R](nmienclr::R) reader structure"]
impl crate::Readable for NMIENCLR {}
#[doc = "`write(|w| ..)` method takes [nmienclr::W](nmienclr::W) writer structure"]
impl crate::Writable for NMIENCLR {}
#[doc = "Disable interrupt"]
pub mod nmienclr;
#[doc = "Run status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [runstatus](runstatus) module"]
pub type RUNSTATUS = crate::Reg<u32, _RUNSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RUNSTATUS;
#[doc = "`read()` method returns [runstatus::R](runstatus::R) reader structure"]
impl crate::Readable for RUNSTATUS {}
#[doc = "Run status"]
pub mod runstatus;
#[doc = "Request status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [reqstatus](reqstatus) module"]
pub type REQSTATUS = crate::Reg<u32, _REQSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REQSTATUS;
#[doc = "`read()` method returns [reqstatus::R](reqstatus::R) reader structure"]
impl crate::Readable for REQSTATUS {}
#[doc = "Request status"]
pub mod reqstatus;
#[doc = "Counter reload value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [crv](crv) module"]
pub type CRV = crate::Reg<u32, _CRV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRV;
#[doc = "`read()` method returns [crv::R](crv::R) reader structure"]
impl crate::Readable for CRV {}
#[doc = "`write(|w| ..)` method takes [crv::W](crv::W) writer structure"]
impl crate::Writable for CRV {}
#[doc = "Counter reload value"]
pub mod crv;
#[doc = "Enable register for reload request registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rren](rren) module"]
pub type RREN = crate::Reg<u32, _RREN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RREN;
#[doc = "`read()` method returns [rren::R](rren::R) reader structure"]
impl crate::Readable for RREN {}
#[doc = "`write(|w| ..)` method takes [rren::W](rren::W) writer structure"]
impl crate::Writable for RREN {}
#[doc = "Enable register for reload request registers"]
pub mod rren;
#[doc = "Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [config](config) module"]
pub type CONFIG = crate::Reg<u32, _CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG;
#[doc = "`read()` method returns [config::R](config::R) reader structure"]
impl crate::Readable for CONFIG {}
#[doc = "`write(|w| ..)` method takes [config::W](config::W) writer structure"]
impl crate::Writable for CONFIG {}
#[doc = "Configuration register"]
pub mod config;
#[doc = "Task Stop Enable\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tsen](tsen) module"]
pub type TSEN = crate::Reg<u32, _TSEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSEN;
#[doc = "`write(|w| ..)` method takes [tsen::W](tsen::W) writer structure"]
impl crate::Writable for TSEN {}
#[doc = "Task Stop Enable"]
pub mod tsen;
#[doc = "Description collection: Reload request n\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rr](rr) module"]
pub type RR = crate::Reg<u32, _RR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RR;
#[doc = "`write(|w| ..)` method takes [rr::W](rr::W) writer structure"]
impl crate::Writable for RR {}
#[doc = "Description collection: Reload request n"]
pub mod rr;
