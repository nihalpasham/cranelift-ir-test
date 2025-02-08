
# A High-Level Overview of Cranelift’s Pipeline:

```md
CLIF -> legalization -> mid-end egraph rewrites if enabled (rules in ISLE) -> lowering to backend-specific VCode (rules in ISLE) -> regalloc -> binary emission
```
## Use rustc’s `cg_clif` to produce CLIF IR from rust as follows:

```sh
../rustc_codegen_cranelift/dist/bin/rustc-clif --emit=llvm-ir src/main.rs
```

```rust
#[no_mangle]
fn test(a: u8, b: u8) -> u8 {
    if a > b {
        let c = a - b;
        c
    } else {
        let c = a + b;
        c
    }
}
```

to this
```llvm
set opt_level=none
set tls_model=macho
set libcall_call_conv=isa_default
set probestack_size_log2=12
set probestack_strategy=inline
set bb_padding_log2_minus_one=0
set regalloc_checker=0
set regalloc_verbose_logs=0
set enable_alias_analysis=1
set enable_verifier=0
set enable_pcc=0
set is_pic=1
set use_colocated_libcalls=0
set enable_float=1
set enable_nan_canonicalization=0
set enable_pinned_reg=0
set enable_atomics=1
set enable_safepoints=0
set enable_llvm_abi_extensions=1
set unwind_info=1
set preserve_frame_pointers=1
set machine_code_cfg_info=0
set enable_probestack=1
set enable_jump_tables=1
set enable_heap_access_spectre_mitigation=1
set enable_table_access_spectre_mitigation=1
set enable_incremental_compilation_cache_checks=0
target aarch64 has_lse=0 has_pauth=0 sign_return_address_all=0 sign_return_address=0 sign_return_address_with_bkey=0 use_bti=0


function u0:9(i8, i8) -> i8 apple_aarch64 {
; symbol test
; instance Instance { def: Item(DefId(0:4 ~ main[1152]::test)), args: [] }
; abi FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: AbiAndPrefAlign { abi: Align(1 bytes), pref: Align(1 bytes) }, abi: Scalar(Initialized { value: Int(I8, false), valid_range: 0..=255 }), fields: Primitive, largest_niche: None, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes) } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: AbiAndPrefAlign { abi: Align(1 bytes), pref: Align(1 bytes) }, abi: Scalar(Initialized { value: Int(I8, false), valid_range: 0..=255 }), fields: Primitive, largest_niche: None, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes) } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: AbiAndPrefAlign { abi: Align(1 bytes), pref: Align(1 bytes) }, abi: Scalar(Initialized { value: Int(I8, false), valid_range: 0..=255 }), fields: Primitive, largest_niche: None, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes) } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }

; kind  loc.idx   param    pass mode                            ty
; ssa   _0    u8                                1b 1, 1              var=0
; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8

; kind  local ty                              size align (abi,pref)
; ssa   _1    u8                                1b 1, 1              var=1
; ssa   _2    u8                                1b 1, 1              var=2
; ssa   _3    bool                              1b 1, 1              var=3
; ssa   _4    u8                                1b 1, 1              var=4
; ssa   _5    (u8, bool)                        2b 1, 8              var=(5, 6)
; ssa   _6    u8                                1b 1, 1              var=7
; ssa   _7    (u8, bool)                        2b 1, 8              var=(8, 9)

    gv0 = symbol colocated userextname0 ; alloc10
    gv1 = symbol colocated userextname2 ; alloc11
    sig0 = (i64) apple_aarch64
    sig1 = (i64) apple_aarch64
    fn0 = u0:11 sig0 ; "_ZN4core9panicking11panic_const24panic_const_sub_overflow17h69a7109fa301a030E"
    fn1 = u0:12 sig1 ; "_ZN4core9panicking11panic_const24panic_const_add_overflow17h1c3192e1cfd46512E"

block0(v0: i8, v1: i8):
    v2 -> v0
    v5 -> v0
    v11 -> v0
    v3 -> v1
    v6 -> v1
    v12 -> v1
    nop 
; write_cvalue: Var(_1, var1): u8 <- ByVal(v0): u8
; write_cvalue: Var(_2, var2): u8 <- ByVal(v1): u8
    jump block1

block1:
    nop 
; _3 = Gt(copy _1, copy _2)
    v4 = icmp.i8 ugt v2, v3
; write_cvalue: Var(_3, var3): bool <- ByVal(v4): bool
; 
; switchInt(move _3)
    brif v4, block2, block4

block2:
    nop 
; _5 = SubWithOverflow(copy _1, copy _2)
    v7 = isub.i8 v5, v6
    v10 -> v7
    v8 = icmp ugt v7, v5
; write_cvalue: VarPair(_5, var5, var6): (u8, bool) <- ByValPair(v7, v8): (u8, bool)
; 
; assert(!move (_5.1: bool), "attempt to compute `{} - {}`, which would overflow", copy _1, copy _2)
    brif v8, block7, block3

block7 cold:
    nop 
    v9 = global_value.i64 gv0
    call fn0(v9)
; lib_call _ZN4core9panicking11panic_const24panic_const_sub_overflow17h69a7109fa301a030E
    trap unreachable

block3:
    nop 
; _4 = move (_5.0: u8)
; write_cvalue: Var(_4, var4): u8 <- ByVal(v10): u8
; _0 = copy _4
; write_cvalue: Var(_0, var0): u8 <- ByVal(v10): u8
; 
; goto
    return v10

block4:
    nop 
; _7 = AddWithOverflow(copy _1, copy _2)
    v13 = iadd.i8 v11, v12
    v16 -> v13
    v14 = icmp ult v13, v11
; write_cvalue: VarPair(_7, var8, var9): (u8, bool) <- ByValPair(v13, v14): (u8, bool)
; 
; assert(!move (_7.1: bool), "attempt to compute `{} + {}`, which would overflow", copy _1, copy _2)
    brif v14, block8, block5

block8 cold:
    nop 
    v15 = global_value.i64 gv1
    call fn1(v15)
; lib_call _ZN4core9panicking11panic_const24panic_const_add_overflow17h1c3192e1cfd46512E
    trap unreachable

block5:
    nop 
; _6 = move (_7.0: u8)
; write_cvalue: Var(_6, var7): u8 <- ByVal(v16): u8
; _0 = copy _6
; write_cvalue: Var(_0, var0): u8 <- ByVal(v16): u8
; 
; goto
    return v16

block6:
    v18 = iconst.i8 0
    v17 -> v18
    nop 
; 
; return
    return v17  ; v17 = 0
}
```
**Notes:** 
- cranelift-module uses `u0:X` to refer to function `X` within the current module (can be an import) and `u1:Y` to refer to data object `Y`.
- The `v2 -> v0` are aliases. So in this case all references to v2 are implicitly replaced with v0 during compilation. They mostly exist to make the SSA lowering in cranelift-frontend easier.