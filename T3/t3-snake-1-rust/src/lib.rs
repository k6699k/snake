use wasm_bindgen::prelude::*;
pub fn cal_food_distance(
    fruit_x:i32,
    fruit_y:i32,
    snake: &[i32]
) -> i32 {
    // 解析蛇的头部和身体
    let head = (snake[0], snake[1]);
    let body = vec![
        (snake[2], snake[3]),
        (snake[4], snake[5]),
        (snake[6], snake[7]),
    ];

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

    return min_distance
}

pub fn is_close_to_other_snake(
    locate_x:i32,
    locate_y:i32,
    other_snakes: &[i32]
)  -> bool{
    for i in (0..other_snakes.len()).step_by(8) {
        if (locate_x-other_snakes[i]).abs() + (locate_y - other_snakes[i+1]).abs() == 1 {
            return true;
        }
    }
    false
}
//这个函数用于避让争夺果子的情况 可以考虑关闭

#[wasm_bindgen]
pub fn greedy_snake_step(
    board_size: i32,
    snake: &[i32],
    snake_num: i32,
    other_snakes: &[i32],
    food_num: i32,
    foods: &[i32],
    round: i32
) -> i32 {
    // 定义方向：0上，1左，2下，3右
    let directions = [(0, 1), (-1, 0), (0, -1), (1, 0)];

    // 蛇头坐标
    let head_x = snake[0];
    let head_y = snake[1];

    // 收集所有障碍物坐标
    let mut obstacles = Vec::new();
    // 添加边界
    for x in 0..board_size + 1 {
        obstacles.push((x, 0));
        obstacles.push((x, board_size + 1));
        obstacles.push((0, x));
        obstacles.push((board_size + 1, x));
    }
    // 添加自己的身体（除头部外）
    for i in (0..snake.len() - 2).step_by(2) {
        obstacles.push((snake[i], snake[i+1]));
    }
    // 添加其他蛇的身体在下一轮次必然出现的位置
    for i in (0..other_snakes.len()).step_by(8) {
        for j in (0..6).step_by(2) {
            obstacles.push((other_snakes[i+j], other_snakes[i+j+1]));
        }
    }

    // 提前计算其他蛇的可能头部位置
    let mut potential_snake_heads = Vec::new();
    for i in (0..other_snakes.len()).step_by(8) {
        potential_snake_heads.push((other_snakes[i] + 1, other_snakes[i+1] + 1));
        potential_snake_heads.push((other_snakes[i] + 1, other_snakes[i+1] - 1));
        potential_snake_heads.push((other_snakes[i] - 1, other_snakes[i+1] + 1));
        potential_snake_heads.push((other_snakes[i] - 1, other_snakes[i+1] - 1));
    }

    //重整果子位置
    let mut fruits = Vec::new();
    for i in (0..foods.len()).step_by(2) {
        fruits.push((foods[i],foods[i+1]));
    }

    // 评估每个方向
    let mut best_direction = 114514;
    let mut best_score = f64::MIN;
    let mut dingerous = Vec::new();
    for (dir_index, (dx, dy)) in directions.iter().enumerate() {
        let new_x = head_x + dx;
        let new_y = head_y + dy;

        // 避免碰撞
        if obstacles.contains(&(new_x, new_y)) {
            continue;
        }

        let mut score = 0.0;
        // 找最近的食物并给分
        for (dir_index, &(food_x, food_y)) in fruits.iter().enumerate(){
            // 直接吃到食物
            if new_x == food_x && new_y == food_y {
                score += 1000.0;
            } else {
                // 靠近食物得分
                let dist_to_food = ((new_y - food_y).abs() + (new_x - food_x).abs()) as f64;
                score += 10.0 / (dist_to_food + 1.0);
            }
        }

        // 多蛇对战额外策略
        // 1. 尽量不靠近其他蛇
        for i in (0..other_snakes.len()).step_by(8) {
            let other_head_x = other_snakes[i];
            let other_head_y = other_snakes[i+1];

            let dist_to_other = ((new_x - other_head_x).abs() + (new_y - other_head_y).abs()) as f64;

            // 远离其他蛇加分
            if dist_to_other > 2.0 {
                score += 3.0 / (dist_to_other + 1.0);
            } else {
                // 非常接近其他蛇扣分
                score -= 12.0 / (dist_to_other + 1.0);
            }
        }

        // 2. 避免被包围,保留更多可选路径的移动会得到额外分数
        let mut safe_moves_count = 0;
        for (dir_index, (possible_dx, possible_dy)) in directions.iter().enumerate(){
            let check_x = new_x + possible_dx;
            let check_y = new_y + possible_dy;
            if !obstacles.contains(&(check_x, check_y)) && !potential_snake_heads.contains(&(check_x, check_y)) {
                safe_moves_count += 1;
            }
        }
        score += safe_moves_count as f64 * 2.0;

        // 3. 靠近边缘？？？？？？
        score += ((new_x - board_size/2).abs() + (new_y - board_size/2).abs()) as f64 * 1.0;

        // 4. 保守策略，如果前进的方向有果子而且与其他蛇蛇头相邻，则暂时弃置
        if fruits.contains(&(new_x, new_y)) && is_close_to_other_snake(new_x,new_y,other_snakes){
            dingerous.push((score, dir_index));
            continue;
        }

        // 选择分数最高的方向
        if score > best_score {
            best_score = score;
            best_direction = dir_index;
        }
    }

    if best_direction == 114514 {
        if dingerous.len() == 0 {
            return 0;
        } else {
            best_direction = 114514;
            best_score = f64::MIN;
            for(index,&(score,direction)) in dingerous.iter().enumerate() {
                if(score > best_score) {
                    best_direction = direction;
                    best_score = score;
                }
            }
        }
    }
    best_direction as i32
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn move_test() {
        assert_eq!(greedy_snake_step(
            5,
            &[2,2,2,3,2,4,2,5],
            1,
            &[5,5,5,4,5,3,5,2],
            3,
            &[4,1,5,1,1,2],
            1
        ), 1);

        assert_eq!(greedy_snake_step(
            5,
            &[2,4,2,5,3,5,3,4],
            1,
            &[5,5,5,4,5,3,5,2],
            3,
            &[1,4,3,4,2,2],
            1
        ), 1);

        assert_eq!(greedy_snake_step(
            5,
            &[2,4,2,5,3,5,3,4],
            1,
            &[5,2,4,2,3,2,2,2],
            2,
            &[2,3,1,4],
            1
        ), 2);
    }
}
