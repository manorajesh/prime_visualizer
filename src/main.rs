use macroquad::prelude::*;

fn get_primes(num: usize) -> Vec<bool> {
    if num < 2 {
        return vec![];
    }

    // Create a sieve where we only care about odd numbers (except for 2)
    let mut nums: Vec<bool> = vec![true; num];
    nums[0] = false; // 0 is not a prime
    nums[1] = false; // 1 is not a prime

    // Set even numbers (except for 2) to false
    for i in (4..num).step_by(2) {
        nums[i] = false;
    }

    let limit = ((num as f64).sqrt() as usize) + 1;

    for i in (3..limit).step_by(2) {
        if nums[i] {
            // Mark all multiples of i, starting from i*i, as not prime
            for j in (i * i..num).step_by(i * 2) {
                nums[j] = false;
            }
        }
    }

    nums
}

// get cell size that makes a 1d vec represented as a square
fn get_cell_size(vec_len: usize) -> f32 {
    let width = screen_width();
    let height = screen_height();
    let cell_size = ((width * height) / (vec_len as f32)).sqrt();
    cell_size
}

#[macroquad::main("Primes")]
async fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let num = if args.len() > 1 { args[1].parse::<usize>().unwrap() } else { 10000 };
    let nums: Vec<bool> = get_primes(num);

    loop {
        clear_background(BLACK);

        let cell_size = get_cell_size(nums.len());
        for (idx, num) in nums.clone().into_iter().enumerate() {
            if num {
                // Calculate coordinates
                let x = ((idx as f32) % (screen_width() / cell_size).floor()) * cell_size;
                let y = ((idx as f32) / (screen_width() / cell_size).floor()).floor() * cell_size;

                // Draw filled rectangle
                draw_rectangle(x, y, cell_size, cell_size, RED);
            } else {
                // Optionally draw border only (commented out)
                // draw_rectangle_lines(x, y, cell_size, cell_size, 3.0, BLUE);
            }
        }

        next_frame().await;
    }
}
