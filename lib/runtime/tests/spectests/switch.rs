// Rust test file autogenerated with cargo build (build/spectests.rs).
// Please do NOT modify it by hand, as it will be reset on next build.
// Test based on spectests/switch.wast
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


// Line 1
fn create_module_1() -> Box<Instance> {
    let module_str = "(module
      (type (;0;) (func (param i32) (result i32)))
      (type (;1;) (func (param i64) (result i64)))
      (type (;2;) (func (result i32)))
      (func (;0;) (type 0) (param i32) (result i32)
        (local i32)
        i32.const 100
        set_local 1
        block  ;; label = @1
          block  ;; label = @2
            block  ;; label = @3
              block  ;; label = @4
                block  ;; label = @5
                  block  ;; label = @6
                    block  ;; label = @7
                      block  ;; label = @8
                        block  ;; label = @9
                          block  ;; label = @10
                            get_local 0
                            br_table 0 (;@10;) 1 (;@9;) 2 (;@8;) 3 (;@7;) 4 (;@6;) 5 (;@5;) 6 (;@4;) 8 (;@2;) 7 (;@3;)
                          end
                          get_local 0
                          return
                        end
                        nop
                      end
                    end
                    i32.const 0
                    get_local 0
                    i32.sub
                    set_local 1
                    br 5 (;@1;)
                  end
                  br 4 (;@1;)
                end
                i32.const 101
                set_local 1
                br 3 (;@1;)
              end
              i32.const 101
              set_local 1
            end
            i32.const 102
            set_local 1
          end
        end
        get_local 1
        return)
      (func (;1;) (type 1) (param i64) (result i64)
        (local i64)
        i64.const 100
        set_local 1
        block (result i64)  ;; label = @1
          block  ;; label = @2
            block  ;; label = @3
              block  ;; label = @4
                block  ;; label = @5
                  block  ;; label = @6
                    block  ;; label = @7
                      block  ;; label = @8
                        block  ;; label = @9
                          block  ;; label = @10
                            get_local 0
                            i32.wrap/i64
                            br_table 0 (;@10;) 1 (;@9;) 2 (;@8;) 3 (;@7;) 6 (;@4;) 5 (;@5;) 4 (;@6;) 8 (;@2;) 7 (;@3;)
                          end
                          get_local 0
                          return
                        end
                        nop
                      end
                    end
                    i64.const 0
                    get_local 0
                    i64.sub
                    br 5 (;@1;)
                  end
                  i64.const 101
                  set_local 1
                end
              end
            end
            get_local 1
            br 1 (;@1;)
          end
          i64.const -5
        end
        return)
      (func (;2;) (type 0) (param i32) (result i32)
        block (result i32)  ;; label = @1
          i32.const 10
          block (result i32)  ;; label = @2
            i32.const 100
            block (result i32)  ;; label = @3
              i32.const 1000
              block (result i32)  ;; label = @4
                i32.const 2
                get_local 0
                i32.mul
                i32.const 3
                get_local 0
                i32.and
                br_table 1 (;@3;) 2 (;@2;) 3 (;@1;) 0 (;@4;)
              end
              i32.add
            end
            i32.add
          end
          i32.add
        end
        return)
      (func (;3;) (type 2) (result i32)
        block  ;; label = @1
          i32.const 0
          br_table 0 (;@1;)
        end
        i32.const 1)
      (export \"stmt\" (func 0))
      (export \"expr\" (func 1))
      (export \"arg\" (func 2))
      (export \"corner\" (func 3)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new()).expect("WASM can't be compiled");
    module.instantiate(&spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_1(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 120
fn c1_l120_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c1_l120_action_invoke");
    let result = instance.call("stmt", &[Value::I32(0 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(0 as i32))));
    result.map(|_| ())
}

// Line 121
fn c2_l121_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c2_l121_action_invoke");
    let result = instance.call("stmt", &[Value::I32(1 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(-1 as i32))));
    result.map(|_| ())
}

// Line 122
fn c3_l122_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c3_l122_action_invoke");
    let result = instance.call("stmt", &[Value::I32(2 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(-2 as i32))));
    result.map(|_| ())
}

// Line 123
fn c4_l123_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c4_l123_action_invoke");
    let result = instance.call("stmt", &[Value::I32(3 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(-3 as i32))));
    result.map(|_| ())
}

// Line 124
fn c5_l124_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c5_l124_action_invoke");
    let result = instance.call("stmt", &[Value::I32(4 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(100 as i32))));
    result.map(|_| ())
}

// Line 125
fn c6_l125_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c6_l125_action_invoke");
    let result = instance.call("stmt", &[Value::I32(5 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(101 as i32))));
    result.map(|_| ())
}

// Line 126
fn c7_l126_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c7_l126_action_invoke");
    let result = instance.call("stmt", &[Value::I32(6 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(102 as i32))));
    result.map(|_| ())
}

// Line 127
fn c8_l127_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c8_l127_action_invoke");
    let result = instance.call("stmt", &[Value::I32(7 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(100 as i32))));
    result.map(|_| ())
}

// Line 128
fn c9_l128_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c9_l128_action_invoke");
    let result = instance.call("stmt", &[Value::I32(-10 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(102 as i32))));
    result.map(|_| ())
}

