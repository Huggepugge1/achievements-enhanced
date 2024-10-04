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

        let settings = self.settings.clone();

        egui::CentralPanel::default().show(ctx, |ui| {
            self.menu_bar(ctx, ui);

            egui::ScrollArea::both()
                .stick_to_right(true)
                .show(ui, |ui| {
                egui::Grid::new("Achievements Grid")
                    .spacing(egui::vec2(
                        self.settings.font_size * 2.0,
                        self.settings.font_size * 2.0,
                    ))
                    .num_columns(12)
                    .with_row_color(move |row_index, style| {
                        if row_index > 0
                            && row_index < achievements.len() + 1
                            && achievements[row_index - 1].done
                        {
                            let green = match settings.dark_mode {
                                true => egui::Color32::from_rgba_unmultiplied(0, 255, 0, 25),
                                false => egui::Color32::from_rgba_unmultiplied(0, 255, 0, 100),
                            };
                            Some(green)
                        } else if row_index > 0
                            && row_index < achievements.len() + 1
                            && achievements[row_index - 1].present_soon
                        {
                            let yellow = match settings.dark_mode {
                                true => egui::Color32::from_rgba_unmultiplied(255, 255, 0, 25),
                                false => egui::Color32::from_rgba_unmultiplied(255, 255, 0, 100),
                            };
                            Some(yellow)
                        } else if row_index % 2 == 1 {
                            Some(style.visuals.faint_bg_color)
                        } else {
                            None
                        }
                    })
                    .show(ui, |ui| {
                        let id = self.heading(ui, self.language.id.clone()).on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_ui(|ui| {
                            ui.label(format!("{} {}", self.language.click_to_sort, self.sorting.id.reverse()));
                        });
                        let title = self.heading(ui, self.language.title.clone()).on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_ui(|ui| {
                            ui.label(format!("{} {}", self.language.click_to_sort, self.sorting.title.reverse()));
                        });
                        let deadline = self.heading(ui, self.language.deadline.clone()).on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_ui(|ui| {
                            ui.label(format!("{} {}", self.language.click_to_sort, self.sorting.deadline.reverse()));
                        });
                        let done = self.heading(ui, self.language.done.clone()).on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_ui(|ui| {
                            ui.label(format!("{} {}", self.language.click_to_sort, self.sorting.done.reverse()));
                        });
                        let present_soon = self.heading(ui, self.language.present_soon.clone()).on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_ui(|ui| {
                            ui.label(format!("{} {}", self.language.click_to_sort, self.sorting.present_soon.reverse()));
                        });
                        let grade = self.heading(ui, self.language.grade.clone()).on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_ui(|ui| {
                            ui.label(format!("{} {}", self.language.click_to_sort, self.sorting.grade.reverse()));
                        });
                        let presenting_type = self.heading(ui, self.language.presenting_type.clone()).on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_ui(|ui| {
                            ui.label(format!("{} {}", self.language.click_to_sort, self.sorting.presenting_type.reverse()));
                        });
                        let programming_language = self.heading(ui, self.language.programming_language.clone()).on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_ui(|ui| {
                            ui.label(format!("{} {}", self.language.click_to_sort, self.sorting.programming_language.reverse()));
                        });
                        let sprint = self.heading(ui, self.language.sprint.clone()).on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_ui(|ui| {
                            ui.label(format!("{} {}", self.language.click_to_sort, self.sorting.sprint.reverse()));
                        });
                        let comment = self.heading(ui, self.language.comment.clone()).on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_ui(|ui| {
                            ui.label(format!("{} {}", self.language.click_to_sort, self.sorting.comment.reverse()));
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
                                .on_hover_text(self.language.right_click_to_filter.clone());

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
                                .on_hover_text(self.language.right_click_to_filter.clone());

                            let deadline = match achievement.deadline {
                                Some(deadline) => {
                                    ui.label(deadline.format("%a %b %d, %Y").to_string())
                                }
                                None => {
                                    ui.label("")
                                }
                            }
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .on_hover_text(self.language.right_click_to_filter.clone());

                            let done = ui.centered_and_justified(|ui| {
                                ui.checkbox(&mut self.achievements[i].done, "")
                                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                                    .on_hover_text(self.language.right_click_to_filter.clone())
                            });
                            let present_soon = ui.centered_and_justified(|ui| {
                                ui.checkbox(&mut self.achievements[i].present_soon, "")
                                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                                    .on_hover_text(self.language.right_click_to_filter.clone())
                            });
                            let grade = ui.label(achievement.grade.to_string())
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .on_hover_text(self.language.right_click_to_filter.clone());

                            let presenting_type = ui.label(achievement.presenting_type.to_string())
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .on_hover_text(self.language.right_click_to_filter.clone());

                            let programming_language = match &achievement.programming_language {
                                AchievementLanguage::Single(programming_language) => {
                                    if programming_language == &ProgrammingLanguage::NoLanguage {
                                        ui.label(self.language.no_specific_language.clone())
                                    } else {
                                        ui.label(format!("{:?}", programming_language))
                                    }
                                },
                                AchievementLanguage::Both { first, second } => {
                                    ui.label(format!("{:?} & {:?}", first, second))
                                },
                                AchievementLanguage::Either { first, second } => {
                                    ui.label(format!("{:?} / {:?}", first, second))
                                },
                            }
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .on_hover_text(self.language.right_click_to_filter.clone());

                            let sprint = ui.label(format!("{:?}", achievement.sprint))
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .on_hover_text(self.language.right_click_to_filter.clone());

                            let comment = match &achievement.comment {
                                Some(comment) => {
                                    ui.label(comment)
                                }
                                None => {
                                    ui.label("")
                                }
                            }
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .on_hover_text(self.language.right_click_to_filter.clone());
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
