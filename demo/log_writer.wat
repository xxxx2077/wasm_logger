(module
  (import "wasi_snapshot_preview1" "fd_write" (func $fd_write (param i32 i32 i32 i32) (result i32)))

  (func $main
    ;; 定义局部变量来存储 fd_write 的返回值
    (local i32)

    ;; 准备写入的数据
    (i32.store offset=0 (i32.const 0) (i32.const 115))  ;; 's'
    (i32.store offset=1 (i32.const 0) (i32.const 104))  ;; 'h'
    (i32.store offset=2 (i32.const 0) (i32.const 101))  ;; 'e'
    (i32.store offset=3 (i32.const 0) (i32.const 108))  ;; 'l'
    (i32.store offset=4 (i32.const 0) (i32.const 108))  ;; 'l'
    (i32.store offset=5 (i32.const 0) (i32.const 111))  ;; 'o'
    (i32.store offset=6 (i32.const 0) (i32.const 44))   ;; ','
    (i32.store offset=7 (i32.const 0) (i32.const 32))   ;; ' '
    (i32.store offset=8 (i32.const 0) (i32.const 119))  ;; 'w'
    (i32.store offset=9 (i32.const 0) (i32.const 111))  ;; 'o'
    (i32.store offset=10 (i32.const 0) (i32.const 114)) ;; 'r'
    (i32.store offset=11 (i32.const 0) (i32.const 108)) ;; 'l'
    (i32.store offset=12 (i32.const 0) (i32.const 100)) ;; 'd'
    (i32.store offset=13 (i32.const 0) (i32.const 33))  ;; '!'

    ;; 准备 iov
    (i32.store offset=0 (i32.const 16) (i32.const 0))  ;; iov.iov_base
    (i32.store offset=4 (i32.const 16) (i32.const 13)) ;; iov.iov_len

    ;; 调用 fd_write 并存储返回值
    (local.set 0 (call $fd_write (i32.const 1) (i32.const 16) (i32.const 1) (i32.const 13)))
  )

  (memory 1)
  (export "memory" (memory 0))
  (export "_start" (func $main))
)