set -x

OUT_DIR=/tmp/issue59535/out
INCREMENTAL_DIR=/tmp/issue59535/incr

mkdir -p $OUT_DIR
mkdir -p $INCREMENTAL_DIR

# MEMORY_X_DIR=/home/pnkfelix/Dev/Mozilla/issue59535/rubble/target/thumbv7em-none-eabi/debug/build/nrf52810-hal-e97e5f1701f7e4f4/out
MEMORY_X_DIR=/home/pnkfelix/Dev/Mozilla/issue59535/nrf52810-hal
LINK_X_DIR=/home/pnkfelix/Dev/Mozilla/issue59535/cortex-m-rt/


rustc --crate-name cortex_m_rt /home/pnkfelix/Dev/Mozilla/issue59535/cortex-m-rt/src/lib.rs --crate-type lib --emit=metadata,link -C opt-level=s -C metadata=b3289259f012c58d -C extra-filename=-b3289259f012c58d --out-dir $OUT_DIR  --target thumbv7em-none-eabi -C link-arg=-Tlink.x.in -L $LINK_X_DIR

rustc --edition=2018 --crate-name nrf52810_hal /home/pnkfelix/Dev/Mozilla/issue59535/nrf52810-hal/src/lib.rs --crate-type lib --emit=metadata,link -C opt-level=s -C metadata=aa86958b67bf89f5 --out-dir $OUT_DIR --target thumbv7em-none-eabi --extern cortex_m_rt=$OUT_DIR/libcortex_m_rt-b3289259f012c58d.rmeta -C link-arg=-Tlink.x.in -L $MEMORY_X_DIR -L $LINK_X_DIR

cp rubble/src/main.rs.v1 rubble/src/main.rs

rustc --crate-name rubble rubble/src/main.rs --crate-type bin --emit=link -C opt-level=s --out-dir $OUT_DIR --target thumbv7em-none-eabi -C incremental=$INCREMENTAL_DIR -L dependency=$OUT_DIR --extern nrf52810_hal=$OUT_DIR/libnrf52810_hal.rlib -C link-arg=-Tlink.x.in -L $MEMORY_X_DIR -L $LINK_X_DIR

cp rubble/src/main.rs.v2 rubble/src/main.rs

rustc --crate-name rubble rubble/src/main.rs --crate-type bin --emit=link -C opt-level=s --out-dir $OUT_DIR --target thumbv7em-none-eabi -C incremental=$INCREMENTAL_DIR -L dependency=$OUT_DIR --extern nrf52810_hal=$OUT_DIR/libnrf52810_hal.rlib -C link-arg=-Tlink.x.in -L $MEMORY_X_DIR -L $LINK_X_DIR
