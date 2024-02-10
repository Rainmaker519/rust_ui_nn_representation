
use nannou::prelude::*;

struct Node {

}




// the design of nannou is roughly based on the model-view-controller pattern (MVC)
// this splits responsibility into:
//  a) a model which describes the internal state
//  b) a view which describes how to present the model
//  c) a controller which describes how to update the model on certain events
fn main() {
    nannou::app(model)  // start building the app and specify the model()
        .event(event) //specify we want to handle app events with event()
        .simple_window(view) //request a simple window to which we'll draw with view()
        .run(); //run it!
}
/*
fn main() {
    nannou::app(model)  // start building the app and specify the model()
        .update(update) //specify we want only deal with update events using the update function
        .simple_window(view) //request a simple window to which we'll draw with view()
        .run(); //run it!
 */

// the Model is where the state of the application is defined
struct Model {}

// app works as a high level wrapper for messy i/o that's hard to deal with in rust
// each nannou application has only one app instance
fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, _model: &Model, frame: Frame){
    let draw = app.draw();
    let window = app.main_window();
    let win = window.rect();

    draw.background().rgb(0.11, 0.12, 0.13);

    // 100-step and 10-step grids.
    draw_grid(&draw, &win, 100.0, 1.0);
    draw_grid(&draw, &win, 25.0, 0.5);

    let crosshair_color = gray(0.5);
    let ends = [
        win.mid_top(),
        win.mid_right(),
        win.mid_bottom(),
        win.mid_left(),
    ];
    for &end in &ends {
        draw.arrow()
            .start_cap_round()
            .head_length(16.0)
            .head_width(8.0)
            .color(crosshair_color)
            .end(end);
    }

    // Crosshair text.
    let top = format!("{:.1}", win.top());
    let bottom = format!("{:.1}", win.bottom());
    let left = format!("{:.1}", win.left());
    let right = format!("{:.1}", win.right());
    let x_off = 30.0;
    let y_off = 20.0;
    draw.text("0.0")
        .x_y(15.0, 15.0)
        .color(crosshair_color)
        .font_size(14);
    draw.text(&top)
        .h(win.h())
        .font_size(14)
        .align_text_top()
        .color(crosshair_color)
        .x(x_off);
    draw.text(&bottom)
        .h(win.h())
        .font_size(14)
        .align_text_bottom()
        .color(crosshair_color)
        .x(x_off);
    draw.text(&left)
        .w(win.w())
        .font_size(14)
        .left_justify()
        .color(crosshair_color)
        .y(y_off);
    draw.text(&right)
        .w(win.w())
        .font_size(14)
        .right_justify()
        .color(crosshair_color)
        .y(y_off);

    if let Some(monitor) = window.current_monitor() {
        let w_scale_factor = window.scale_factor();
        let m_scale_factor = monitor.scale_factor();
        let mon_phys = monitor.size();
        let mon = mon_phys.to_logical(w_scale_factor as f64);
        let mon_w: f32 = mon.width;
        let mon_h: f32 = mon.height;
        let text = format!(
            "
        Window size: [{:.0}, {:.0}]
        Window ratio: {:.2}
        Window scale factor: {:.2}
        Monitor size: [{:.0}, {:.0}]
        Monitor ratio: {:.2}
        Monitor scale factor: {:.2}
        ",
            win.w(),
            win.h(),
            win.w() / win.h(),
            w_scale_factor,
            mon_w,
            mon_h,
            mon_w / mon_h,
            m_scale_factor
        );

        let pad = 6.0;
        draw.text(&text)
            .h(win.pad(pad).h())
            .w(win.pad(pad).w())
            .line_spacing(pad)
            .font_size(14)
            .align_text_bottom()
            .color(crosshair_color)
            .left_justify();

        // Ellipse at mouse.
        draw.ellipse().wh([5.0; 2].into()).xy(app.mouse.position());

        // Mouse position text.
        let mouse = app.mouse.position();
        let pos = format!("[{:.1}, {:.1}]", mouse.x, mouse.y);
        draw.text(&pos)
            .xy(mouse + vec2(0.0, 20.0))
            .font_size(14)
            .color(WHITE);
    }

    draw_my_stuff(&draw);

    draw.to_frame(app, &frame).unwrap();
}

fn draw_my_stuff(draw: &Draw) {
    draw.rect()
        .x_y(0.0,0.0)
        .w_h(100.0,100.0)
        .color(PLUM);
}

// the event function is some code that runs every time some kind of app event occurs
fn event(_app: &App, _model: &mut Model, _event: Event) {

}

fn draw_grid(draw: &Draw, win: &Rect, step: f32, weight: f32) {
    let step_by = || (0..).map(|i| i as f32 * step);
    let r_iter = step_by().take_while(|&f| f < win.right());
    let l_iter = step_by().map(|f| -f).take_while(|&f| f > win.left());
    let x_iter = r_iter.chain(l_iter);
    for x in x_iter {
        draw.line()
            .weight(weight)
            .points(pt2(x, win.bottom()), pt2(x, win.top()));
    }
    let t_iter = step_by().take_while(|&f| f < win.top());
    let b_iter = step_by().map(|f| -f).take_while(|&f| f > win.bottom());
    let y_iter = t_iter.chain(b_iter);
    for y in y_iter {
        draw.line()
            .weight(weight)
            .points(pt2(win.left(), y), pt2(win.right(), y));
    }
}