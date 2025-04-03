use wasm_bindgen::prelude::*;
use std::collections::{VecDeque, HashSet};

#[wasm_bindgen]
pub fn greedy_snake_move_barriers(snake: Vec<i32>, fruit: Vec<i32>, barriers: Vec<i32>) -> i32 {
    if snake.len() != 8 || fruit.len() != 2 || barriers.len() != 24 {
        return -1; // 输入异常
    }

    let head = (snake[0], snake[1]);
    let body = vec![
        (snake[2], snake[3]),
        (snake[4], snake[5]),
        (snake[6], snake[7]),
    ];
    let fruit_pos = (fruit[0], fruit[1]);

    // 构建障碍物集合
    let mut obstacles = HashSet::new();
    for i in (0..barriers.len()).step_by(2) {
        obstacles.insert((barriers[i], barriers[i + 1]));
    }

    // BFS 计算最短路径
    let directions = [
        (0, 1),  // 上 (0)
        (-1, 0), // 左 (1)
        (0, -1), // 下 (2)
        (1, 0),  // 右 (3)
    ];

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut parent = std::collections::HashMap::new();

    queue.push_back(head);
    visited.insert(head);
    parent.insert(head, None);

    while let Some((x, y)) = queue.pop_front() {
        if (x, y) == fruit_pos {
            break;
        }

        for &(dx, dy) in &directions {
            let nx = x + dx;
            let ny = y + dy;
            let next_pos = (nx, ny);

            // 确保新位置合法
            if nx < 1 || nx > 8 || ny < 1 || ny > 8 {
                continue;
            }
            if obstacles.contains(&next_pos) || body.contains(&next_pos) {
                continue;
            }
            if visited.contains(&next_pos) {
                continue;
            }

            queue.push_back(next_pos);
            visited.insert(next_pos);
            parent.insert(next_pos, Some((x, y)));
        }
    }

    // 如果找不到路径，返回 -1
    if !parent.contains_key(&fruit_pos) {
        return -1;
    }

    // 反向寻找蛇的下一步
    let mut target = fruit_pos;
    while let Some(&Some(prev)) = parent.get(&target) {
        if prev == head {
            break;
        }
        target = prev;
    }

    // 计算方向
    let (nx, ny) = target;
    match (nx - head.0, ny - head.1) {
        (0, 1) => 0,  // 上
        (-1, 0) => 1, // 左
        (0, -1) => 2, // 下
        (1, 0) => 3,  // 右
        _ => -1,      // 不应该发生
    }
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
