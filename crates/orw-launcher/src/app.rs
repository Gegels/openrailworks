use std::path::PathBuf;

use eframe::egui;
use orw_core::{
    catalog_matches_benchmarks, load_user_config, save_user_config, scan_routes,
    GraphicsProfile, InstallError, RailWorksInstall, RouteCatalog, RouteScanOptions,
    UpdateChannel, UserConfig,
};
use orw_engine::{launch_session, DriveMode, LaunchSession, PerformanceSlo, ENGINE_NAME};
use orw_vfs::RailWorksVfs;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Page {
    Home,
    Routes,
    Drive,
    Settings,
    About,
}

pub struct LauncherApp {
    page: Page,
    config: UserConfig,
    install: Option<RailWorksInstall>,
    catalog: Option<RouteCatalog>,
    install_error: Option<String>,
    selected_route: Option<String>,
    route_filter: String,
    drive_mode: DriveMode,
    ap_count: Option<usize>,
    status: String,
    config_saved_hint: Option<String>,
}

impl LauncherApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let config = load_user_config();
        let mut app = Self {
            page: Page::Home,
            config,
            install: None,
            catalog: None,
            install_error: None,
            selected_route: None,
            route_filter: String::new(),
            drive_mode: DriveMode::FreeRoam,
            ap_count: None,
            status: String::new(),
            config_saved_hint: None,
        };
        app.refresh_installation();
        app
    }

    fn refresh_installation(&mut self) {
        self.install_error = None;
        match RailWorksInstall::discover(self.config.railworks_path.as_deref()) {
            Ok(install) => {
                if let Ok(catalog) =
                    scan_routes(&install, &RouteScanOptions::default())
                {
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
                    "Kein RailWorks-Pfad gesetzt — unter Einstellungen den Steam-Ordner wählen."
                        .into(),
                );
            }
            Err(e) => {
                self.install = None;
                self.catalog = None;
                self.install_error = Some(e.to_string());
            }
        }
    }

    fn save_config(&mut self) {
        match save_user_config(&self.config) {
            Ok(path) => {
                self.config_saved_hint = Some(format!("Gespeichert: {}", path.display()));
                self.refresh_installation();
            }
            Err(e) => self.status = format!("Config speichern fehlgeschlagen: {e}"),
        }
    }

    fn try_start_drive(&mut self) {
        let Some(route_id) = self.selected_route.clone() else {
            self.status = "Bitte zuerst eine Strecke auf der Strecken-Seite wählen.".into();
            self.page = Page::Routes;
            return;
        };
        let mut session = LaunchSession::free_roam(route_id, self.config.graphics_profile);
        session.performance = PerformanceSlo::from(&self.config);
        session.mode = self.drive_mode;
        match launch_session(&session) {
            Ok(()) => self.status = "Lauf gestartet.".into(),
            Err(e) => {
                self.status = format!(
                    "Engine noch nicht bereit ({e}). Phase 1: VFS und Route-Anzeige kommen als Nächstes."
                );
            }
        }
    }
}

impl eframe::App for LauncherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("nav")
            .resizable(false)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading(ENGINE_NAME);
                ui.label("Launcher");
                ui.separator();
                if ui.selectable_label(self.page == Page::Home, "Start").clicked() {
                    self.page = Page::Home;
                }
                if ui.selectable_label(self.page == Page::Routes, "Strecken").clicked() {
                    self.page = Page::Routes;
                }
                if ui.selectable_label(self.page == Page::Drive, "Fahren").clicked() {
                    self.page = Page::Drive;
                }
                if ui.selectable_label(self.page == Page::Settings, "Einstellungen").clicked() {
                    self.page = Page::Settings;
                }
                if ui.selectable_label(self.page == Page::About, "Über").clicked() {
                    self.page = Page::About;
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.page {
                Page::Home => self.page_home(ui),
                Page::Routes => self.page_routes(ui),
                Page::Drive => self.page_drive(ui),
                Page::Settings => self.page_settings(ui),
                Page::About => self.page_about(ui),
            }
        });
    }
}

