export C_INCLUDE_PATH=$JULIA_DIR/include/julia
echo $C_INCLUDE_PATH
rm src/etc/bindings.rs
bindgen $C_INCLUDE_PATH/julia.h -o src/etc/bindings.rs \
--blacklist-item FP_NAN \
--blacklist-item FP_INFINITE \
--blacklist-item FP_ZERO \
--blacklist-item FP_SUBNORMAL \
--blacklist-item FP_NORMAL \
--blacklist-item IPPORT_RESERVED
