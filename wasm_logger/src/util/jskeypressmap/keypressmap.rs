use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::ops::Deref;

// 模拟 direction.Direction_Type 枚举
#[derive(Clone, Copy)]
enum DirectionType {
    Dir_sw,
    Dir_s,
    Dir_se,
    Dir_w,
    Dir_stop,
    Dir_e,
    Dir_nw,
    Dir_n,
    Dir_ne,
}

impl DirectionType {
    fn dx(self) -> i32 {
        match self {
            DirectionType::Dir_sw => -1,
            DirectionType::Dir_s => 0,
            DirectionType::Dir_se => 1,
            DirectionType::Dir_w => -1,
            DirectionType::Dir_stop => 0,
            DirectionType::Dir_e => 1,
            DirectionType::Dir_nw => -1,
            DirectionType::Dir_n => 0,
            DirectionType::Dir_ne => 1,
        }
    }

    fn dy(self) -> i32 {
        match self {
            DirectionType::Dir_sw => 1,
            DirectionType::Dir_s => 1,
            DirectionType::Dir_se => 1,
            DirectionType::Dir_w => 0,
            DirectionType::Dir_stop => 0,
            DirectionType::Dir_e => 0,
            DirectionType::Dir_nw => -1,
            DirectionType::Dir_n => -1,
            DirectionType::Dir_ne => -1,
        }
    }
}

// 定义 KeyPressMap 结构体
#[derive(Default)]
struct KeyPressMap {
    keyboard_pressed_map: Arc<Mutex<HashMap<String, bool>>>,
}

impl KeyPressMap {
    fn new() -> Self {
        KeyPressMap {
            keyboard_pressed_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn key_up(&self, kcode: &str) {
        let mut map = self.keyboard_pressed_map.lock().unwrap();
        map.remove(kcode);
    }

    fn key_down(&self, kcode: &str) {
        let mut map = self.keyboard_pressed_map.lock().unwrap();
        map.insert(kcode.to_string(), true);
    }

    fn is_pressed(&self, kcode: &str) -> bool {
        let map = self.keyboard_pressed_map.lock().unwrap();
        map.get(kcode).cloned().unwrap_or(false)
    }

    fn sum_move_dx_dy(&self, key2dir: &HashMap<String, DirectionType>) -> (i32, i32) {
        let map = self.keyboard_pressed_map.lock().unwrap();
        let mut dx = 0;
        let mut dy = 0;
        for (key, dir) in key2dir.iter() {
            if map.get(key).cloned().unwrap_or(false) {
                dx += dir.dx();
                dy += dir.dy();
            }
        }
        (dx, dy)
    }
}

fn main() {
    let key_press_map = KeyPressMap::new();

    // 示例键映射
    let key2dir = [
        ("End", DirectionType::Dir_sw),
        ("ArrowDown", DirectionType::Dir_s),
        ("PageDown", DirectionType::Dir_se),
        ("ArrowLeft", DirectionType::Dir_w),
        ("Clear", DirectionType::Dir_stop),
        ("ArrowRight", DirectionType::Dir_e),
        ("Home", DirectionType::Dir_nw),
        ("ArrowUp", DirectionType::Dir_n),
        ("PageUp", DirectionType::Dir_ne),
    ]
    .iter()
    .cloned()
    .collect::<HashMap<_, _>>();

    // 模拟按键按下和释放
    key_press_map.key_down("ArrowUp");
    key_press_map.key_down("ArrowRight");

    // 计算移动方向
    let (dx, dy) = key_press_map.sum_move_dx_dy(&key2dir);
    println!("Movement: ({}, {})", dx, dy);

    // 模拟按键释放
    key_press_map.key_up("ArrowUp");
    key_press_map.key_up("ArrowRight");

    // 再次计算移动方向
    let (dx, dy) = key_press_map.sum_move_dx_dy(&key2dir);
    println!("Movement: ({}, {})", dx, dy);
}