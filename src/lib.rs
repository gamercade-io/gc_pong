mod console;

use console::*;

const SCREEN_WIDTH: i32 = 320;
const SCREEN_HEIGHT: i32 = 180;

struct GameState {
    paddles: [Paddle; 2],
    ball: Ball,
    ball_color: i32,
    paddle_color: [i32; 2],
}

struct Paddle {
    x: f32,
    y: f32,
}

impl Paddle {
    const WIDTH: i32 = 6;
    const HEIGHT: i32 = 32;
    const START_Y: f32 = (SCREEN_HEIGHT as f32 / 2.0) - Self::HEIGHT as f32;
    const SPEED: f32 = 1.0;

    const fn new(x: f32) -> Self {
        Self {
            x,
            y: Self::START_Y,
        }
    }
}

struct Ball {
    x: f32,
    y: f32,
    x_vel: f32,
    y_vel: f32,
}

impl Ball {
    const RADIUS: i32 = 4;
    const SPEED: f32 = 0.75;

    const fn new() -> Self {
        Self {
            x: (SCREEN_WIDTH / 2) as f32,
            y: (SCREEN_HEIGHT / 2) as f32,
            x_vel: Self::SPEED,
            y_vel: Self::SPEED,
        }
    }
}

static mut GAME_STATE: GameState = GameState {
    paddles: [
        Paddle::new(10.0),
        Paddle::new((SCREEN_WIDTH - (10 + Paddle::WIDTH)) as f32),
    ],
    ball: Ball::new(),
    ball_color: 0,
    paddle_color: [0, 0],
};

#[no_mangle]
pub unsafe extern "C" fn init() {
    GAME_STATE.ball_color = color_index(43);
    GAME_STATE.paddle_color[0] = color_index(31);
    GAME_STATE.paddle_color[1] = color_index(15);
}

#[no_mangle]
pub unsafe extern "C" fn update() {
    let ball = &mut GAME_STATE.ball;
    let player_1 = &mut GAME_STATE.paddles[0];
    let player_2 = &mut GAME_STATE.paddles[1];

    // Physics Simulation
    ball.x += ball.x_vel;
    ball.y += ball.y_vel;

    if (ball.x as i32 + Ball::RADIUS) > SCREEN_WIDTH {
        *ball = Ball::new();
        ball.x_vel *= -1.0;
    }

    if (ball.x as i32 - Ball::RADIUS) < 0 {
        *ball = Ball::new();
    }

    if (ball.y as i32 - (Ball::RADIUS + 1)) < 0 || (ball.y as i32 + Ball::RADIUS > SCREEN_HEIGHT - 1) {
        ball.y_vel *= -1.0;
    }

    if ball.x_vel < 0.0 {
        if intersects(ball, player_1) {
            ball.x_vel *= -1.0;
        }
    } else {
        if intersects(ball, player_2) {
            ball.x_vel *= -1.0;
        }
    }

    // Handle inputs
    if button_up_held(0) == 1 {
        player_1.y -= Paddle::SPEED;
    }

    if button_down_held(0) == 1 {
        player_1.y += Paddle::SPEED;
    }

    if num_players() == 2 {
        // Handle Player Input
        if button_up_held(1) == 1 {
            player_2.y -= Paddle::SPEED;
        }

        if button_down_held(1) == 1 {
            player_2.y += Paddle::SPEED;
        }
    } else {
        // Be an AI
        if player_2.y > ball.y {
            player_2.y -= Paddle::SPEED
        } else if (player_2.y + Paddle::HEIGHT as f32) < ball.y {
            player_2.y += Paddle::SPEED
        }
    }

    player_1.y = player_1
        .y
        .clamp(0.0, (SCREEN_HEIGHT - Paddle::HEIGHT) as f32);
    player_2.y = player_2
        .y
        .clamp(0.0, (SCREEN_HEIGHT - Paddle::HEIGHT) as f32);
}

fn intersects(ball: &Ball, paddle: &Paddle) -> bool {
    let closest_x = f32::clamp(ball.x, paddle.x as f32, paddle.x + Paddle::WIDTH as f32);
    let closest_y = f32::clamp(ball.y, paddle.y as f32, paddle.y + Paddle::HEIGHT as f32);

    let distance_x = ball.x - closest_x;
    let distance_y = ball.y - closest_y;

    let distance_squared = (distance_x * distance_x) + (distance_y * distance_y);
    distance_squared < (Ball::RADIUS * Ball::RADIUS) as f32
}

#[no_mangle]
pub unsafe extern "C" fn draw() {
    let ball = &GAME_STATE.ball;
    let player_1 = &GAME_STATE.paddles[0];
    let player_2 = &GAME_STATE.paddles[1];

    clear_screen(0);

    circle(
        GAME_STATE.ball_color,
        ball.x as i32,
        ball.y as i32,
        Ball::RADIUS,
    );

    rect_filled(
        GAME_STATE.paddle_color[0],
        player_1.x as i32,
        player_1.y as i32,
        Paddle::WIDTH,
        Paddle::HEIGHT,
    );
    rect_filled(
        GAME_STATE.paddle_color[1],
        player_2.x as i32,
        player_2.y as i32,
        Paddle::WIDTH,
        Paddle::HEIGHT,
    );
}
