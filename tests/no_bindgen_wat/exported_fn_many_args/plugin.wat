(module
  (func $add_ten_all (export "add_ten_all") (param $p0 i32) (param $p1 i64) (result i32 i64)
    (i32.add (local.get $p0) (i32.const 10))
    (i64.add (local.get $p1) (i64.const 10))
  )
)


  ;; (func $add_ten_all_extern (export "add_ten_all_extern") (type $t1) (param $p0 i32) (param $p1 i64) (param $p2 i32) (param $p3 i64) (param $p4 f32) (param $p5 f64) (result i64 i64 i32 i32 f32 i32 f64)
  ;;   (i64.add
  ;;     (local.get $p1)
  ;;     (i64.const 10))
  ;;   (i64.add
  ;;     (local.get $p3)
  ;;     (i64.const 10))
  ;;   (i32.add
  ;;     (local.get $p0)
  ;;     (i32.const 10))
  ;;   (i32.add
  ;;     (local.get $p2)
  ;;     (i32.const 10))
  ;;   (f32.add
  ;;     (local.get $p4)
  ;;     (f32.const 0x1.4p+3 (;=10;)))
  ;;   (local.get $p2)
  ;;   (f64.add
  ;;     (local.get $p5)
  ;;     (f64.const 0x1.4p+3 (;=10;))))