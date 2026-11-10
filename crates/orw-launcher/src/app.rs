use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use eframe::egui;
use orw_core::{
    catalog_matches_benchmarks, graphics_profile_label, load_user_config, save_user_config,
    scan_routes, CameraMode, GraphicsProfile, InstallError, RailWorksInstall, RouteCatalog,
    RouteScanOptions, UpdateChannel, UserConfig,
};
use orw_engine::{launch_session, DriveMode, LaunchSession, PerformanceSlo, ENGINE_NAME};
use orw_vfs::RailWorksVfs;

use crate::preview::{show_gamecam_preview, PreviewShared};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Page {
    Home,
    Routes,
    Drive,
    SettingsGraphics,
    SettingsCamera,
    SettingsDisplay,
    SettingsGeneral,
    About,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SettingsSection {
    Graphics,
    Camera,
    Display,
    General,
}

pub struct LauncherApp {
    page: Page,
    config: UserConfig,
    preview_shared: PreviewShared,
    preview_aspect: Arc<Mutex<f32>>,
    wgpu_preview: bool,
    install: Option<RailWorksInstall>,
    catalog: Option<RouteCatalog>,
    install_error: Option<String>,
    selected_route: Option<String>,
    route_filter: String,
    drive_mode: DriveMode,
    ap_count: Option<usize>,
    status: String,
    last_save_hint: String,
    settings_dirty: bool,
}

impl LauncherApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let config = load_user_config();
        let preview_shared = PreviewShared {
            config: Arc::new(Mutex::new(config.clone())),
        };
        let wgpu_preview = cc.wgpu_render_state.is_some();
        let mut app = Self {
            page: Page::Home,
            config,
            preview_shared,
            preview_aspect: Arc::new(Mutex::new(16.0 / 9.0)),
            wgpu_preview,
            install: None,
            catalog: None,
            install_error: None,
            selected_route: None,
            route_filter: String::new(),
            drive_mode: DriveMode::FreeRoam,
            ap_count: None,
            status: String::new(),
            last_save_hint: String::new(),
            settings_dirty: false,
        };
        app.sync_preview_config();
        app.refresh_installation();
        app
    }

    fn sync_preview_config(&mut self) {
        if let Ok(mut locked) = self.preview_shared.config.lock() {
            *locked = self.config.clone();
        }
    }

    fn apply_config_live(&mut self, ctx: &egui::Context) {
        self.sync_preview_config();
        self.settings_dirty = true;
        ctx.request_repaint();
    }

    fn persist_config(&mut self) {
        match save_user_config(&self.config) {
            Ok(path) => {
                self.last_save_hint = format!("Gespeichert · {}", path.display());
                self.settings_dirty = false;
                self.refresh_installation();
            }
            Err(e) => self.status = format!("Speichern fehlgeschlagen: {e}"),
        }
    }

    fn refresh_installation(&mut self) {
        self.install_error = None;
        match RailWorksInstall::discover(self.config.railworks_path.as_deref()) {
            Ok(install) => {
                if let Ok(catalog) = scan_routes(&install, &RouteScanOptions::default()) {
                    self.catalog = Some(catalog);
                }
                if let Ok(vfs) = RailWorksVfs::open(&install) {
                    self.ap_count = vfs.list_ap_archives().ok().map(|v| v.len());
                }
                self.install = Some(install);
            }
            Err(InstallError::PathNotConfigured) => {
                self.install = None;
                self.catalog = None;
                self.install_error = Some(
                    "Kein RailWorks-Pfad — unter Allgemein den Steam-Ordner setzen.".into(),
                );
            }
            Err(e) => {
                self.install = None;
                self.catalog = None;
                self.install_error = Some(e.to_string());
            }
        }
    }

    fn try_start_drive(&mut self) {
        let Some(route_id) = self.selected_route.clone() else {
            self.status = "Bitte zuerst eine Strecke wählen.".into();
            self.page = Page::Routes;
            return;
        };
        let _ = save_user_config(&self.config);
        let mut session = LaunchSession::free_roam(route_id, self.config.graphics_profile);
        session.performance = PerformanceSlo::from(&self.config);
        session.mode = self.drive_mode;
        match launch_session(&session) {
            Ok(()) => self.status = "Lauf gestartet.".into(),
            Err(e) => self.status = format!("Engine noch nicht bereit ({e}). Vorschau zeigt bereits deine Grafik-Einstellungen."),
        }
    }

    fn show_preview_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("gamecam")
            .resizable(true)
            .default_width(420.0)
            .min_width(300.0)
            .show(ctx, |ui| {
                ui.heading("GameCam");
                ui.label("Live-Vorschau — Änderungen sofort sichtbar (openOMSI-Stil).");
                ui.separator();
                let resp = show_gamecam_preview(
                    ui,
                    &self.preview_shared,
                    &self.preview_aspect,
                    self.wgpu_preview,
                );
                if resp.dragged() {
                    if let Ok(locked) = self.preview_shared.config.lock() {
                        self.config.camera = locked.camera.clone();
                    }
                    let _ = save_user_config(&self.config);
                }
                let cfg = &self.config;
                ui.separator();
                ui.small(format!(
                    "{} · {}×{} · {} FPS · FOV {:.0}° · {:?}",
                    graphics_profile_label(cfg.graphics_profile),
                    cfg.display_width,
                    cfg.display_height,
                    cfg.target_fps,
                    cfg.camera.fov_vertical_deg,
                    cfg.camera.mode
                ));
                ui.small(format!(
                    "Tageszeit {:.1} h · Sichtweite {:.0} m · MSAA {}× · Scale {:.0}%",
                    cfg.graphics.time_of_day_hours,
                    cfg.graphics.view_distance_m,
                    cfg.graphics.msaa_samples,
                    cfg.graphics.render_scale * 100.0
                ));
                if self.settings_dirty {
                    ui.horizontal(|ui| {
                        if ui.button("Jetzt speichern").clicked() {
                            self.persist_config();
                        }
                    });
                } else if !self.last_save_hint.is_empty() {
                    ui.small(&self.last_save_hint);
                }
            });
    }

    fn settings_nav(&mut self, ui: &mut egui::Ui) {
        ui.label("Einstellungen");
        ui.separator();
        for (section, label) in [
            (SettingsSection::Graphics, "Grafik"),
            (SettingsSection::Camera, "Kamera"),
            (SettingsSection::Display, "Anzeige"),
            (SettingsSection::General, "Allgemein"),
        ] {
            let current = matches!(
                (section, self.page),
                (SettingsSection::Graphics, Page::SettingsGraphics)
                    | (SettingsSection::Camera, Page::SettingsCamera)
                    | (SettingsSection::Display, Page::SettingsDisplay)
                    | (SettingsSection::General, Page::SettingsGeneral)
            );
            if ui.selectable_label(current, label).clicked() {
                self.page = match section {
                    SettingsSection::Graphics => Page::SettingsGraphics,
                    SettingsSection::Camera => Page::SettingsCamera,
                    SettingsSection::Display => Page::SettingsDisplay,
                    SettingsSection::General => Page::SettingsGeneral,
                };
            }
        }
    }
}

