use crate::achievements::*;
use crate::application::{Application, Fieled, Filter, FilterType};
use eframe::egui;

impl Application {
    pub fn achievements_ui(&mut self, ctx: &egui::Context) {
        let achievements = self
            .filtered_achievements()
            .into_iter()
            .map(|(_, achievement)| achievement)
            .collect::<Vec<Achievement>>();

        egui::CentralPanel::default().show(ctx, |ui| {
            self.menu_bar(ctx, ui);

            egui::ScrollArea::both()
                .stick_to_right(true)
                .show(ui, |ui| {
                egui::Grid::new("Achievements Grid")
                    .spacing(egui::vec2(
                        self.settings.font_size * 5.0,
                        self.settings.font_size * 2.0,
                    ))
                    .num_columns(11)
                    .with_row_color(move |row_index, style| {
                        if row_index > 0
                            && row_index < achievements.len() + 1
                            && achievements[row_index - 1].done
                        {
                            let green = egui::Color32::from_rgba_unmultiplied(0, 255, 0, 50);
                            Some(green)
                        } else if row_index > 0
                            && row_index < achievements.len() + 1
                            && achievements[row_index - 1].present_soon
                        {
                            let yellow = egui::Color32::from_rgba_unmultiplied(255, 255, 0, 25);
                            Some(yellow)
                        } else if row_index % 2 == 1 {
                            Some(style.visuals.faint_bg_color)
                        } else {
                            None
                        }
                    })
                    .show(ui, |ui| {
                        let id = ui.heading("ID").on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_ui(|ui| {
                            ui.label(format!("Click to sort {}", self.sorting.id.reverse()));
                        });
                        let title = ui.heading("Title").on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_ui(|ui| {
                            ui.label(format!("Click to sort {}", self.sorting.title.reverse()));
                        });
                        let deadline = ui.heading("Deadline").on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_ui(|ui| {
                            ui.label(format!("Click to sort {}", self.sorting.deadline.reverse()));
                        });
                        let done = ui.heading("Done").on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_ui(|ui| {
                            ui.label(format!("Click to sort {}", self.sorting.done.reverse()));
                        });
                        let present_soon = ui.heading("Present Soon").on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_ui(|ui| {
                            ui.label(format!("Click to sort {}", self.sorting.present_soon.reverse()));
                        });
                        let grade = ui.heading("Grade").on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_ui(|ui| {
                            ui.label(format!("Click to sort {}", self.sorting.grade.reverse()));
                        });
                        let presenting_type = ui.heading("Presenting Type").on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_ui(|ui| {
                            ui.label(format!("Click to sort {}", self.sorting.presenting_type.reverse()));
                        });
                        let programming_language = ui.heading("Programming Language").on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_ui(|ui| {
                            ui.label(format!("Click to sort {}", self.sorting.programming_language.reverse()));
                        });
                        let sprint = ui.heading("Sprint").on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_ui(|ui| {
                            ui.label(format!("Click to sort {}", self.sorting.sprint.reverse()));
                        });
                        let comment = ui.heading("Comment").on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_ui(|ui| {
                            ui.label(format!("Click to sort {}", self.sorting.comment.reverse()));
                        });
                        if id.clicked_by(egui::PointerButton::Primary) {
                            self.sort_achievements(Fieled::ID);
                        }
                        if title.clicked_by(egui::PointerButton::Primary) {
                            self.sort_achievements(Fieled::Title);
                        }
                        if deadline.clicked_by(egui::PointerButton::Primary) {
                            self.sort_achievements(Fieled::Deadline);
                        }
                        if done.clicked_by(egui::PointerButton::Primary) {
                            self.sort_achievements(Fieled::Done);
                        }
                        if present_soon.clicked_by(egui::PointerButton::Primary) {
                            self.sort_achievements(Fieled::PresentSoon);
                        }
                        if grade.clicked_by(egui::PointerButton::Primary) {
                            self.sort_achievements(Fieled::Grade);
                        }
                        if presenting_type.clicked_by(egui::PointerButton::Primary) {
                            self.sort_achievements(Fieled::PresentingType);
                        }
                        if programming_language.clicked_by(egui::PointerButton::Primary) {
                            self.sort_achievements(Fieled::ProgrammingLanguage);
                        }
                        if sprint.clicked_by(egui::PointerButton::Primary) {
                            self.sort_achievements(Fieled::Sprint);
                        }
                        if comment.clicked_by(egui::PointerButton::Primary) {
                            self.sort_achievements(Fieled::Comment);
                        }
                        ui.end_row();

                        for (i, achievement) in self.filtered_achievements() {
                            let id = ui.add(egui::Hyperlink::from_label_and_url(
                                achievement.id.clone(),
                                format!("https://uppsala.instructure.com/courses/97453/pages/achievements#{}", achievement.id[1..].to_string()),
                            ))
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .on_hover_text("Right click to filter out\nShift Right click to only show this");

                            let title = ui.add(egui::Hyperlink::from_label_and_url(
                                egui::RichText::new(format!("{}", achievement.title))
                                    .color(
                                        match self.settings.dark_mode {
                                            true => egui::Color32::LIGHT_GRAY,
                                            false => egui::Color32::DARK_GRAY,
                                        }),
                                format!("https://uppsala.instructure.com/courses/97453/pages/achievements#{}", achievement.id[1..].to_string()),
                            ))
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .on_hover_text("Right click to filter out\nShift Right click to only show this");

                            let deadline = match achievement.deadline {
                                Some(deadline) => {
                                    ui.label(deadline.format("%a %b %d, %Y").to_string())
                                }
                                None => {
                                    ui.label("")
                                }
                            }
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .on_hover_text("Right click to filter out\nShift Right click to only show this");

                            let done = ui.centered_and_justified(|ui| {
                                ui.checkbox(&mut self.achievements[i].done, "")
                                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                                    .on_hover_text("Right click to filter out\nShift Right click to only show this")
                            });
                            let present_soon = ui.centered_and_justified(|ui| {
                                ui.checkbox(&mut self.achievements[i].present_soon, "")
                                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                                    .on_hover_text("Right click to filter out\nShift Right click to only show this")
                            });
                            let grade = ui.label(achievement.grade.to_string())
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .on_hover_text("Right click to filter out\nShift Right click to only show this");

                            let presenting_type = match &achievement.presenting_type {
                                AchievementPresention::Single(presenting_type) => {
                                    ui.label(format!("{:?}", presenting_type))
                                },
                                AchievementPresention::Either { first, second } => {
                                    ui.label(format!("{:?} OR {:?}", first, second))
                                },
                            }
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .on_hover_text("Right click to filter out\nShift Right click to only show this");

                            let programming_language = match &achievement.programming_language {
                                AchievementLanguage::Single(programming_language) => {
                                    if programming_language == &ProgrammingLanguage::NoLanguage {
                                        ui.label("No Specific Language")
                                    } else {
                                        ui.label(format!("{:?}", programming_language))
                                    }
                                },
                                AchievementLanguage::Both { first, second } => {
                                    ui.label(format!("{:?} AND {:?}", first, second))
                                },
                                AchievementLanguage::Either { first, second } => {
                                    ui.label(format!("{:?} OR {:?}", first, second))
                                },
                            }
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .on_hover_text("Right click to filter out\nShift Right click to only show this");

                            let sprint = ui.label(format!("{:?}", achievement.sprint))
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .on_hover_text("Right click to filter out\nShift Right click to only show this");

                            let comment = match &achievement.comment {
                                Some(comment) => {
                                    ui.label(comment)
                                }
                                None => {
                                    ui.label("")
                                }
                            }
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .on_hover_text("Right click to filter out\nShift Right click to only show this");
                            ui.allocate_space(egui::vec2(10.0, 0.0));

                            if id.clicked_by(egui::PointerButton::Secondary) {
                                if ctx.input(|i| i.modifiers.shift) {
                                    self.filters.id = Filter::new();
                                    self.filters.id.typ = FilterType::Include;
                                } else {
                                    self.filters.id.typ = FilterType::Remove;
                                }
                                self.filters.id.push(achievement.id.clone());
                            }
                            if title.clicked_by(egui::PointerButton::Secondary) {
                                if ctx.input(|i| i.modifiers.shift) {
                                    self.filters.title = Filter::new();
                                    self.filters.title.typ = FilterType::Include;
                                } else {
                                    self.filters.title.typ = FilterType::Remove;
                                }
                                self.filters.title.push(achievement.title.clone());
                            }
                            if deadline.clicked_by(egui::PointerButton::Secondary) {
                                if ctx.input(|i| i.modifiers.shift) {
                                    self.filters.deadline = Filter::new();
                                    self.filters.deadline.typ = FilterType::Include;
                                } else {
                                    self.filters.deadline.typ = FilterType::Remove;
                                }
                                self.filters.deadline.push(achievement.deadline.clone());
                            }
                            if done.inner.clicked_by(egui::PointerButton::Secondary) {
                                if  ctx.input(|i| i.modifiers.shift) {
                                    self.filters.done = Filter::new();
                                    self.filters.done.typ = FilterType::Include;
                                } else {
                                    self.filters.done.typ = FilterType::Remove;
                                }
                                self.filters.done.push(achievement.done);
                            }
                            if present_soon.inner.clicked_by(egui::PointerButton::Secondary) {
                                if ctx.input(|i| i.modifiers.shift) {
                                    self.filters.present_soon = Filter::new();
                                    self.filters.present_soon.typ = FilterType::Include;
                                } else {
                                    self.filters.present_soon.typ = FilterType::Remove;
                                }
                                self.filters.present_soon.push(achievement.present_soon);
                            }
                            if grade.clicked_by(egui::PointerButton::Secondary) {
                                if ctx.input(|i| i.modifiers.shift) {
                                    self.filters.grade = Filter::new();
                                    self.filters.grade.typ = FilterType::Include;
                                } else {
                                    self.filters.grade.typ = FilterType::Remove;
                                }
                                self.filters.grade.push(achievement.grade);
                            }
                            if presenting_type.clicked_by(egui::PointerButton::Secondary) {
                                if ctx.input(|i| i.modifiers.shift) {
                                    self.filters.presenting_type = Filter::new();
                                    self.filters.presenting_type.typ = FilterType::Include;
                                } else {
                                    self.filters.presenting_type.typ = FilterType::Remove;
                                }
                                self.filters.presenting_type.push(achievement.presenting_type.clone());
                            }
                            if programming_language.clicked_by(egui::PointerButton::Secondary) {
                                if ctx.input(|i| i.modifiers.shift) {
                                    self.filters.programming_language = Filter::new();
                                    self.filters.programming_language.typ = FilterType::Include;
                                } else {
                                    self.filters.programming_language.typ = FilterType::Remove;
                                }
                                self.filters.programming_language.push(achievement.programming_language.clone());
                            }
                            if sprint.clicked_by(egui::PointerButton::Secondary) {
                                if ctx.input(|i| i.modifiers.shift) {
                                    self.filters.sprint = Filter::new();
                                    self.filters.sprint.typ = FilterType::Include;
                                } else {
                                    self.filters.sprint.typ = FilterType::Remove;
                                }
                                self.filters.sprint.push(achievement.sprint.clone());
                            }
                            if comment.clicked_by(egui::PointerButton::Secondary) {
                                if ctx.input(|i| i.modifiers.shift) {
                                    self.filters.comment = Filter::new();
                                    self.filters.comment.typ = FilterType::Include;
                                } else {
                                    self.filters.comment.typ = FilterType::Remove;
                                }
                                self.filters.comment.push(achievement.comment.clone());
                            }
                            ui.end_row();
                        }
                    });
                });
        });
    }
}
