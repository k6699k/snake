use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greedy_snake_move(snake: Vec<i32>, fruit: Vec<i32>) -> i32 {
    if snake.len() != 8 || fruit.len() != 2 {
        return -1; // 非法输入，返回默认值
    }

    // 解析蛇的头部和身体
    let head = (snake[0], snake[1]);
    let body = vec![
        (snake[2], snake[3]),
        (snake[4], snake[5]),
        (snake[6], snake[7]),
    ];

    // 解析果子坐标
    let fruit_x = fruit[0];
    let fruit_y = fruit[1];

    // 辅助函数：判断某个位置是否可移动
    fn is_valid_move(x: i32, y: i32, body: &Vec<(i32, i32)>, max_x: i32, max_y: i32) -> bool {
        if x < 1 || x > max_x || y < 1 || y > max_y {
            return false; // 超出边界
        }
        for segment in body {
            if segment.0 == x && segment.1 == y {
                return false; // 不能撞到自己的身体
            }
        }
        true
    }

    // 定义蛇的四个可能移动方向
    let directions = [
        (head.0, head.1 + 1), // 上 (0)
        (head.0 - 1, head.1), // 左 (1)
        (head.0, head.1 - 1), // 下 (2)
        (head.0 + 1, head.1), // 右 (3)
    ];

    // 选择最佳方向
    let mut best_direction = -1;
    let mut min_distance = i32::MAX;

    for (i, (x, y)) in directions.iter().enumerate() {
        if is_valid_move(*x, *y, &body, 8, 8) {
            let distance = (fruit_x - *x).abs() + (fruit_y - *y).abs(); // 曼哈顿距离
            if distance < min_distance {
                min_distance = distance;
                best_direction = i as i32;
            }
        }
    }

    best_direction
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