impl eframe::App for LauncherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let opacity = self.config.display.ui_opacity.clamp(0.2, 1.0);
        let mut style = (*ctx.style()).clone();
        style.visuals.panel_fill = style
            .visuals
            .panel_fill
            .gamma_multiply(opacity);
        ctx.set_style(style);

        let show_cam = matches!(
            self.page,
            Page::Home
                | Page::Drive
                | Page::SettingsGraphics
                | Page::SettingsCamera
                | Page::SettingsDisplay
        );
        if show_cam {
            self.show_preview_panel(ctx);
        }

        egui::SidePanel::left("nav")
            .resizable(false)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading(ENGINE_NAME);
                ui.separator();
                nav_button(ui, &mut self.page, Page::Home, "Start");
                nav_button(ui, &mut self.page, Page::Routes, "Strecken");
                nav_button(ui, &mut self.page, Page::Drive, "Fahren");
                ui.separator();
                self.settings_nav(ui);
                ui.separator();
                nav_button(ui, &mut self.page, Page::About, "Über");
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.page {
                Page::Home => self.page_home(ui, ctx),
                Page::Routes => self.page_routes(ui),
                Page::Drive => self.page_drive(ui, ctx),
                Page::SettingsGraphics => self.page_settings_graphics(ui, ctx),
                Page::SettingsCamera => self.page_settings_camera(ui, ctx),
                Page::SettingsDisplay => self.page_settings_display(ui, ctx),
                Page::SettingsGeneral => self.page_settings_general(ui, ctx),
                Page::About => self.page_about(ui),
            }
        });
    }
}

fn nav_button(ui: &mut egui::Ui, page: &mut Page, target: Page, label: &str) {
    if ui.selectable_label(*page == target, label).clicked() {
        *page = target;
    }
}

