// Rust test file autogenerated with cargo build (build/spectests.rs).
// Please do NOT modify it by hand, as it will be reset on next build.
// Test based on spectests/br.wast
#![allow(
    warnings,
    dead_code
)]
use wabt::wat2wasm;
use std::{f32, f64};

use wasmer_runtime::types::Value;
use wasmer_runtime::{Instance, module::Module};
use wasmer_clif_backend::CraneliftCompiler;

use crate::spectests::_common::{
    spectest_importobject,
    NaNCheck,
};


// Line 3
fn create_module_1() -> Box<Instance> {
    let module_str = "(module
      (type (;0;) (func (param i32 i32 i32) (result i32)))
      (type (;1;) (func))
      (type (;2;) (func (result i32)))
      (type (;3;) (func (result i64)))
      (type (;4;) (func (result f32)))
      (type (;5;) (func (result f64)))
      (type (;6;) (func (param i32 i32) (result i32)))
      (func (;0;) (type 1))
      (func (;1;) (type 1)
        block  ;; label = @1
          br 0 (;@1;)
          i32.ctz
          drop
        end)
      (func (;2;) (type 1)
        block  ;; label = @1
          br 0 (;@1;)
          i64.ctz
          drop
        end)
      (func (;3;) (type 1)
        block  ;; label = @1
          br 0 (;@1;)
          f32.neg
          drop
        end)
      (func (;4;) (type 1)
        block  ;; label = @1
          br 0 (;@1;)
          f64.neg
          drop
        end)
      (func (;5;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          i32.const 1
          br 0 (;@1;)
          i32.ctz
        end)
      (func (;6;) (type 3) (result i64)
        block (result i64)  ;; label = @1
          i64.const 2
          br 0 (;@1;)
          i64.ctz
        end)
      (func (;7;) (type 4) (result f32)
        block (result f32)  ;; label = @1
          f32.const 0x1.8p+1 (;=3;)
          br 0 (;@1;)
          f32.neg
        end)
      (func (;8;) (type 5) (result f64)
        block (result f64)  ;; label = @1
          f64.const 0x1p+2 (;=4;)
          br 0 (;@1;)
          f64.neg
        end)
      (func (;9;) (type 1)
        block  ;; label = @1
          br 0 (;@1;)
          call 0
        end)
      (func (;10;) (type 1)
        block  ;; label = @1
          call 0
          br 0 (;@1;)
          call 0
        end)
      (func (;11;) (type 1)
        block  ;; label = @1
          nop
          call 0
          br 0 (;@1;)
        end)
      (func (;12;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          nop
          call 0
          i32.const 2
          br 0 (;@1;)
        end)
      (func (;13;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          loop (result i32)  ;; label = @2
            i32.const 3
            br 1 (;@1;)
            i32.const 2
          end
        end)
      (func (;14;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          loop (result i32)  ;; label = @2
            call 0
            i32.const 4
            br 1 (;@1;)
            i32.const 2
          end
        end)
      (func (;15;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          loop (result i32)  ;; label = @2
            nop
            call 0
            i32.const 5
            br 1 (;@1;)
          end
        end)
      (func (;16;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          i32.const 9
          br 0 (;@1;)
          br 0 (;@1;)
        end)
      (func (;17;) (type 1)
        block  ;; label = @1
          br 0 (;@1;)
          br_if 0 (;@1;)
        end)
      (func (;18;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          i32.const 8
          br 0 (;@1;)
          i32.const 1
          br_if 0 (;@1;)
          drop
          i32.const 7
        end)
      (func (;19;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          i32.const 6
          i32.const 9
          br 0 (;@1;)
          br_if 0 (;@1;)
          drop
          i32.const 7
        end)
      (func (;20;) (type 1)
        block  ;; label = @1
          br 0 (;@1;)
          br_table 0 (;@1;) 0 (;@1;) 0 (;@1;)
        end)
      (func (;21;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          i32.const 10
          br 0 (;@1;)
          i32.const 1
          br_table 0 (;@1;) 0 (;@1;) 0 (;@1;)
          i32.const 7
        end)
      (func (;22;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          i32.const 6
          i32.const 11
          br 0 (;@1;)
          br_table 0 (;@1;) 0 (;@1;)
          i32.const 7
        end)
      (func (;23;) (type 3) (result i64)
        block (result i64)  ;; label = @1
          i64.const 7
          br 0 (;@1;)
          return
        end)
      (func (;24;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          i32.const 2
          br 0 (;@1;)
          if (result i32)  ;; label = @2
            i32.const 0
          else
            i32.const 1
          end
        end)
      (func (;25;) (type 6) (param i32 i32) (result i32)
        block (result i32)  ;; label = @1
          get_local 0
          if (result i32)  ;; label = @2
            i32.const 3
            br 1 (;@1;)
          else
            get_local 1
          end
        end)
      (func (;26;) (type 6) (param i32 i32) (result i32)
        block (result i32)  ;; label = @1
          get_local 0
          if (result i32)  ;; label = @2
            get_local 1
          else
            i32.const 4
            br 1 (;@1;)
          end
        end)
      (func (;27;) (type 6) (param i32 i32) (result i32)
        block (result i32)  ;; label = @1
          i32.const 5
          br 0 (;@1;)
          get_local 0
          get_local 1
          select
        end)
      (func (;28;) (type 6) (param i32 i32) (result i32)
        block (result i32)  ;; label = @1
          get_local 0
          i32.const 6
          br 0 (;@1;)
          get_local 1
          select
        end)
      (func (;29;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          i32.const 0
          i32.const 1
          i32.const 7
          br 0 (;@1;)
          select
        end)
      (func (;30;) (type 0) (param i32 i32 i32) (result i32)
        i32.const -1)
      (func (;31;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          i32.const 12
          br 0 (;@1;)
          i32.const 2
          i32.const 3
          call 30
        end)
      (func (;32;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          i32.const 1
          i32.const 13
          br 0 (;@1;)
          i32.const 3
          call 30
        end)
      (func (;33;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          i32.const 1
          i32.const 2
          i32.const 14
          br 0 (;@1;)
          call 30
        end)
      (func (;34;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          i32.const 20
          br 0 (;@1;)
          i32.const 1
          i32.const 2
          i32.const 3
          call_indirect (type 0)
        end)
      (func (;35;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          i32.const 0
          i32.const 21
          br 0 (;@1;)
          i32.const 2
          i32.const 3
          call_indirect (type 0)
        end)
      (func (;36;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          i32.const 0
          i32.const 1
          i32.const 22
          br 0 (;@1;)
          i32.const 3
          call_indirect (type 0)
        end)
      (func (;37;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          i32.const 0
          i32.const 1
          i32.const 2
          i32.const 23
          br 0 (;@1;)
          call_indirect (type 0)
        end)
      (func (;38;) (type 2) (result i32)
        (local f32)
        block (result i32)  ;; label = @1
          i32.const 17
          br 0 (;@1;)
          set_local 0
          i32.const -1
        end)
      (func (;39;) (type 2) (result i32)
        (local i32)
        block (result i32)  ;; label = @1
          i32.const 1
          br 0 (;@1;)
          tee_local 0
        end)
      (func (;40;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          i32.const 1
          br 0 (;@1;)
          set_global 0
        end)
      (func (;41;) (type 4) (result f32)
        block (result f32)  ;; label = @1
          f32.const 0x1.b33334p+0 (;=1.7;)
          br 0 (;@1;)
          f32.load
        end)
      (func (;42;) (type 3) (result i64)
        block (result i64)  ;; label = @1
          i64.const 30
          br 0 (;@1;)
          i64.load8_s
        end)
      (func (;43;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          i32.const 30
          br 0 (;@1;)
          f64.const 0x1.cp+2 (;=7;)
          f64.store
          i32.const -1
        end)
      (func (;44;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          i32.const 2
          i32.const 31
          br 0 (;@1;)
          i64.store
          i32.const -1
        end)
      (func (;45;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          i32.const 32
          br 0 (;@1;)
          i32.const 7
          i32.store8
          i32.const -1
        end)
      (func (;46;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          i32.const 2
          i32.const 33
          br 0 (;@1;)
          i64.store16
          i32.const -1
        end)
      (func (;47;) (type 4) (result f32)
        block (result f32)  ;; label = @1
          f32.const 0x1.b33334p+1 (;=3.4;)
          br 0 (;@1;)
          f32.neg
        end)
      (func (;48;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          i32.const 3
          br 0 (;@1;)
          i32.const 10
          i32.add
        end)
      (func (;49;) (type 3) (result i64)
        block (result i64)  ;; label = @1
          i64.const 10
          i64.const 45
          br 0 (;@1;)
          i64.sub
        end)
      (func (;50;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          i32.const 44
          br 0 (;@1;)
          i32.eqz
        end)
      (func (;51;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          i32.const 43
          br 0 (;@1;)
          f64.const 0x1.4p+3 (;=10;)
          f64.le
        end)
      (func (;52;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          f32.const 0x1.4p+3 (;=10;)
          i32.const 42
          br 0 (;@1;)
          f32.ne
        end)
      (func (;53;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          i32.const 41
          br 0 (;@1;)
          i32.wrap/i64
        end)
      (func (;54;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          i32.const 40
          br 0 (;@1;)
          memory.grow
        end)
      (func (;55;) (type 2) (result i32)
        i32.const 1
        block (result i32)  ;; label = @1
          call 0
          i32.const 4
          i32.const 8
          br 0 (;@1;)
          i32.add
        end
        i32.add)
      (func (;56;) (type 2) (result i32)
        i32.const 1
        block (result i32)  ;; label = @1
          i32.const 2
          drop
          block (result i32)  ;; label = @2
            i32.const 4
            drop
            i32.const 8
            br 1 (;@1;)
            br 0 (;@2;)
          end
          drop
          i32.const 16
        end
        i32.add)
      (func (;57;) (type 2) (result i32)
        i32.const 1
        block (result i32)  ;; label = @1
          i32.const 2
          drop
          block (result i32)  ;; label = @2
            i32.const 4
            drop
            i32.const 8
            br 1 (;@1;)
            i32.const 1
            br_if 0 (;@2;)
            drop
            i32.const 32
          end
          drop
          i32.const 16
        end
        i32.add)
      (func (;58;) (type 2) (result i32)
        i32.const 1
        block (result i32)  ;; label = @1
          i32.const 2
          drop
          i32.const 4
          i32.const 8
          br 0 (;@1;)
          br_if 0 (;@1;)
          drop
          i32.const 16
        end
        i32.add)
      (func (;59;) (type 2) (result i32)
        i32.const 1
        block (result i32)  ;; label = @1
          i32.const 2
          drop
          block (result i32)  ;; label = @2
            i32.const 4
            drop
            i32.const 8
            br 1 (;@1;)
            i32.const 1
            br_table 0 (;@2;)
          end
          drop
          i32.const 16
        end
        i32.add)
      (func (;60;) (type 2) (result i32)
        i32.const 1
        block (result i32)  ;; label = @1
          i32.const 2
          drop
          i32.const 4
          i32.const 8
          br 0 (;@1;)
          br_table 0 (;@1;)
          i32.const 16
        end
        i32.add)
      (table (;0;) 1 1 anyfunc)
      (memory (;0;) 1)
      (global (;0;) (mut i32) (i32.const 10))
      (export \"type-i32\" (func 1))
      (export \"type-i64\" (func 2))
      (export \"type-f32\" (func 3))
      (export \"type-f64\" (func 4))
      (export \"type-i32-value\" (func 5))
      (export \"type-i64-value\" (func 6))
      (export \"type-f32-value\" (func 7))
      (export \"type-f64-value\" (func 8))
      (export \"as-block-first\" (func 9))
      (export \"as-block-mid\" (func 10))
      (export \"as-block-last\" (func 11))
      (export \"as-block-value\" (func 12))
      (export \"as-loop-first\" (func 13))
      (export \"as-loop-mid\" (func 14))
      (export \"as-loop-last\" (func 15))
      (export \"as-br-value\" (func 16))
      (export \"as-br_if-cond\" (func 17))
      (export \"as-br_if-value\" (func 18))
      (export \"as-br_if-value-cond\" (func 19))
      (export \"as-br_table-index\" (func 20))
      (export \"as-br_table-value\" (func 21))
      (export \"as-br_table-value-index\" (func 22))
      (export \"as-return-value\" (func 23))
      (export \"as-if-cond\" (func 24))
      (export \"as-if-then\" (func 25))
      (export \"as-if-else\" (func 26))
      (export \"as-select-first\" (func 27))
      (export \"as-select-second\" (func 28))
      (export \"as-select-cond\" (func 29))
      (export \"as-call-first\" (func 31))
      (export \"as-call-mid\" (func 32))
      (export \"as-call-last\" (func 33))
      (export \"as-call_indirect-func\" (func 34))
      (export \"as-call_indirect-first\" (func 35))
      (export \"as-call_indirect-mid\" (func 36))
      (export \"as-call_indirect-last\" (func 37))
      (export \"as-set_local-value\" (func 38))
      (export \"as-tee_local-value\" (func 39))
      (export \"as-set_global-value\" (func 40))
      (export \"as-load-address\" (func 41))
      (export \"as-loadN-address\" (func 42))
      (export \"as-store-address\" (func 43))
      (export \"as-store-value\" (func 44))
      (export \"as-storeN-address\" (func 45))
      (export \"as-storeN-value\" (func 46))
      (export \"as-unary-operand\" (func 47))
      (export \"as-binary-left\" (func 48))
      (export \"as-binary-right\" (func 49))
      (export \"as-test-operand\" (func 50))
      (export \"as-compare-left\" (func 51))
      (export \"as-compare-right\" (func 52))
      (export \"as-convert-operand\" (func 53))
      (export \"as-memory.grow-size\" (func 54))
      (export \"nested-block-value\" (func 55))
      (export \"nested-br-value\" (func 56))
      (export \"nested-br_if-value\" (func 57))
      (export \"nested-br_if-value-cond\" (func 58))
      (export \"nested-br_table-value\" (func 59))
      (export \"nested-br_table-value-index\" (func 60))
      (elem (;0;) (i32.const 0) 30))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new()).expect("WASM can't be compiled");
    module.instantiate(&spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_1(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 334
fn c1_l334_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c1_l334_action_invoke");
    let result = instance.call("type-i32", &[]);
    assert_eq!(result, Ok(None));
    result.map(|_| ())
}

// Line 335
fn c2_l335_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c2_l335_action_invoke");
    let result = instance.call("type-i64", &[]);
    assert_eq!(result, Ok(None));
    result.map(|_| ())
}

// Line 336
fn c3_l336_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c3_l336_action_invoke");
    let result = instance.call("type-f32", &[]);
    assert_eq!(result, Ok(None));
    result.map(|_| ())
}

// Line 337
fn c4_l337_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c4_l337_action_invoke");
    let result = instance.call("type-f64", &[]);
    assert_eq!(result, Ok(None));
    result.map(|_| ())
}

// Line 339
fn c5_l339_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c5_l339_action_invoke");
    let result = instance.call("type-i32-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(1 as i32))));
    result.map(|_| ())
}

// Line 340
fn c6_l340_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c6_l340_action_invoke");
    let result = instance.call("type-i64-value", &[]);
    assert_eq!(result, Ok(Some(Value::I64(2 as i64))));
    result.map(|_| ())
}

// Line 341
fn c7_l341_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c7_l341_action_invoke");
    let result = instance.call("type-f32-value", &[]);
    assert_eq!(result, Ok(Some(Value::F32((3.0f32).to_bits()))));
    result.map(|_| ())
}

// Line 342
fn c8_l342_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c8_l342_action_invoke");
    let result = instance.call("type-f64-value", &[]);
    assert_eq!(result, Ok(Some(Value::F64((4.0f64).to_bits()))));
    result.map(|_| ())
}

// Line 344
fn c9_l344_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c9_l344_action_invoke");
    let result = instance.call("as-block-first", &[]);
    assert_eq!(result, Ok(None));
    result.map(|_| ())
}

// Line 345
fn c10_l345_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c10_l345_action_invoke");
    let result = instance.call("as-block-mid", &[]);
    assert_eq!(result, Ok(None));
    result.map(|_| ())
}

// Line 346
fn c11_l346_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c11_l346_action_invoke");
    let result = instance.call("as-block-last", &[]);
    assert_eq!(result, Ok(None));
    result.map(|_| ())
}

// Line 347
fn c12_l347_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c12_l347_action_invoke");
    let result = instance.call("as-block-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(2 as i32))));
    result.map(|_| ())
}

// Line 349
fn c13_l349_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c13_l349_action_invoke");
    let result = instance.call("as-loop-first", &[]);
    assert_eq!(result, Ok(Some(Value::I32(3 as i32))));
    result.map(|_| ())
}

// Line 350
fn c14_l350_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c14_l350_action_invoke");
    let result = instance.call("as-loop-mid", &[]);
    assert_eq!(result, Ok(Some(Value::I32(4 as i32))));
    result.map(|_| ())
}

// Line 351
fn c15_l351_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c15_l351_action_invoke");
    let result = instance.call("as-loop-last", &[]);
    assert_eq!(result, Ok(Some(Value::I32(5 as i32))));
    result.map(|_| ())
}

// Line 353
fn c16_l353_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c16_l353_action_invoke");
    let result = instance.call("as-br-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 355
fn c17_l355_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c17_l355_action_invoke");
    let result = instance.call("as-br_if-cond", &[]);
    assert_eq!(result, Ok(None));
    result.map(|_| ())
}

// Line 356
fn c18_l356_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c18_l356_action_invoke");
    let result = instance.call("as-br_if-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(8 as i32))));
    result.map(|_| ())
}

// Line 357
fn c19_l357_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c19_l357_action_invoke");
    let result = instance.call("as-br_if-value-cond", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 359
fn c20_l359_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c20_l359_action_invoke");
    let result = instance.call("as-br_table-index", &[]);
    assert_eq!(result, Ok(None));
    result.map(|_| ())
}

// Line 360
fn c21_l360_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c21_l360_action_invoke");
    let result = instance.call("as-br_table-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(10 as i32))));
    result.map(|_| ())
}

// Line 361
fn c22_l361_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c22_l361_action_invoke");
    let result = instance.call("as-br_table-value-index", &[]);
    assert_eq!(result, Ok(Some(Value::I32(11 as i32))));
    result.map(|_| ())
}

// Line 363
fn c23_l363_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c23_l363_action_invoke");
    let result = instance.call("as-return-value", &[]);
    assert_eq!(result, Ok(Some(Value::I64(7 as i64))));
    result.map(|_| ())
}

// Line 365
fn c24_l365_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c24_l365_action_invoke");
    let result = instance.call("as-if-cond", &[]);
    assert_eq!(result, Ok(Some(Value::I32(2 as i32))));
    result.map(|_| ())
}

// Line 366
fn c25_l366_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c25_l366_action_invoke");
    let result = instance.call("as-if-then", &[Value::I32(1 as i32), Value::I32(6 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(3 as i32))));
    result.map(|_| ())
}

// Line 367
fn c26_l367_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c26_l367_action_invoke");
    let result = instance.call("as-if-then", &[Value::I32(0 as i32), Value::I32(6 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(6 as i32))));
    result.map(|_| ())
}

// Line 368
fn c27_l368_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c27_l368_action_invoke");
    let result = instance.call("as-if-else", &[Value::I32(0 as i32), Value::I32(6 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(4 as i32))));
    result.map(|_| ())
}

// Line 369
fn c28_l369_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c28_l369_action_invoke");
    let result = instance.call("as-if-else", &[Value::I32(1 as i32), Value::I32(6 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(6 as i32))));
    result.map(|_| ())
}

// Line 371
fn c29_l371_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c29_l371_action_invoke");
    let result = instance.call("as-select-first", &[Value::I32(0 as i32), Value::I32(6 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(5 as i32))));
    result.map(|_| ())
}

// Line 372
fn c30_l372_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c30_l372_action_invoke");
    let result = instance.call("as-select-first", &[Value::I32(1 as i32), Value::I32(6 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(5 as i32))));
    result.map(|_| ())
}

// Line 373
fn c31_l373_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c31_l373_action_invoke");
    let result = instance.call("as-select-second", &[Value::I32(0 as i32), Value::I32(6 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(6 as i32))));
    result.map(|_| ())
}

// Line 374
fn c32_l374_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c32_l374_action_invoke");
    let result = instance.call("as-select-second", &[Value::I32(1 as i32), Value::I32(6 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(6 as i32))));
    result.map(|_| ())
}

// Line 375
fn c33_l375_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c33_l375_action_invoke");
    let result = instance.call("as-select-cond", &[]);
    assert_eq!(result, Ok(Some(Value::I32(7 as i32))));
    result.map(|_| ())
}

// Line 377
fn c34_l377_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c34_l377_action_invoke");
    let result = instance.call("as-call-first", &[]);
    assert_eq!(result, Ok(Some(Value::I32(12 as i32))));
    result.map(|_| ())
}

// Line 378
fn c35_l378_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c35_l378_action_invoke");
    let result = instance.call("as-call-mid", &[]);
    assert_eq!(result, Ok(Some(Value::I32(13 as i32))));
    result.map(|_| ())
}

// Line 379
fn c36_l379_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c36_l379_action_invoke");
    let result = instance.call("as-call-last", &[]);
    assert_eq!(result, Ok(Some(Value::I32(14 as i32))));
    result.map(|_| ())
}

// Line 381
fn c37_l381_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c37_l381_action_invoke");
    let result = instance.call("as-call_indirect-func", &[]);
    assert_eq!(result, Ok(Some(Value::I32(20 as i32))));
    result.map(|_| ())
}

// Line 382
fn c38_l382_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c38_l382_action_invoke");
    let result = instance.call("as-call_indirect-first", &[]);
    assert_eq!(result, Ok(Some(Value::I32(21 as i32))));
    result.map(|_| ())
}

// Line 383
fn c39_l383_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c39_l383_action_invoke");
    let result = instance.call("as-call_indirect-mid", &[]);
    assert_eq!(result, Ok(Some(Value::I32(22 as i32))));
    result.map(|_| ())
}

// Line 384
fn c40_l384_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c40_l384_action_invoke");
    let result = instance.call("as-call_indirect-last", &[]);
    assert_eq!(result, Ok(Some(Value::I32(23 as i32))));
    result.map(|_| ())
}

// Line 386
fn c41_l386_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c41_l386_action_invoke");
    let result = instance.call("as-set_local-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(17 as i32))));
    result.map(|_| ())
}

// Line 387
fn c42_l387_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c42_l387_action_invoke");
    let result = instance.call("as-tee_local-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(1 as i32))));
    result.map(|_| ())
}

// Line 388
fn c43_l388_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c43_l388_action_invoke");
    let result = instance.call("as-set_global-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(1 as i32))));
    result.map(|_| ())
}

// Line 390
fn c44_l390_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c44_l390_action_invoke");
    let result = instance.call("as-load-address", &[]);
    assert_eq!(result, Ok(Some(Value::F32((1.7f32).to_bits()))));
    result.map(|_| ())
}

// Line 391
fn c45_l391_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c45_l391_action_invoke");
    let result = instance.call("as-loadN-address", &[]);
    assert_eq!(result, Ok(Some(Value::I64(30 as i64))));
    result.map(|_| ())
}

// Line 393
fn c46_l393_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c46_l393_action_invoke");
    let result = instance.call("as-store-address", &[]);
    assert_eq!(result, Ok(Some(Value::I32(30 as i32))));
    result.map(|_| ())
}

// Line 394
fn c47_l394_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c47_l394_action_invoke");
    let result = instance.call("as-store-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(31 as i32))));
    result.map(|_| ())
}

// Line 395
fn c48_l395_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c48_l395_action_invoke");
    let result = instance.call("as-storeN-address", &[]);
    assert_eq!(result, Ok(Some(Value::I32(32 as i32))));
    result.map(|_| ())
}

// Line 396
fn c49_l396_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c49_l396_action_invoke");
    let result = instance.call("as-storeN-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(33 as i32))));
    result.map(|_| ())
}

// Line 398
fn c50_l398_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c50_l398_action_invoke");
    let result = instance.call("as-unary-operand", &[]);
    assert_eq!(result, Ok(Some(Value::F32((3.4f32).to_bits()))));
    result.map(|_| ())
}

// Line 400
fn c51_l400_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c51_l400_action_invoke");
    let result = instance.call("as-binary-left", &[]);
    assert_eq!(result, Ok(Some(Value::I32(3 as i32))));
    result.map(|_| ())
}

// Line 401
fn c52_l401_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c52_l401_action_invoke");
    let result = instance.call("as-binary-right", &[]);
    assert_eq!(result, Ok(Some(Value::I64(45 as i64))));
    result.map(|_| ())
}

// Line 403
fn c53_l403_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c53_l403_action_invoke");
    let result = instance.call("as-test-operand", &[]);
    assert_eq!(result, Ok(Some(Value::I32(44 as i32))));
    result.map(|_| ())
}

// Line 405
fn c54_l405_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c54_l405_action_invoke");
    let result = instance.call("as-compare-left", &[]);
    assert_eq!(result, Ok(Some(Value::I32(43 as i32))));
    result.map(|_| ())
}

// Line 406
fn c55_l406_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c55_l406_action_invoke");
    let result = instance.call("as-compare-right", &[]);
    assert_eq!(result, Ok(Some(Value::I32(42 as i32))));
    result.map(|_| ())
}

// Line 408
fn c56_l408_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c56_l408_action_invoke");
    let result = instance.call("as-convert-operand", &[]);
    assert_eq!(result, Ok(Some(Value::I32(41 as i32))));
    result.map(|_| ())
}

// Line 410
fn c57_l410_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c57_l410_action_invoke");
    let result = instance.call("as-memory.grow-size", &[]);
    assert_eq!(result, Ok(Some(Value::I32(40 as i32))));
    result.map(|_| ())
}

// Line 412
fn c58_l412_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c58_l412_action_invoke");
    let result = instance.call("nested-block-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 413
fn c59_l413_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c59_l413_action_invoke");
    let result = instance.call("nested-br-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 414
fn c60_l414_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c60_l414_action_invoke");
    let result = instance.call("nested-br_if-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 415
fn c61_l415_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c61_l415_action_invoke");
    let result = instance.call("nested-br_if-value-cond", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 416
fn c62_l416_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c62_l416_action_invoke");
    let result = instance.call("nested-br_table-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 417
fn c63_l417_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c63_l417_action_invoke");
    let result = instance.call("nested-br_table-value-index", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 420
#[test]
fn c64_l420_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 0, 1, 127, 3, 2, 1, 0, 10, 11, 1, 9, 0, 2, 127, 12, 0, 65, 1, 11, 11];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

// Line 427
#[test]
fn c65_l427_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 0, 1, 127, 3, 2, 1, 0, 10, 12, 1, 10, 0, 2, 127, 1, 12, 0, 65, 1, 11, 11];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

// Line 433
#[test]
fn c66_l433_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 0, 1, 127, 3, 2, 1, 0, 10, 14, 1, 12, 0, 2, 127, 65, 0, 2, 64, 12, 1, 11, 11, 11];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

// Line 439
#[test]
fn c67_l439_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 0, 1, 127, 3, 2, 1, 0, 10, 13, 1, 11, 0, 2, 127, 66, 1, 12, 0, 65, 1, 11, 11];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

// Line 446
#[test]
fn c68_l446_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 6, 1, 4, 0, 12, 1, 11];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

// Line 450
#[test]
fn c69_l450_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 12, 1, 10, 0, 2, 64, 2, 64, 12, 5, 11, 11, 11];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

// Line 454
#[test]
fn c70_l454_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 10, 1, 8, 0, 12, 129, 128, 128, 128, 1, 11];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

#[test]
fn test_module_1() {
    let mut instance = create_module_1();
    // We group the calls together
    start_module_1(&mut instance);
    c1_l334_action_invoke(&mut instance);
    c2_l335_action_invoke(&mut instance);
    c3_l336_action_invoke(&mut instance);
    c4_l337_action_invoke(&mut instance);
    c5_l339_action_invoke(&mut instance);
    c6_l340_action_invoke(&mut instance);
    c7_l341_action_invoke(&mut instance);
    c8_l342_action_invoke(&mut instance);
    c9_l344_action_invoke(&mut instance);
    c10_l345_action_invoke(&mut instance);
    c11_l346_action_invoke(&mut instance);
    c12_l347_action_invoke(&mut instance);
    c13_l349_action_invoke(&mut instance);
    c14_l350_action_invoke(&mut instance);
    c15_l351_action_invoke(&mut instance);
    c16_l353_action_invoke(&mut instance);
    c17_l355_action_invoke(&mut instance);
    c18_l356_action_invoke(&mut instance);
    c19_l357_action_invoke(&mut instance);
    c20_l359_action_invoke(&mut instance);
    c21_l360_action_invoke(&mut instance);
    c22_l361_action_invoke(&mut instance);
    c23_l363_action_invoke(&mut instance);
    c24_l365_action_invoke(&mut instance);
    c25_l366_action_invoke(&mut instance);
    c26_l367_action_invoke(&mut instance);
    c27_l368_action_invoke(&mut instance);
    c28_l369_action_invoke(&mut instance);
    c29_l371_action_invoke(&mut instance);
    c30_l372_action_invoke(&mut instance);
    c31_l373_action_invoke(&mut instance);
    c32_l374_action_invoke(&mut instance);
    c33_l375_action_invoke(&mut instance);
    c34_l377_action_invoke(&mut instance);
    c35_l378_action_invoke(&mut instance);
    c36_l379_action_invoke(&mut instance);
    c37_l381_action_invoke(&mut instance);
    c38_l382_action_invoke(&mut instance);
    c39_l383_action_invoke(&mut instance);
    c40_l384_action_invoke(&mut instance);
    c41_l386_action_invoke(&mut instance);
    c42_l387_action_invoke(&mut instance);
    c43_l388_action_invoke(&mut instance);
    c44_l390_action_invoke(&mut instance);
    c45_l391_action_invoke(&mut instance);
    c46_l393_action_invoke(&mut instance);
    c47_l394_action_invoke(&mut instance);
    c48_l395_action_invoke(&mut instance);
    c49_l396_action_invoke(&mut instance);
    c50_l398_action_invoke(&mut instance);
    c51_l400_action_invoke(&mut instance);
    c52_l401_action_invoke(&mut instance);
    c53_l403_action_invoke(&mut instance);
    c54_l405_action_invoke(&mut instance);
    c55_l406_action_invoke(&mut instance);
    c56_l408_action_invoke(&mut instance);
    c57_l410_action_invoke(&mut instance);
    c58_l412_action_invoke(&mut instance);
    c59_l413_action_invoke(&mut instance);
    c60_l414_action_invoke(&mut instance);
    c61_l415_action_invoke(&mut instance);
    c62_l416_action_invoke(&mut instance);
    c63_l417_action_invoke(&mut instance);
}
