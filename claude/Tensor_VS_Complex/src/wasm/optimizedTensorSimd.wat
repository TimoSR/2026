(module
  (memory (export "memory") 1)

  ;; Memory layout, in bytes:
  ;; amp[16]   -> 0
  ;; freq[16]  -> 64
  ;; phase[16] -> 128
  (func (export "combine") (param $count i32) (param $t f32) (result f32)
    (local $i i32)
    (local $sum v128)
    (local $angle v128)
    (local $cycles v128)
    (local $x v128)
    (local $x2 v128)
    (local $poly v128)

    (local.set $sum (f32x4.splat (f32.const 0)))

    (block $exit
      (loop $loop
        (br_if $exit (i32.ge_u (local.get $i) (local.get $count)))

        (local.set $angle
          (f32x4.add
            (f32x4.mul
              (v128.load offset=64 (i32.shl (local.get $i) (i32.const 2)))
              (f32x4.splat (local.get $t)))
            (v128.load offset=128 (i32.shl (local.get $i) (i32.const 2)))))

        ;; WebAssembly has SIMD lanes, but no native trig op. This kernel
        ;; range-reduces and approximates cos(x) with a vector polynomial.
        (local.set $cycles
          (f32x4.nearest
            (f32x4.mul
              (local.get $angle)
              (f32x4.splat (f32.const 0.15915494309189535)))))
        (local.set $x
          (f32x4.sub
            (local.get $angle)
            (f32x4.mul
              (local.get $cycles)
              (f32x4.splat (f32.const 6.283185307179586)))))

        (local.set $x2 (f32x4.mul (local.get $x) (local.get $x)))
        (local.set $poly (f32x4.splat (f32.const -0.0000002755731922398589)))
        (local.set $poly
          (f32x4.add
            (f32x4.splat (f32.const 0.0000248015873015873))
            (f32x4.mul (local.get $x2) (local.get $poly))))
        (local.set $poly
          (f32x4.add
            (f32x4.splat (f32.const -0.001388888888888889))
            (f32x4.mul (local.get $x2) (local.get $poly))))
        (local.set $poly
          (f32x4.add
            (f32x4.splat (f32.const 0.041666666666666664))
            (f32x4.mul (local.get $x2) (local.get $poly))))
        (local.set $poly
          (f32x4.add
            (f32x4.splat (f32.const -0.5))
            (f32x4.mul (local.get $x2) (local.get $poly))))
        (local.set $poly
          (f32x4.add
            (f32x4.splat (f32.const 1))
            (f32x4.mul (local.get $x2) (local.get $poly))))

        (local.set $sum
          (f32x4.add
            (local.get $sum)
            (f32x4.mul
              (v128.load offset=0 (i32.shl (local.get $i) (i32.const 2)))
              (local.get $poly))))

        (local.set $i (i32.add (local.get $i) (i32.const 4)))
        (br $loop)))

    (f32.add
      (f32.add
        (f32x4.extract_lane 0 (local.get $sum))
        (f32x4.extract_lane 1 (local.get $sum)))
      (f32.add
        (f32x4.extract_lane 2 (local.get $sum))
        (f32x4.extract_lane 3 (local.get $sum)))))
)