impl LauncherApp {
    fn page_home(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.heading("Train Simulator Classic");
        ui.label("OpenRailWorks — deine Strecken, neue Engine. Keine Inhalte im Installer.");
        ui.separator();
        if let Some(err) = &self.install_error {
            ui.colored_label(egui::Color32::from_rgb(255, 180, 80), err);
        } else if let Some(install) = &self.install {
            ui.label(format!("Installation: {}", install.root.display()));
            if let Some(catalog) = &self.catalog {
                ui.label(format!(
                    "Strecken: {} · .ap: {}",
                    catalog.routes.len(),
                    self.ap_count.unwrap_or(0)
                ));
                let hits = catalog_matches_benchmarks(catalog);
                if !hits.is_empty() {
                    ui.label(format!("Benchmark: {}", hits.join(", ")));
                }
            }
        }
        ui.horizontal(|ui| {
            if ui.button("Strecken").clicked() {
                self.page = Page::Routes;
            }
            if ui.button("Grafik einstellen").clicked() {
                self.page = Page::SettingsGraphics;
            }
        });
        let _ = ctx;
    }

    fn page_routes(&mut self, ui: &mut egui::Ui) {
        ui.heading("Strecken");
        ui.horizontal(|ui| {
            ui.label("Suche");
            ui.text_edit_singleline(&mut self.route_filter);
        });
        let Some(catalog) = self.catalog.clone() else {
            ui.label("Kein Katalog — Installation prüfen.");
            return;
        };
        let filter = self.route_filter.to_lowercase();
        egui::ScrollArea::vertical().show(ui, |ui| {
            for route in &catalog.routes {
                if !filter.is_empty() && !route.id.to_lowercase().contains(&filter) {
                    continue;
                }
                let is_sel = self.selected_route.as_deref() == Some(route.id.as_str());
                let fav = self.config.favorite_routes.contains(&route.id);
                ui.horizontal(|ui| {
                    let label = format!(
                        "{}{}  (Szenarien: {})",
                        if fav { "★ " } else { "" },
                        route.id,
                        route.scenario_count
                    );
                    if ui.selectable_label(is_sel, label).clicked() {
                        self.selected_route = Some(route.id.clone());
                    }
                    if ui.small_button(if fav { "☆" } else { "★" }).clicked() {
                        if fav {
                            self.config.favorite_routes.retain(|id| id != &route.id);
                        } else {
                            self.config.favorite_routes.push(route.id.clone());
                        }
                        let _ = save_user_config(&self.config);
                    }
                });
            }
        });
    }