// Line 130
fn c10_l130_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c10_l130_action_invoke");
    let result = instance.call("expr", &[Value::I64(0 as i64)]);
    assert_eq!(result, Ok(Some(Value::I64(0 as i64))));
    result.map(|_| ())
}

// Line 131
fn c11_l131_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c11_l131_action_invoke");
    let result = instance.call("expr", &[Value::I64(1 as i64)]);
    assert_eq!(result, Ok(Some(Value::I64(-1 as i64))));
    result.map(|_| ())
}

// Line 132
fn c12_l132_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c12_l132_action_invoke");
    let result = instance.call("expr", &[Value::I64(2 as i64)]);
    assert_eq!(result, Ok(Some(Value::I64(-2 as i64))));
    result.map(|_| ())
}

// Line 133
fn c13_l133_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c13_l133_action_invoke");
    let result = instance.call("expr", &[Value::I64(3 as i64)]);
    assert_eq!(result, Ok(Some(Value::I64(-3 as i64))));
    result.map(|_| ())
}

// Line 134
fn c14_l134_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c14_l134_action_invoke");
    let result = instance.call("expr", &[Value::I64(6 as i64)]);
    assert_eq!(result, Ok(Some(Value::I64(101 as i64))));
    result.map(|_| ())
}

// Line 135
fn c15_l135_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c15_l135_action_invoke");
    let result = instance.call("expr", &[Value::I64(7 as i64)]);
    assert_eq!(result, Ok(Some(Value::I64(-5 as i64))));
    result.map(|_| ())
}

// Line 136
fn c16_l136_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c16_l136_action_invoke");
    let result = instance.call("expr", &[Value::I64(-10 as i64)]);
    assert_eq!(result, Ok(Some(Value::I64(100 as i64))));
    result.map(|_| ())
}

// Line 138
fn c17_l138_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c17_l138_action_invoke");
    let result = instance.call("arg", &[Value::I32(0 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(110 as i32))));
    result.map(|_| ())
}

// Line 139
fn c18_l139_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c18_l139_action_invoke");
    let result = instance.call("arg", &[Value::I32(1 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(12 as i32))));
    result.map(|_| ())
}

// Line 140
fn c19_l140_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c19_l140_action_invoke");
    let result = instance.call("arg", &[Value::I32(2 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(4 as i32))));
    result.map(|_| ())
}

// Line 141
fn c20_l141_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c20_l141_action_invoke");
    let result = instance.call("arg", &[Value::I32(3 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(1116 as i32))));
    result.map(|_| ())
}

// Line 142
fn c21_l142_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c21_l142_action_invoke");
    let result = instance.call("arg", &[Value::I32(4 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(118 as i32))));
    result.map(|_| ())
}

// Line 143
fn c22_l143_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c22_l143_action_invoke");
    let result = instance.call("arg", &[Value::I32(5 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(20 as i32))));
    result.map(|_| ())
}

// Line 144
fn c23_l144_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c23_l144_action_invoke");
    let result = instance.call("arg", &[Value::I32(6 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(12 as i32))));
    result.map(|_| ())
}

// Line 145
fn c24_l145_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c24_l145_action_invoke");
    let result = instance.call("arg", &[Value::I32(7 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(1124 as i32))));
    result.map(|_| ())
}

// Line 146
fn c25_l146_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c25_l146_action_invoke");
    let result = instance.call("arg", &[Value::I32(8 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(126 as i32))));
    result.map(|_| ())
}

// Line 148
fn c26_l148_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c26_l148_action_invoke");
    let result = instance.call("corner", &[]);
    assert_eq!(result, Ok(Some(Value::I32(1 as i32))));
    result.map(|_| ())
}

// Line 150
#[test]
fn c27_l150_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 9, 1, 7, 0, 65, 0, 14, 0, 3, 11];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

#[test]
fn test_module_1() {
    let mut instance = create_module_1();
    // We group the calls together
    start_module_1(&mut instance);
    c1_l120_action_invoke(&mut instance);
    c2_l121_action_invoke(&mut instance);
    c3_l122_action_invoke(&mut instance);
    c4_l123_action_invoke(&mut instance);
    c5_l124_action_invoke(&mut instance);
    c6_l125_action_invoke(&mut instance);
    c7_l126_action_invoke(&mut instance);
    c8_l127_action_invoke(&mut instance);
    c9_l128_action_invoke(&mut instance);
    c10_l130_action_invoke(&mut instance);
    c11_l131_action_invoke(&mut instance);
    c12_l132_action_invoke(&mut instance);
    c13_l133_action_invoke(&mut instance);
    c14_l134_action_invoke(&mut instance);
    c15_l135_action_invoke(&mut instance);
    c16_l136_action_invoke(&mut instance);
    c17_l138_action_invoke(&mut instance);
    c18_l139_action_invoke(&mut instance);
    c19_l140_action_invoke(&mut instance);
    c20_l141_action_invoke(&mut instance);
    c21_l142_action_invoke(&mut instance);
    c22_l143_action_invoke(&mut instance);
    c23_l144_action_invoke(&mut instance);
    c24_l145_action_invoke(&mut instance);
    c25_l146_action_invoke(&mut instance);
    c26_l148_action_invoke(&mut instance);
}
