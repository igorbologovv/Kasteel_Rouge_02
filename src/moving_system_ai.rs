use bevy::math::Vec3;

fn define_direction_to_enemy(dir: Vec3, my_pos: Vec3) -> [u8; 8] {
    // Массив направлений: N, NE, E, SE, S, SW, W, NW
    let mut result_dir = [0u8; 8];
    let dx = dir.x - my_pos.x;
    let dy = dir.y - my_pos.y;

    if dx == 0.0 && dy == 0.0 {
        // Солдат и враг находятся в одной точке
        return result_dir; // Все нули
    }

    // Проверка на нулевой dx, чтобы избежать деления на ноль
    if dx == 0.0 {
        if dy > 0.0 {
            result_dir[0] = 1; // Север
        } else if dy < 0.0 {
            result_dir[4] = 1; // Юг
        }
        // Если dy == 0.0, то враг находится прямо на солдате
        return result_dir;
    }

    let slope = dy / dx;

    if dx > 0.0 {
        if dy >= 0.0 {
            // Первый квадрант (NE)
            if slope >= 2.4142 {
                // tan(67.5°)
                result_dir[0] = 1; // Север
            } else if slope >= 0.4142 {
                // tan(22.5°)
                result_dir[1] = 1; // NE
            } else {
                result_dir[2] = 1; // Восток
            }
        } else {
            // Четвертый квадрант (SE)
            if slope <= -2.4142 {
                // tan(-67.5°)
                result_dir[4] = 1; // Юг
            } else if slope <= -0.4142 {
                // tan(-22.5°)
                result_dir[3] = 1; // SE
            } else {
                result_dir[2] = 1; // Восток
            }
        }
    } else {
        if dy >= 0.0 {
            // Второй квадрант (NW)
            if slope <= -2.4142 {
                // tan(-67.5°)
                result_dir[0] = 1; // Север
            } else if slope <= -0.4142 {
                // tan(-22.5°)
                result_dir[7] = 1; // NW
            } else {
                result_dir[6] = 1; // Запад
            }
        } else {
            // Третий квадрант (SW)
            if slope >= 2.4142 {
                // tan(67.5°)
                result_dir[4] = 1; // Юг
            } else if slope >= 0.4142 {
                // tan(22.5°)
                result_dir[5] = 1; // SW
            } else {
                result_dir[6] = 1; // Запад
            }
        }
    }

    result_dir
}