    fn page_drive(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.heading("Fahren");
        let route = self
            .selected_route
            .as_deref()
            .unwrap_or("(Strecke auf „Strecken“ wählen)");
        ui.label(format!("Strecke: {route}"));
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.drive_mode, DriveMode::FreeRoam, "Free Roam");
            ui.radio_value(&mut self.drive_mode, DriveMode::Scenario, "Szenario (bald)");
        });
        ui.separator();
        if ui
            .add(egui::Button::new("Fahrt starten").min_size(egui::vec2(220.0, 44.0)))
            .clicked()
        {
            self.try_start_drive();
        }
        if !self.status.is_empty() {
            ui.label(&self.status);
        }
        let _ = ctx;
    }

    fn page_settings_graphics(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.heading("Grafik");
        let mut changed = false;
        egui::ComboBox::from_label("Grafikmodus")
            .selected_text(graphics_profile_label(self.config.graphics_profile))
            .show_ui(ui, |ui| {
                for (p, name) in [
                    (GraphicsProfile::Vanilla, graphics_profile_label(GraphicsProfile::Vanilla)),
                    (
                        GraphicsProfile::VanillaPlus,
                        graphics_profile_label(GraphicsProfile::VanillaPlus),
                    ),
                    (
                        GraphicsProfile::Enhanced,
                        graphics_profile_label(GraphicsProfile::Enhanced),
                    ),
                ] {
                    if ui
                        .selectable_value(&mut self.config.graphics_profile, p, name)
                        .changed()
                    {
                        changed = true;
                    }
                }
            });

        changed |= ui
            .add(
                egui::Slider::new(&mut self.config.graphics.time_of_day_hours, 0.0..=24.0)
                    .text("Tageszeit (h)")
                    .suffix(" h"),
            )
            .changed();
        changed |= ui
            .add(
                egui::Slider::new(&mut self.config.graphics.view_distance_m, 400.0..=8000.0)
                    .text("Sichtweite"),
            )
            .changed();
        changed |= ui
            .add(
                egui::Slider::new(&mut self.config.graphics.render_scale, 0.5..=1.0)
                    .text("Render-Scale"),
            )
            .changed();

        ui.horizontal(|ui| {
            ui.label("MSAA");
            for samples in [1u32, 2, 4, 8] {
                if ui
                    .selectable_label(self.config.graphics.msaa_samples == samples, format!("{samples}×"))
                    .clicked()
                {
                    self.config.graphics.msaa_samples = samples;
                    changed = true;
                }
            }
        });

        changed |= ui
            .checkbox(&mut self.config.graphics.shadows, "Sonnenschatten (Enhanced)")
            .changed();
        changed |= ui
            .checkbox(&mut self.config.graphics.vsync, "VSync")
            .changed();

        if changed {
            self.apply_config_live(ctx);
            self.persist_config();
        }
    }

    fn page_settings_camera(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.heading("Kamera");
        let mut changed = false;
        ui.horizontal(|ui| {
            for (mode, label) in [
                (CameraMode::Cab, "Führerstand"),
                (CameraMode::Chase, "Chase"),
                (CameraMode::Aerial, "Oben"),
            ] {
                if ui
                    .selectable_value(&mut self.config.camera.mode, mode, label)
                    .changed()
                {
                    changed = true;
                }
            }
        });
        changed |= ui
            .add(
                egui::Slider::new(&mut self.config.camera.fov_vertical_deg, 45.0..=110.0)
                    .text("Vertikales Sichtfeld"),
            )
            .changed();
        if self.config.camera.mode != CameraMode::Cab {
            changed |= ui
                .add(
                    egui::Slider::new(&mut self.config.camera.chase_distance_m, 5.0..=80.0)
                        .text("Abstand"),
                )
                .changed();
        }
        ui.label("In der Vorschau: mit der Maus drehen (Orbit).");
        if changed {
            self.apply_config_live(ctx);
            self.persist_config();
        }
    }

    fn page_settings_display(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.heading("Anzeige");
        let mut changed = false;
        ui.horizontal(|ui| {
            changed |= ui
                .add(egui::DragValue::new(&mut self.config.display_width).suffix(" px"))
                .changed();
            ui.label("×");
            changed |= ui
                .add(egui::DragValue::new(&mut self.config.display_height).suffix(" px"))
                .changed();
        });
        changed |= ui
            .add(egui::DragValue::new(&mut self.config.target_fps).range(30..=240).suffix(" FPS"))
            .changed();
        changed |= ui
            .checkbox(&mut self.config.display.fullscreen, "Vollbild beim Spielstart")
            .changed();
        changed |= ui
            .add(
                egui::Slider::new(&mut self.config.display.ui_opacity, 0.2..=1.0)
                    .text("UI-Deckkraft"),
            )
            .changed();
        if changed {
            self.apply_config_live(ctx);
            self.persist_config();
        }
    }

    fn page_settings_general(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.heading("Allgemein");
        let mut changed = false;
        ui.label("RailWorks-Ordner (nur Lesen):");
        let mut path_text = self
            .config
            .railworks_path
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_default();
        ui.horizontal(|ui| {
            changed |= ui.text_edit_singleline(&mut path_text).changed();
            if ui.button("Durchsuchen…").clicked() {
                if let Some(picked) = rfd::FileDialog::new().pick_folder() {
                    self.config.railworks_path = Some(picked);
                    path_text = self
                        .config
                        .railworks_path
                        .as_ref()
                        .unwrap()
                        .display()
                        .to_string();
                    changed = true;
                }
            }
        });
        if !path_text.is_empty() {
            self.config.railworks_path = Some(PathBuf::from(path_text));
        }

        egui::ComboBox::from_label("Update-Kanal")
            .selected_text(format!("{:?}", self.config.update_channel))
            .show_ui(ui, |ui| {
                changed |= ui
                    .selectable_value(&mut self.config.update_channel, UpdateChannel::Stable, "Stable")
                    .changed();
                changed |= ui
                    .selectable_value(&mut self.config.update_channel, UpdateChannel::Beta, "Beta")
                    .changed();
            });

        if changed {
            self.apply_config_live(ctx);
            self.persist_config();
        }
    }

    fn page_about(&mut self, ui: &mut egui::Ui) {
        ui.heading("Über");
        ui.label(format!("Version {}", env!("CARGO_PKG_VERSION")));
        ui.hyperlink_to("GitHub", "https://github.com/Gegels/openrailworks");
        ui.label("Launcher-Einstellungen gelten für Vorschau und spätere Spielsessions.");
    }
}
