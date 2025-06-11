use astrolabe::DateTime;
use eframe::egui::{self, RichText, Vec2};

fn main() {
    let _ = eframe::run_native(
        "Clock",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_min_inner_size((550.0, 400.0)),
            ..Default::default()
        },
        Box::new(|_| Ok(Box::<Clock>::default())),
    );
}

struct Clock {
    current_tab: i8,
    stopwatch: bool,
    stopwatch_start: DateTime,
    stopwatch_stop: DateTime,
}

impl Default for Clock {
    fn default() -> Self {
        Self {
            current_tab: 0,
            stopwatch: false,
            stopwatch_start: DateTime::now_local(),
            stopwatch_stop: DateTime::now_local(),
        }
    }
}

fn clock_tab(ui: &mut egui::Ui, now: DateTime) {
    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.add_space(64.0);
        ui.label(RichText::new(now.format("HH:mm:ss")).size(128.0));
    });
}

fn stopwatch_tab(ui: &mut egui::Ui, clock: &mut Clock, now: DateTime) {
    let mut time: String = "00:00:00".to_string();
    if clock.stopwatch_stop != clock.stopwatch_start {
        let mut diff = clock.stopwatch_start.duration_between(&now);
        if clock.stopwatch == false {
            diff = clock
                .stopwatch_start
                .duration_between(&clock.stopwatch_stop);
        }

        let seconds = diff.as_secs() % 60;
        let minutes = (diff.as_secs() / 60) % 60;
        let hours = (diff.as_secs() / 60) / 60;

        time = format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds);
    }

    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.add_space(64.0);
        ui.label(RichText::new(time).size(128.0));
    });
}

fn add_tab(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    clock: &mut Clock,
    tab: i8,
    name: &str,
    x: f32,
    y: f32,
) {
    ui.add_space(20.);

    let mut enabled = true;
    if clock.current_tab == tab {
        enabled = false;
    }

    let button = ui.add_enabled(
        enabled,
        egui::Button::new(name).min_size(egui::Vec2 { x: x, y: y }),
    );

    if button.hovered() {
        ctx.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
    }

    if button.clicked() {
        clock.current_tab = tab;
    }

    ui.add_space(20.);
}

impl eframe::App for Clock {
    fn update(&mut self, ctx: &egui::Context, _eframe: &mut eframe::Frame) {
        let now = DateTime::now_local();

        ctx.request_repaint_after(std::time::Duration::from_millis(500));

        if self.current_tab == 1 {
            egui::TopBottomPanel::top("stopwatch_control")
                .show_separator_line(false)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(16.0);

                        let mut button_name = "Start";
                        if self.stopwatch == true {
                            button_name = "Stop";
                        }

                        let button = ui
                            .add(egui::Button::new(button_name).min_size(Vec2 { x: 64., y: 32. }));

                        if button.hovered() {
                            ctx.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
                        }

                        if button.clicked() {
                            if self.stopwatch == true {
                                self.stopwatch = false;
                                self.stopwatch_stop = DateTime::now_local();
                            } else {
                                self.stopwatch = true;
                                self.stopwatch_start = DateTime::now_local();
                            }
                        }
                    })
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| match self.current_tab {
            0 => clock_tab(ui, now),
            1 => stopwatch_tab(ui, self, now),
            _ => clock_tab(ui, now),
        });

        egui::TopBottomPanel::bottom("navbar")
            .show_separator_line(false)
            .frame(egui::Frame::default().inner_margin(16.0))
            .show(ctx, |ui| {
                ui.columns(2, |columns| {
                    columns[0]
                        .with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            add_tab(ui, ctx, self, 0, "Clock", 64., 32.)
                        });
                    columns[1]
                        .with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            add_tab(ui, ctx, self, 1, "Stopwatch", 96., 32.)
                        });
                });
            });
    }
}