impl LauncherApp {
    fn page_home(&mut self, ui: &mut egui::Ui) {
        ui.heading("Train Simulator Classic — Open Source Laufzeit");
        ui.label("Deine gekauften Strecken bleiben auf der Platte; dieses Programm liefert keine Spielinhalte.");
        ui.separator();
        if let Some(err) = &self.install_error {
            ui.colored_label(egui::Color32::from_rgb(255, 180, 80), err);
            if ui.button("Zu Einstellungen").clicked() {
                self.page = Page::Settings;
            }
        } else if let Some(install) = &self.install {
            ui.label(format!("Installation: {}", install.root.display()));
            if let Some(catalog) = &self.catalog {
                ui.label(format!(
                    "Strecken: {} · .ap-Archive: {}",
                    catalog.routes.len(),
                    self.ap_count.unwrap_or(0)
                ));
                let hits = catalog_matches_benchmarks(catalog);
                if !hits.is_empty() {
                    ui.label(format!("Benchmark im Setup: {}", hits.join(", ")));
                }
            }
            ui.horizontal(|ui| {
                if ui.button("Strecken durchsuchen").clicked() {
                    self.page = Page::Routes;
                }
                if ui.button("Fahren").clicked() {
                    self.page = Page::Drive;
                }
            });
        }
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

    fn page_drive(&mut self, ui: &mut egui::Ui) {
        ui.heading("Fahren");
        let route = self
            .selected_route
            .as_deref()
            .unwrap_or("(keine Strecke gewählt)");
        ui.label(format!("Strecke: {route}"));
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.drive_mode, DriveMode::FreeRoam, "Free Roam");
            ui.radio_value(&mut self.drive_mode, DriveMode::Scenario, "Szenario (bald)");
        });
        ui.label(format!(
            "Grafik: {:?} · {}×{} @ {} FPS",
            self.config.graphics_profile,
            self.config.display_width,
            self.config.display_height,
            self.config.target_fps
        ));
        if ui
            .add(egui::Button::new("Fahrt starten").min_size(egui::vec2(200.0, 40.0)))
            .clicked()
        {
            self.try_start_drive();
        }
        if !self.status.is_empty() {
            ui.separator();
            ui.label(&self.status);
        }
    }

    fn page_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("Einstellungen");
        ui.label("RailWorks-Installationsordner (read-only):");
        let mut path_text = self
            .config
            .railworks_path
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_default();
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut path_text);
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
                }
            }
        });
        if !path_text.is_empty() {
            self.config.railworks_path = Some(PathBuf::from(&path_text));
        }

        egui::ComboBox::from_label("Update-Kanal")
            .selected_text(format!("{:?}", self.config.update_channel))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut self.config.update_channel,
                    UpdateChannel::Stable,
                    "Stable",
                );
                ui.selectable_value(&mut self.config.update_channel, UpdateChannel::Beta, "Beta");
            });

        egui::ComboBox::from_label("Grafikprofil")
            .selected_text(format!("{:?}", self.config.graphics_profile))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut self.config.graphics_profile,
                    GraphicsProfile::Vanilla,
                    "Vanilla",
                );
                ui.selectable_value(
                    &mut self.config.graphics_profile,
                    GraphicsProfile::Enhanced,
                    "Enhanced (später)",
                );
            });

        ui.horizontal(|ui| {
            ui.label("Auflösung");
            ui.add(
                egui::DragValue::new(&mut self.config.display_width)
                    .suffix(" px"),
            );
            ui.label("×");
            ui.add(
                egui::DragValue::new(&mut self.config.display_height)
                    .suffix(" px"),
            );
        });
        ui.add(egui::DragValue::new(&mut self.config.target_fps).range(30..=240).suffix(" FPS"));

        if ui.button("Speichern").clicked() {
            self.save_config();
        }
        if let Some(hint) = &self.config_saved_hint {
            ui.small(hint);
        }
    }

    fn page_about(&mut self, ui: &mut egui::Ui) {
        ui.heading("Über OpenRailWorks");
        ui.label(format!("Version {}", env!("CARGO_PKG_VERSION")));
        ui.label("Inspiriert von openOMSI — neu implementierte TSC-Laufzeit (AGPL-3.0).");
        ui.hyperlink_to(
            "GitHub",
            "https://github.com/Gegels/openrailworks",
        );
        ui.label("Updates: Stable/Beta über GitHub Releases (Auto-Update folgt).");
    }
}
