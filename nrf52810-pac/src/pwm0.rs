#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock;
#[doc = r" Register block"]
#[repr(C)]
pub struct SEQ;
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod seq;
#[doc = r" Register block"]
#[repr(C)]
pub struct PSEL;
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "Stops PWM pulse generation on all channels at the end of current PWM period, and stops sequence playback"]
pub struct TASKS_STOP;
#[doc = "Stops PWM pulse generation on all channels at the end of current PWM period, and stops sequence playback"]
pub mod tasks_stop;
#[doc = "Description collection: Loads the first PWM value on all enabled channels from sequence n, and starts playing that sequence at the rate defined in SEQ\\[n\\]REFRESH and/or DECODER.MODE. Causes PWM generation to start if not running."]
pub struct TASKS_SEQSTART;
#[doc = "Description collection: Loads the first PWM value on all enabled channels from sequence n, and starts playing that sequence at the rate defined in SEQ\\[n\\]REFRESH and/or DECODER.MODE. Causes PWM generation to start if not running."]
pub mod tasks_seqstart;
#[doc = "Steps by one value in the current sequence on all enabled channels if DECODER.MODE=NextStep. Does not cause PWM generation to start if not running."]
pub struct TASKS_NEXTSTEP;
#[doc = "Steps by one value in the current sequence on all enabled channels if DECODER.MODE=NextStep. Does not cause PWM generation to start if not running."]
pub mod tasks_nextstep;
#[doc = "Response to STOP task, emitted when PWM pulses are no longer generated"]
pub struct EVENTS_STOPPED;
#[doc = "Response to STOP task, emitted when PWM pulses are no longer generated"]
pub mod events_stopped;
#[doc = "Description collection: First PWM period started on sequence n"]
pub struct EVENTS_SEQSTARTED;
#[doc = "Description collection: First PWM period started on sequence n"]
pub mod events_seqstarted;
#[doc = "Description collection: Emitted at end of every sequence n, when last value from RAM has been applied to wave counter"]
pub struct EVENTS_SEQEND;
#[doc = "Description collection: Emitted at end of every sequence n, when last value from RAM has been applied to wave counter"]
pub mod events_seqend;
#[doc = "Emitted at the end of each PWM period"]
pub struct EVENTS_PWMPERIODEND;
#[doc = "Emitted at the end of each PWM period"]
pub mod events_pwmperiodend;
#[doc = "Concatenated sequences have been played the amount of times defined in LOOP.CNT"]
pub struct EVENTS_LOOPSDONE;
#[doc = "Concatenated sequences have been played the amount of times defined in LOOP.CNT"]
pub mod events_loopsdone;
#[doc = "Shortcuts between local events and tasks"]
pub struct SHORTS;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "Enable or disable interrupt"]
pub struct INTEN;
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "Enable interrupt"]
pub struct INTENSET;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "Disable interrupt"]
pub struct INTENCLR;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "PWM module enable register"]
pub struct ENABLE;
#[doc = "PWM module enable register"]
pub mod enable;
#[doc = "Selects operating mode of the wave counter"]
pub struct MODE;
#[doc = "Selects operating mode of the wave counter"]
pub mod mode;
#[doc = "Value up to which the pulse generator counter counts"]
pub struct COUNTERTOP;
#[doc = "Value up to which the pulse generator counter counts"]
pub mod countertop;
#[doc = "Configuration for PWM_CLK"]
pub struct PRESCALER;
#[doc = "Configuration for PWM_CLK"]
pub mod prescaler;
#[doc = "Configuration of the decoder"]
pub struct DECODER;
#[doc = "Configuration of the decoder"]
pub mod decoder;
#[doc = "Number of playbacks of a loop"]
pub struct LOOP;
#[doc = "Number of playbacks of a loop"]
pub mod loop_;
