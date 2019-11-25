set -x


OUT_DIR=/tmp/issue59535/out
INCREMENTAL_DIR=/tmp/issue59535/incr

CORTEX_OUT_DIR=$OUT_DIR/cortex
NRFHAL_OUT_DIR=$OUT_DIR/nrfhal
RUBBLE1_OUT_DIR=$OUT_DIR/rubble1
RUBBLE2_OUT_DIR=$OUT_DIR/rubble2

RUBBLE_RUSTC_LOG=rustc_mir::monomorphize::partitioning,rustc_mir::monomorphize::collector,rustc::dep_graph::cgu_reuse_tracker,rustc_codegen_llvm::back::lto
# RUBBLE_ZFLAGS="-Z incremental-info -Z incremental-dont-internalize-symbols -Z query-dep-graph -Z no-parallel-llvm "
RUBBLE_ZFLAGS="-Z incremental-info -Z query-dep-graph -Z no-parallel-llvm  -Z print-mono-items=lazy -Z human-readable-cgu-names -Zsymbol-mangling-version=v0 "
RUSTC=/home/pnkfelix/Dev/Mozilla/rust.git/objdir-dbgopt/build/x86_64-unknown-linux-gnu/stage1/bin/rustc
# RUSTC="rustc +nightly"
# RUBBLE_ZFLAGS="-Z incremental-info -Z query-dep-graph"
# RUSTC="rustc +nightly-2019-01-01"


rm -r $OUT_DIR
rm -r $INCREMENTAL_DIR
rm -r $INCREMENTAL_DIR.v1
rm -r $INCREMENTAL_DIR.v2
mkdir -p $OUT_DIR
mkdir -p $INCREMENTAL_DIR
mkdir -p $CORTEX_OUT_DIR
mkdir -p $NRFHAL_OUT_DIR
mkdir -p $RUBBLE1_OUT_DIR
mkdir -p $RUBBLE2_OUT_DIR

LLVMDIS=/home/pnkfelix/Dev/Mozilla/rust.git/objdir-dbgopt/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-dis
# MEMORY_X_DIR=rubble/target/thumbv7em-none-eabi/debug/build/nrf52810-hal-e97e5f1701f7e4f4/out
MEMORY_X_DIR=nrf52810-hal
LINK_X_DIR=cortex-m-rt

$RUSTC --crate-name cortex_m_rt cortex-m-rt/src/lib.rs --crate-type lib --emit=mir,metadata,link -C save-temps -C opt-level=s --out-dir $CORTEX_OUT_DIR  --target thumbv7em-none-eabi -C link-arg=-Tlink.x.in -L $LINK_X_DIR

# $RUSTC --edition=2018 --crate-name nrf52810_hal nrf52810-hal/src/lib.rs --crate-type lib --emit=mir,metadata,link -C save-temps -C opt-level=s -C metadata=aa86958b67bf89f5 --out-dir $NRFHAL_OUT_DIR --target thumbv7em-none-eabi --extern cortex_m_rt=$CORTEX_OUT_DIR/libcortex_m_rt.rmeta -C link-arg=-Tlink.x.in -L $MEMORY_X_DIR -L $LINK_X_DIR -Z unpretty=expanded > /tmp/nrf52810_hal_expanded.rs

$RUSTC --edition=2018 --crate-name nrf52810_hal nrf52810-hal/src/lib.rs --crate-type lib --emit=mir,metadata,link -C save-temps -C opt-level=s -C metadata=aa86958b67bf89f5 --out-dir $NRFHAL_OUT_DIR --target thumbv7em-none-eabi --extern cortex_m_rt=$CORTEX_OUT_DIR/libcortex_m_rt.rmeta -C link-arg=-Tlink.x.in -L $MEMORY_X_DIR -L $LINK_X_DIR

# $RUSTC --edition=2018 --crate-name nrf52810_hal nrf52810_hal_expanded.rs --crate-type lib --emit=mir,metadata,link -C save-temps -C opt-level=s -C metadata=aa86958b67bf89f5 --out-dir $NRFHAL_OUT_DIR --target thumbv7em-none-eabi --extern cortex_m_rt=$CORTEX_OUT_DIR/libcortex_m_rt.rmeta -C link-arg=-Tlink.x.in -L $MEMORY_X_DIR -L $LINK_X_DIR


cp rubble/src/main.rs.v1 rubble/src/main.rs

RUSTC_LOG=$RUBBLE_RUSTC_LOG $RUSTC --crate-name rubble rubble/src/main.rs --crate-type bin --emit=mir,link -C save-temps -C opt-level=s --out-dir $RUBBLE1_OUT_DIR --target thumbv7em-none-eabi -C incremental=$INCREMENTAL_DIR -L dependency=$CORTEX_OUT_DIR --extern nrf52810_hal=$NRFHAL_OUT_DIR/libnrf52810_hal.rlib -C link-arg=-Tlink.x.in -L $MEMORY_X_DIR -L $LINK_X_DIR  -C linker-flavor=ld.lld -C codegen-units=2 $RUBBLE_ZFLAGS > /tmp/rubble1.log 2>&1

cp rubble/src/main.rs.v2 rubble/src/main.rs
cp -a $INCREMENTAL_DIR $INCREMENTAL_DIR.v1

# RUSTC_LOG=$RUBBLE_RUSTC_LOG $RUSTC --crate-name rubble rubble/src/main.rs --crate-type bin --emit=mir,link -C save-temps -C opt-level=s --out-dir $RUBBLE2_OUT_DIR --target thumbv7em-none-eabi -C incremental=$INCREMENTAL_DIR -L dependency=$CORTEX_OUT_DIR --extern nrf52810_hal=$NRFHAL_OUT_DIR/libnrf52810_hal.rlib -C link-arg=-Tlink.x.in -L $MEMORY_X_DIR -L $LINK_X_DIR  -C linker-flavor=ld.lld -C codegen-units=2 $RUBBLE_ZFLAGS -C llvm-args=-debug  > /tmp/rubble2.log 2>&1
RUSTC_LOG=$RUBBLE_RUSTC_LOG $RUSTC --crate-name rubble rubble/src/main.rs --crate-type bin --emit=mir,link -C save-temps -C opt-level=s --out-dir $RUBBLE2_OUT_DIR --target thumbv7em-none-eabi -C incremental=$INCREMENTAL_DIR -L dependency=$CORTEX_OUT_DIR --extern nrf52810_hal=$NRFHAL_OUT_DIR/libnrf52810_hal.rlib -C link-arg=-Tlink.x.in -L $MEMORY_X_DIR -L $LINK_X_DIR  -C linker-flavor=ld.lld -C codegen-units=2 $RUBBLE_ZFLAGS  > /tmp/rubble2.log 2>&1

tail /tmp/rubble2.log

cp -a $INCREMENTAL_DIR $INCREMENTAL_DIR.v2

# set +x
# for f in /tmp/issue59535/out/rubble*/*.bc; do $LLVMDIS $f; done
find /tmp/issue59535/incr.v1/ -name '*.bc' -exec $LLVMDIS {} \;
find /tmp/issue59535/incr.v2/ -name '*.bc' -exec $LLVMDIS {} \;
find /tmp/issue59535/out/rubble1 -name '*.bc' -exec $LLVMDIS {} \;
find /tmp/issue59535/out/rubble2 -name '*.bc' -exec $LLVMDIS {} \;
