#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock;
#[doc = "Start temperature measurement"]
pub struct TASKS_START;
#[doc = "Start temperature measurement"]
pub mod tasks_start;
#[doc = "Stop temperature measurement"]
pub struct TASKS_STOP;
#[doc = "Stop temperature measurement"]
pub mod tasks_stop;
#[doc = "Temperature measurement complete, data ready"]
pub struct EVENTS_DATARDY;
#[doc = "Temperature measurement complete, data ready"]
pub mod events_datardy;
#[doc = "Enable interrupt"]
pub struct INTENSET;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "Disable interrupt"]
pub struct INTENCLR;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "Temperature in degC (0.25deg steps)"]
pub struct TEMP;
#[doc = "Temperature in degC (0.25deg steps)"]
pub mod temp;
#[doc = "Slope of 1st piece wise linear function"]
pub struct A0;
#[doc = "Slope of 1st piece wise linear function"]
pub mod a0;
#[doc = "Slope of 2nd piece wise linear function"]
pub struct A1;
#[doc = "Slope of 2nd piece wise linear function"]
pub mod a1;
#[doc = "Slope of 3rd piece wise linear function"]
pub struct A2;
#[doc = "Slope of 3rd piece wise linear function"]
pub mod a2;
#[doc = "Slope of 4th piece wise linear function"]
pub struct A3;
#[doc = "Slope of 4th piece wise linear function"]
pub mod a3;
#[doc = "Slope of 5th piece wise linear function"]
pub struct A4;
#[doc = "Slope of 5th piece wise linear function"]
pub mod a4;
#[doc = "Slope of 6th piece wise linear function"]
pub struct A5;
#[doc = "Slope of 6th piece wise linear function"]
pub mod a5;
#[doc = "y-intercept of 1st piece wise linear function"]
pub struct B0;
#[doc = "y-intercept of 1st piece wise linear function"]
pub mod b0;
#[doc = "y-intercept of 2nd piece wise linear function"]
pub struct B1;
#[doc = "y-intercept of 2nd piece wise linear function"]
pub mod b1;
#[doc = "y-intercept of 3rd piece wise linear function"]
pub struct B2;
#[doc = "y-intercept of 3rd piece wise linear function"]
pub mod b2;
#[doc = "y-intercept of 4th piece wise linear function"]
pub struct B3;
#[doc = "y-intercept of 4th piece wise linear function"]
pub mod b3;
#[doc = "y-intercept of 5th piece wise linear function"]
pub struct B4;
#[doc = "y-intercept of 5th piece wise linear function"]
pub mod b4;
#[doc = "y-intercept of 6th piece wise linear function"]
pub struct B5;
#[doc = "y-intercept of 6th piece wise linear function"]
pub mod b5;
#[doc = "End point of 1st piece wise linear function"]
pub struct T0;
#[doc = "End point of 1st piece wise linear function"]
pub mod t0;
#[doc = "End point of 2nd piece wise linear function"]
pub struct T1;
#[doc = "End point of 2nd piece wise linear function"]
pub mod t1;
#[doc = "End point of 3rd piece wise linear function"]
pub struct T2;
#[doc = "End point of 3rd piece wise linear function"]
pub mod t2;
#[doc = "End point of 4th piece wise linear function"]
pub struct T3;
#[doc = "End point of 4th piece wise linear function"]
pub mod t3;
#[doc = "End point of 5th piece wise linear function"]
pub struct T4;
#[doc = "End point of 5th piece wise linear function"]
pub mod t4;
