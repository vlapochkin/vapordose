use libadwaita::prelude::*;
use libadwaita::{ActionRow, ApplicationWindow, ExpanderRow, HeaderBar, MessageDialog, PreferencesGroup, ResponseAppearance, StatusPage, Toast, ToastOverlay};
use gtk4::{gdk, gio, glib, Align, Box, Button, DropDown, Entry, Image, Label, LinkButton, ListBox, MenuButton, Orientation, ProgressBar, ScrolledWindow, SearchEntry, SelectionMode, Spinner, StringList};
use gtk4::pango::EllipsizeMode;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::process::Command;
use std::rc::Rc;
use std::sync::Arc;

use crate::config::{AppConfig, Language};
use crate::injector::{DeployOutcome, DeployWarning, Injector};
use crate::steam_scanner::{discover_steam_libraries, scan_games, AppCategory, GameType, SteamGame};

struct Translations {
    title: &'static str,
    scanning: &'static str,
    games_not_found: &'static str,
    apply_patch: &'static str,
    restore: &'static str,
    status_patched: &'static str,
    status_safe: &'static str,
    status_warn: &'static str,
    open_folder: &'static str,
    copy_params: &'static str,
    dlc_list_title: &'static str,
    dlc_none: &'static str,
    view_store: &'static str,
    view_protondb: &'static str,
    group_games: &'static str,
    group_system: &'static str,
    system_expander: &'static str,
    confirm_title: &'static str,
    confirm_body: &'static str,
    confirm_yes: &'static str,
    confirm_no: &'static str,
    search_placeholder: &'static str,
    batch_patch: &'static str,
    batch_restore: &'static str,
    batch_patch_heading: &'static str,
    batch_patch_body: &'static str,
    batch_restore_heading: &'static str,
    batch_restore_body: &'static str,
    /// `{n}` = count, `{f}` = failures
    batch_patched: &'static str,
    batch_restored: &'static str,
    batch_nothing_to_patch: &'static str,
    batch_nothing_to_restore: &'static str,
    edit_dlc: &'static str,
    edit_heading: &'static str,
    edit_body: &'static str,
    fetch_dlc: &'static str,
    fetching_dlc: &'static str,
    /// `{n}` = count
    fetched_dlc: &'static str,
    fetch_failed: &'static str,
    config_saved: &'static str,
    config_save_failed: &'static str,
    filter_all: &'static str,
    filter_dlc: &'static str,
    filter_patched: &'static str,
    filter_anticheat: &'static str,
    save: &'static str,
    cancel: &'static str,
    success: &'static str,
    restored: &'static str,
    copied: &'static str,
    error_prefix: &'static str,
    warn_steam_running: &'static str,
    warn_launch_options: &'static str,
    menu_update_smokeapi: &'static str,
    checking_updates: &'static str,
    update_heading: &'static str,
    /// `{cur}` / `{new}`
    update_body: &'static str,
    update_button: &'static str,
    downloading: &'static str,
    /// `{v}`
    updated_to: &'static str,
    update_error: &'static str,
    /// `{v}`
    up_to_date: &'static str,
    check_error: &'static str,
}

const EN_TRANS: Translations = Translations {
    title: "VaporDose",
    scanning: "Scanning games...",
    games_not_found: "No Steam games found.",
    apply_patch: "Patch DLC",
    restore: "Restore",
    status_patched: "Patched",
    status_safe: "Safe",
    status_warn: "Risk: anti-cheat / online",
    open_folder: "Open folder",
    copy_params: "Copy launch options",
    dlc_list_title: "DLC IDs",
    dlc_none: "None",
    view_store: "Steam Store",
    view_protondb: "ProtonDB",
    group_games: "Installed games",
    group_system: "System components",
    system_expander: "System components and tools (Proton / SDK)",
    confirm_title: "Warning",
    confirm_body: "This game has anti-cheat or online features. Patching may lead to a ban. Continue?",
    confirm_yes: "Yes",
    confirm_no: "Cancel",
    search_placeholder: "Search games...",
    batch_patch: "Patch all safe",
    batch_restore: "Restore all",
    batch_patch_heading: "Patch all games",
    batch_patch_body: "Patch every safe game automatically?\n\nGames with anti-cheat or online features will be skipped.",
    batch_restore_heading: "Restore all games",
    batch_restore_body: "Restore the original files for every patched game?",
    batch_patched: "Patched games: {n} (failed: {f})",
    batch_restored: "Restored games: {n} (failed: {f})",
    batch_nothing_to_patch: "No games available for patching",
    batch_nothing_to_restore: "No patched games to restore",
    edit_dlc: "DLC config",
    edit_heading: "Edit DLC list / config",
    edit_body: "Enter DLC IDs separated by commas, or fetch them from Steam:",
    fetch_dlc: "Fetch DLC list from Steam",
    fetching_dlc: "Fetching DLC from Steam...",
    fetched_dlc: "Fetched {n} DLCs",
    fetch_failed: "No DLCs found or Steam API error",
    config_saved: "Config saved",
    config_save_failed: "Failed to save config",
    filter_all: "All",
    filter_dlc: "Has DLC",
    filter_patched: "Patched",
    filter_anticheat: "Anti-cheat",
    save: "Save",
    cancel: "Cancel",
    success: "Patched successfully",
    restored: "Restored",
    copied: "Copied",
    error_prefix: "Error",
    warn_steam_running: "Steam is running: restart it, otherwise the launch options may be lost",
    warn_launch_options: "Could not update Steam launch options. Set them manually (copy button)",
    menu_update_smokeapi: "Update SmokeAPI",
    checking_updates: "Checking SmokeAPI updates...",
    update_heading: "SmokeAPI update available",
    update_body: "Installed: {cur}\nLatest: {new}\n\nDownload and install?",
    update_button: "Update",
    downloading: "Downloading SmokeAPI...",
    updated_to: "SmokeAPI updated to {v}",
    update_error: "Update failed",
    up_to_date: "SmokeAPI is up to date ({v})",
    check_error: "Update check failed",
};

const RU_TRANS: Translations = Translations {
    title: "VaporDose",
    scanning: "Поиск игр...",
    games_not_found: "Игры Steam не найдены.",
    apply_patch: "Патчить",
    restore: "Вернуть",
    status_patched: "Готово",
    status_safe: "Безопасно",
    status_warn: "Риск: античит / онлайн",
    open_folder: "Открыть папку",
    copy_params: "Копировать параметры запуска",
    dlc_list_title: "ID DLC",
    dlc_none: "Нет",
    view_store: "Магазин",
    view_protondb: "ProtonDB",
    group_games: "Установленные игры",
    group_system: "Системные компоненты",
    system_expander: "Системные компоненты и инструменты (Proton / SDK)",
    confirm_title: "Внимание",
    confirm_body: "В игре есть античит или онлайн-функции. Патч может привести к бану. Продолжить?",
    confirm_yes: "Да",
    confirm_no: "Отмена",
    search_placeholder: "Поиск игр...",
    batch_patch: "Патчить все безопасные",
    batch_restore: "Восстановить всё",
    batch_patch_heading: "Патчинг всех игр",
    batch_patch_body: "Автоматически пропатчить все безопасные игры?\n\nИгры с античитом и сетевым режимом будут пропущены.",
    batch_restore_heading: "Восстановление всех игр",
    batch_restore_body: "Восстановить оригинальные файлы для всех пропатченных игр?",
    batch_patched: "Пропатчено игр: {n} (ошибок: {f})",
    batch_restored: "Восстановлено игр: {n} (ошибок: {f})",
    batch_nothing_to_patch: "Нет доступных игр для патчинга",
    batch_nothing_to_restore: "Нет пропатченных игр для восстановления",
    edit_dlc: "Конфиг DLC",
    edit_heading: "Редактировать список DLC / конфиг",
    edit_body: "Введите ID DLC через запятую или загрузите их из Steam:",
    fetch_dlc: "Загрузить список DLC из Steam",
    fetching_dlc: "Загрузка DLC из Steam...",
    fetched_dlc: "Загружено DLC: {n}",
    fetch_failed: "DLC не найдены или ошибка Steam API",
    config_saved: "Конфиг сохранён",
    config_save_failed: "Не удалось сохранить конфиг",
    filter_all: "Все",
    filter_dlc: "Есть DLC",
    filter_patched: "Пропатчено",
    filter_anticheat: "Античит",
    save: "Сохранить",
    cancel: "Отмена",
    success: "Успешно пропатчено",
    restored: "Восстановлено",
    copied: "Скопировано",
    error_prefix: "Ошибка",
    warn_steam_running: "Steam запущен: перезапустите его, иначе параметры запуска могут потеряться",
    warn_launch_options: "Не удалось обновить параметры запуска Steam. Задайте их вручную (кнопка копирования)",
    menu_update_smokeapi: "Обновить SmokeAPI",
    checking_updates: "Проверка обновлений SmokeAPI...",
    update_heading: "Доступно обновление SmokeAPI",
    update_body: "Установлена версия: {cur}\nДоступна версия: {new}\n\nЗагрузить и обновить?",
    update_button: "Обновить",
    downloading: "Загрузка SmokeAPI...",
    updated_to: "SmokeAPI обновлён до {v}",
    update_error: "Ошибка обновления",
    up_to_date: "Установлена последняя версия SmokeAPI ({v})",
    check_error: "Ошибка проверки обновлений",
};

fn tr(lang: Language) -> &'static Translations {
    match lang {
        Language::RU => &RU_TRANS,
        Language::EN => &EN_TRANS,
    }
}

fn warning_text(trans: &Translations, warning: DeployWarning) -> &'static str {
    match warning {
        DeployWarning::SteamRunning => trans.warn_steam_running,
        DeployWarning::LaunchOptionsNotApplied => trans.warn_launch_options,
    }
}

/// Shared UI state. Everything here lives on the GTK main thread.
#[derive(Clone)]
struct AppState {
    lang: Rc<Cell<Language>>,
    injector: Arc<Injector>,
    games: Rc<RefCell<Vec<SteamGame>>>,
    rows: Rc<RefCell<HashMap<String, ExpanderRow>>>,
    toast_overlay: ToastOverlay,
    window: ApplicationWindow,
}

impl AppState {
    fn trans(&self) -> &'static Translations {
        tr(self.lang.get())
    }

    fn toast(&self, text: &str) {
        self.toast_overlay.add_toast(Toast::new(text));
    }

    fn set_patched(&self, appid: &str, patched: bool) {
        if let Some(game) = self.games.borrow_mut().iter_mut().find(|g| g.appid == appid) {
            game.is_patched = patched;
        }
    }

    /// Reports the result of a patch/restore run to the user. Returns `true` on success.
    fn report_outcome(&self, result: Result<anyhow::Result<DeployOutcome>, tokio::task::JoinError>, success_text: &str) -> bool {
        let trans = self.trans();
        match result {
            Ok(Ok(outcome)) => {
                self.toast(success_text);
                for warning in outcome.warnings {
                    self.toast(warning_text(trans, warning));
                }
                true
            }
            Ok(Err(e)) => {
                self.toast(&format!("{}: {}", trans.error_prefix, e));
                false
            }
            Err(_) => {
                self.toast(trans.error_prefix);
                false
            }
        }
    }
}

fn build_menu(trans: &Translations) -> gio::Menu {
    let m = gio::Menu::new();
    m.append(Some("Русский"), Some("app.set_lang_ru"));
    m.append(Some("English"), Some("app.set_lang_en"));
    m.append(Some(trans.menu_update_smokeapi), Some("app.update_smokeapi"));
    m
}

fn is_batch_patchable(game: &SteamGame) -> bool {
    game.category != AppCategory::SystemTool
        && !game.has_anticheat
        && !game.is_online_multiplayer
        && !game.is_patched
        && !game.targets.is_empty()
}

pub fn build_ui(app: &libadwaita::Application) {
    let config = AppConfig::load();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("VaporDose")
        .default_width(920)
        .default_height(640)
        .width_request(360)
        .height_request(500)
        .build();

    let toast_overlay = ToastOverlay::new();
    let state = AppState {
        lang: Rc::new(Cell::new(config.language())),
        injector: Arc::new(Injector::new()),
        games: Rc::new(RefCell::new(Vec::new())),
        rows: Rc::new(RefCell::new(HashMap::new())),
        toast_overlay: toast_overlay.clone(),
        window: window.clone(),
    };

    let main_box = Box::new(Orientation::Vertical, 0);
    let header_bar = HeaderBar::new();
    main_box.append(&header_bar);

    let search_box = Box::builder().margin_top(12).margin_start(24).margin_end(24).spacing(12).build();
    let search_entry = SearchEntry::builder().hexpand(true).build();
    let filter_model = StringList::new(&[]);
    let filter_dropdown = DropDown::builder().model(&filter_model).build();
    search_box.append(&search_entry);
    search_box.append(&filter_dropdown);
    main_box.append(&search_box);

    let scrolled_window = ScrolledWindow::builder().vexpand(true).build();
    let content_box = Box::builder().orientation(Orientation::Vertical).margin_top(12).margin_bottom(24).margin_start(24).margin_end(24).spacing(24).build();
    let games_group = PreferencesGroup::builder().build();
    let games_list = ListBox::builder().selection_mode(SelectionMode::None).css_classes(vec!["boxed-list".to_string()]).build();
    games_group.add(&games_list);
    let system_group = PreferencesGroup::builder().build();
    let system_expander = ExpanderRow::builder().enable_expansion(true).expanded(false).build();
    let system_list = ListBox::builder().selection_mode(SelectionMode::None).css_classes(vec!["boxed-list".to_string()]).build();
    system_expander.add_row(&system_list);
    system_group.add(&system_expander);
    content_box.append(&games_group);
    content_box.append(&system_group);

    let loading_box = Box::builder().orientation(Orientation::Vertical).spacing(20).valign(Align::Center).halign(Align::Center).build();
    let spinner = Spinner::builder().spinning(true).width_request(64).height_request(64).build();
    let loading_label = Label::builder().css_classes(vec!["title-1".to_string()]).build();
    let progress_bar = ProgressBar::builder().width_request(400).build();
    loading_box.append(&spinner);
    loading_box.append(&loading_label);
    loading_box.append(&progress_bar);

    let empty_page = StatusPage::builder().icon_name("system-search-symbolic").build();

    scrolled_window.set_child(Some(&loading_box));
    main_box.append(&scrolled_window);
    toast_overlay.set_child(Some(&main_box));
    window.set_content(Some(&toast_overlay));

    let lang_menu_btn = MenuButton::builder().icon_name("open-menu-symbolic").build();
    let batch_patch_btn = Button::builder().css_classes(vec!["suggested-action".to_string()]).build();
    let batch_restore_btn = Button::builder().css_classes(vec!["destructive-action".to_string()]).build();
    let refresh_button = Button::builder().icon_name("view-refresh-symbolic").build();
    header_bar.pack_start(&lang_menu_btn);
    header_bar.pack_start(&batch_patch_btn);
    header_bar.pack_start(&batch_restore_btn);
    header_bar.pack_end(&refresh_button);

    // --- Filtering: driven by game data, not by subtitle text ---
    let filter_func: Rc<dyn Fn()> = {
        let state = state.clone();
        let search_entry = search_entry.clone();
        let filter_dropdown = filter_dropdown.clone();
        let games_group = games_group.clone();
        let system_group = system_group.clone();
        Rc::new(move || {
            let text = search_entry.text().to_lowercase();
            let selected_idx = filter_dropdown.selected();
            let rows = state.rows.borrow();
            let mut any_game_visible = false;
            let mut any_system_visible = false;

            for game in state.games.borrow().iter() {
                let Some(row) = rows.get(&game.appid) else { continue };
                let title_match = text.is_empty() || game.name.to_lowercase().contains(&text);
                let category_match = match selected_idx {
                    1 => !game.dlc_list.is_empty(),
                    2 => game.is_patched,
                    3 => game.has_anticheat || game.is_online_multiplayer,
                    _ => true,
                };
                let visible = title_match && category_match;
                row.set_visible(visible);
                if visible {
                    if game.category == AppCategory::SystemTool { any_system_visible = true; } else { any_game_visible = true; }
                }
            }
            games_group.set_visible(any_game_visible);
            system_group.set_visible(any_system_visible);
        })
    };

    // --- Rescan everything and rebuild the list (also re-applies the current language) ---
    let run_scan: Rc<dyn Fn()> = {
        let state = state.clone();
        let search_entry = search_entry.clone();
        let filter_model = filter_model.clone();
        let filter_dropdown = filter_dropdown.clone();
        let games_group = games_group.clone();
        let system_group = system_group.clone();
        let system_expander = system_expander.clone();
        let games_list = games_list.clone();
        let system_list = system_list.clone();
        let scrolled_window = scrolled_window.clone();
        let content_box = content_box.clone();
        let loading_box = loading_box.clone();
        let loading_label = loading_label.clone();
        let progress_bar = progress_bar.clone();
        let empty_page = empty_page.clone();
        let lang_menu_btn = lang_menu_btn.clone();
        let batch_patch_btn = batch_patch_btn.clone();
        let batch_restore_btn = batch_restore_btn.clone();
        let filter_func = filter_func.clone();
        Rc::new(move || {
            let trans = state.trans();
            state.window.set_title(Some(trans.title));
            loading_label.set_label(trans.scanning);
            search_entry.set_placeholder_text(Some(trans.search_placeholder));
            games_group.set_title(trans.group_games);
            system_group.set_title(trans.group_system);
            system_expander.set_title(trans.system_expander);
            empty_page.set_title(trans.games_not_found);
            batch_patch_btn.set_label(trans.batch_patch);
            batch_restore_btn.set_label(trans.batch_restore);
            lang_menu_btn.set_menu_model(Some(&build_menu(trans)));
            // Keep the user's filter across rescans; on the very first fill the model is
            // empty and `selected()` is INVALID_LIST_POSITION, which must map to "All".
            let selected = filter_dropdown.selected();
            filter_model.splice(0, filter_model.n_items(), &[trans.filter_all, trans.filter_dlc, trans.filter_patched, trans.filter_anticheat]);
            let keep = if selected == gtk4::INVALID_LIST_POSITION { 0 } else { selected.min(3) };
            filter_dropdown.set_selected(keep);

            progress_bar.set_fraction(0.1);
            scrolled_window.set_child(Some(&loading_box));

            let state = state.clone();
            let games_list = games_list.clone();
            let system_list = system_list.clone();
            let scrolled_window = scrolled_window.clone();
            let content_box = content_box.clone();
            let progress_bar = progress_bar.clone();
            let empty_page = empty_page.clone();
            let filter_func = filter_func.clone();

            glib::spawn_future_local(async move {
                let libraries = tokio::task::spawn_blocking(discover_steam_libraries).await.unwrap_or_default();
                progress_bar.set_fraction(0.3);
                let games = scan_games(&libraries).await;
                progress_bar.set_fraction(0.9);

                while let Some(child) = games_list.first_child() { games_list.remove(&child); }
                while let Some(child) = system_list.first_child() { system_list.remove(&child); }
                state.rows.borrow_mut().clear();

                if games.is_empty() {
                    scrolled_window.set_child(Some(&empty_page));
                } else {
                    scrolled_window.set_child(Some(&content_box));
                    for game in &games {
                        let row = create_game_row(game.clone(), state.clone());
                        if game.category == AppCategory::SystemTool { system_list.append(&row); } else { games_list.append(&row); }
                        state.rows.borrow_mut().insert(game.appid.clone(), row);
                    }
                }
                *state.games.borrow_mut() = games;
                filter_func();
                progress_bar.set_fraction(1.0);
            });
        })
    };

    {
        let f = filter_func.clone();
        search_entry.connect_search_changed(move |_| f());
        let f = filter_func.clone();
        filter_dropdown.connect_selected_notify(move |_| f());
    }

    // --- Language actions (persisted to ~/.config/vapordose/config.json) ---
    for (name, lang) in [("set_lang_ru", Language::RU), ("set_lang_en", Language::EN)] {
        let action = gio::SimpleAction::new(name, None);
        let state = state.clone();
        let run_scan = run_scan.clone();
        action.connect_activate(move |_, _| {
            if state.lang.get() == lang { return; }
            state.lang.set(lang);
            AppConfig { language: Some(lang) }.save();
            run_scan();
        });
        app.add_action(&action);
    }

    // --- SmokeAPI update action ---
    {
        let action = gio::SimpleAction::new("update_smokeapi", None);
        let state = state.clone();
        action.connect_activate(move |_, _| {
            let state = state.clone();
            glib::spawn_future_local(async move {
                let trans = state.trans();
                state.toast(trans.checking_updates);
                let status = match crate::smokeapi_manager::check_for_updates().await {
                    Ok(status) => status,
                    Err(e) => { state.toast(&format!("{}: {}", trans.check_error, e)); return; }
                };
                if !status.update_available {
                    state.toast(&trans.up_to_date.replace("{v}", &status.installed_version));
                    return;
                }
                let latest = status.latest_version.unwrap_or_default();
                let dialog = MessageDialog::builder()
                    .transient_for(&state.window)
                    .heading(trans.update_heading)
                    .body(trans.update_body.replace("{cur}", &status.installed_version).replace("{new}", &latest))
                    .build();
                dialog.add_response("cancel", trans.cancel);
                dialog.add_response("update", trans.update_button);
                dialog.set_response_appearance("update", ResponseAppearance::Suggested);
                dialog.set_default_response(Some("update"));
                dialog.set_close_response("cancel");

                let state = state.clone();
                dialog.connect_response(None, move |d, res| {
                    d.close();
                    if res != "update" { return; }
                    let state = state.clone();
                    glib::spawn_future_local(async move {
                        let trans = state.trans();
                        state.toast(trans.downloading);
                        match crate::smokeapi_manager::download_and_install_latest().await {
                            Ok(ver) => state.toast(&trans.updated_to.replace("{v}", &ver)),
                            Err(e) => state.toast(&format!("{}: {}", trans.update_error, e)),
                        }
                    });
                });
                dialog.present();
            });
        });
        app.add_action(&action);
    }

    // --- Batch patch ---
    {
        let state = state.clone();
        let run_scan = run_scan.clone();
        batch_patch_btn.connect_clicked(move |_| {
            let trans = state.trans();
            let dialog = MessageDialog::builder()
                .transient_for(&state.window)
                .heading(trans.batch_patch_heading)
                .body(trans.batch_patch_body)
                .build();
            dialog.add_response("cancel", trans.cancel);
            dialog.add_response("patch", trans.batch_patch);
            dialog.set_response_appearance("patch", ResponseAppearance::Suggested);
            dialog.set_default_response(Some("cancel"));
            dialog.set_close_response("cancel");

            let state = state.clone();
            let run_scan = run_scan.clone();
            dialog.connect_response(None, move |d, response| {
                d.close();
                if response != "patch" { return; }
                let games: Vec<SteamGame> = state.games.borrow().iter().filter(|g| is_batch_patchable(g)).cloned().collect();
                let trans = state.trans();
                if games.is_empty() {
                    state.toast(trans.batch_nothing_to_patch);
                    return;
                }
                let injector = state.injector.clone();
                let state = state.clone();
                let run_scan = run_scan.clone();
                glib::spawn_future_local(async move {
                    let (ok, failed, first_error) = tokio::task::spawn_blocking(move || {
                        let (mut ok, mut failed, mut first_error) = (0usize, 0usize, None);
                        for game in games {
                            match injector.backup_and_deploy(&game) {
                                Ok(_) => ok += 1,
                                Err(e) => {
                                    failed += 1;
                                    if first_error.is_none() { first_error = Some(format!("{}: {}", game.name, e)); }
                                }
                            }
                        }
                        (ok, failed, first_error)
                    }).await.unwrap_or((0, 0, None));

                    state.toast(&trans.batch_patched.replace("{n}", &ok.to_string()).replace("{f}", &failed.to_string()));
                    if let Some(err) = first_error { state.toast(&format!("{}: {}", trans.error_prefix, err)); }
                    run_scan();
                });
            });
            dialog.present();
        });
    }

    // --- Batch restore ---
    {
        let state = state.clone();
        let run_scan = run_scan.clone();
        batch_restore_btn.connect_clicked(move |_| {
            let trans = state.trans();
            let dialog = MessageDialog::builder()
                .transient_for(&state.window)
                .heading(trans.batch_restore_heading)
                .body(trans.batch_restore_body)
                .build();
            dialog.add_response("cancel", trans.cancel);
            dialog.add_response("restore", trans.batch_restore);
            dialog.set_response_appearance("restore", ResponseAppearance::Destructive);
            dialog.set_default_response(Some("cancel"));
            dialog.set_close_response("cancel");

            let state = state.clone();
            let run_scan = run_scan.clone();
            dialog.connect_response(None, move |d, response| {
                d.close();
                if response != "restore" { return; }
                let games: Vec<SteamGame> = state.games.borrow().iter().filter(|g| g.is_patched).cloned().collect();
                let trans = state.trans();
                if games.is_empty() {
                    state.toast(trans.batch_nothing_to_restore);
                    return;
                }
                let injector = state.injector.clone();
                let state = state.clone();
                let run_scan = run_scan.clone();
                glib::spawn_future_local(async move {
                    let (ok, failed, first_error) = tokio::task::spawn_blocking(move || {
                        let (mut ok, mut failed, mut first_error) = (0usize, 0usize, None);
                        for game in games {
                            match injector.restore_original(&game) {
                                Ok(_) => ok += 1,
                                Err(e) => {
                                    failed += 1;
                                    if first_error.is_none() { first_error = Some(format!("{}: {}", game.name, e)); }
                                }
                            }
                        }
                        (ok, failed, first_error)
                    }).await.unwrap_or((0, 0, None));

                    state.toast(&trans.batch_restored.replace("{n}", &ok.to_string()).replace("{f}", &failed.to_string()));
                    if let Some(err) = first_error { state.toast(&format!("{}: {}", trans.error_prefix, err)); }
                    run_scan();
                });
            });
            dialog.present();
        });
    }

    {
        let run_scan = run_scan.clone();
        refresh_button.connect_clicked(move |_| run_scan());
    }

    run_scan();
    window.present();
}

fn create_game_row(game: SteamGame, state: AppState) -> ExpanderRow {
    let trans = state.trans();

    let mut subtitle_info = Vec::new();
    if game.is_patched { subtitle_info.push("[Patched]"); }
    if !game.dlc_list.is_empty() { subtitle_info.push("[DLC]"); }
    if let Some(ac) = &game.anticheat_name { subtitle_info.push(ac.as_str()); }

    let full_subtitle = format!("{} {}", game.install_dir.to_string_lossy(), subtitle_info.join(" "));
    let row = ExpanderRow::builder()
        .title(glib::markup_escape_text(&game.name))
        .subtitle(glib::markup_escape_text(&full_subtitle))
        .focusable(true)
        .build();

    if let Some(child) = row.first_child() {
        if let Some(box_widget) = child.downcast_ref::<Box>() {
            let mut next = box_widget.first_child();
            while let Some(w) = next {
                if let Some(lbl) = w.downcast_ref::<Label>() {
                    lbl.set_ellipsize(EllipsizeMode::End);
                    lbl.set_max_width_chars(45);
                }
                next = w.next_sibling();
            }
        }
    }

    // --- Prefix: icon + folder button ---
    let prefix_box = Box::new(Orientation::Horizontal, 12);
    let icon_img = Image::builder().width_request(40).height_request(40).build();
    let mut icon_loaded = false;

    let icon_name = format!("steam_icon_{}", game.appid);
    if let Some(display) = gdk::Display::default() {
        let theme = gtk4::IconTheme::for_display(&display);
        if theme.has_icon(&icon_name) {
            icon_img.set_pixel_size(40);
            icon_img.set_icon_name(Some(&icon_name));
            icon_loaded = true;
        }
    }
    if !icon_loaded {
        if let Some(path) = &game.icon_path {
            match gdk::Texture::from_filename(path) {
                Ok(texture) => icon_img.set_paintable(Some(&texture)),
                Err(_) => icon_img.set_from_file(Some(path)),
            }
            icon_loaded = true;
        }
    }
    if !icon_loaded {
        icon_img.set_pixel_size(40);
        icon_img.set_icon_name(Some("input-gaming-symbolic"));
    }
    prefix_box.append(&icon_img);

    let folder_btn = Button::builder().icon_name("folder-open-symbolic").css_classes(vec!["flat".to_string()]).tooltip_text(trans.open_folder).focusable(true).build();
    {
        let dir = game.install_dir.clone();
        folder_btn.connect_clicked(move |_| { Command::new("xdg-open").arg(&dir).spawn().ok(); });
    }
    prefix_box.append(&folder_btn);
    row.add_prefix(&prefix_box);

    // --- Suffix: status dot + action buttons ---
    let suffix_box = Box::new(Orientation::Horizontal, 8);
    suffix_box.set_valign(Align::Center);
    let (status_color, status_text, can_patch) = match game.category {
        AppCategory::SystemTool => ("error", trans.group_system, false),
        _ if game.has_anticheat => ("warning", trans.status_warn, true),
        _ => ("success", trans.status_safe, true),
    };
    let status_dot = Label::builder().label("●").css_classes(vec![status_color.to_string(), "title-1".to_string()]).tooltip_text(status_text).build();
    suffix_box.append(&status_dot);
    let apply_btn = Button::builder().css_classes(vec!["suggested-action".to_string()]).focusable(true).build();
    let restore_btn = Button::builder().label(trans.restore).css_classes(vec!["destructive-action".to_string()]).focusable(true).build();
    let copy_btn = Button::builder().icon_name("edit-copy-symbolic").tooltip_text(trans.copy_params).focusable(true).build();
    suffix_box.append(&apply_btn);
    suffix_box.append(&restore_btn);
    suffix_box.append(&copy_btn);
    row.add_suffix(&suffix_box);

    // --- Expanded details: DLC list + links ---
    let details_group = PreferencesGroup::builder().margin_top(12).margin_bottom(12).build();
    let dlc_text = if game.dlc_list.is_empty() {
        trans.dlc_none.to_string()
    } else {
        game.dlc_list.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(", ")
    };
    let dlc_row = ActionRow::builder().title(trans.dlc_list_title).subtitle(&dlc_text).build();
    let edit_dlc_btn = Button::builder().label(trans.edit_dlc).css_classes(vec!["flat".to_string()]).focusable(true).build();
    {
        let state = state.clone();
        let game = game.clone();
        let dlc_row = dlc_row.clone();
        let initial_ids = if game.dlc_list.is_empty() { String::new() } else { dlc_text.clone() };
        edit_dlc_btn.connect_clicked(move |_| {
            let trans = state.trans();
            let dialog = MessageDialog::builder()
                .transient_for(&state.window)
                .heading(trans.edit_heading)
                .body(trans.edit_body)
                .build();

            let container = Box::new(Orientation::Vertical, 12);
            container.set_margin_top(12);
            container.set_margin_start(24);
            container.set_margin_end(24);

            let current = dlc_row.subtitle().map(|s| s.to_string()).unwrap_or_default();
            let entry_text = if current == trans.dlc_none { initial_ids.clone() } else { current };
            let entry = Entry::builder().text(&entry_text).focusable(true).build();
            let fetch_btn = Button::builder().label(trans.fetch_dlc).css_classes(vec!["suggested-action".to_string()]).focusable(true).build();

            {
                let entry = entry.clone();
                let appid = game.appid.clone();
                let state = state.clone();
                fetch_btn.connect_clicked(move |btn| {
                    let trans = state.trans();
                    btn.set_sensitive(false);
                    btn.set_label(trans.fetching_dlc);
                    let entry = entry.clone();
                    let appid = appid.clone();
                    let state = state.clone();
                    let btn = btn.clone();
                    glib::spawn_future_local(async move {
                        let fetcher = crate::steam_api::SteamApiFetcher::new();
                        match fetcher.fetch_dlcs_for_app(&appid).await {
                            Ok(dlcs) if !dlcs.is_empty() => {
                                let ids_str = dlcs.iter().map(|(id, _)| id.to_string()).collect::<Vec<_>>().join(", ");
                                entry.set_text(&ids_str);
                                state.toast(&trans.fetched_dlc.replace("{n}", &dlcs.len().to_string()));
                            }
                            Ok(_) => state.toast(trans.fetch_failed),
                            Err(e) => state.toast(&format!("{}: {}", trans.fetch_failed, e)),
                        }
                        btn.set_label(trans.fetch_dlc);
                        btn.set_sensitive(true);
                    });
                });
            }

            container.append(&entry);
            container.append(&fetch_btn);
            dialog.set_extra_child(Some(&container));
            dialog.add_response("cancel", trans.cancel);
            dialog.add_response("save", trans.save);
            dialog.set_response_appearance("save", ResponseAppearance::Suggested);
            dialog.set_close_response("cancel");

            let state = state.clone();
            let game = game.clone();
            let dlc_row = dlc_row.clone();
            dialog.connect_response(None, move |d, res| {
                d.close();
                if res != "save" { return; }
                let trans = state.trans();
                let text = entry.text().to_string();
                match state.injector.save_custom_config(&game, &text) {
                    Ok(()) => {
                        dlc_row.set_subtitle(if text.trim().is_empty() { trans.dlc_none } else { &text });
                        state.toast(trans.config_saved);
                    }
                    Err(e) => state.toast(&format!("{}: {}", trans.config_save_failed, e)),
                }
            });
            dialog.present();
        });
    }
    dlc_row.add_suffix(&edit_dlc_btn);
    details_group.add(&dlc_row);

    let links_box = Box::new(Orientation::Horizontal, 12);
    links_box.set_margin_start(12);
    links_box.set_margin_bottom(12);
    links_box.append(&LinkButton::with_label(&format!("https://store.steampowered.com/app/{}", game.appid), trans.view_store));
    links_box.append(&LinkButton::with_label(&format!("https://www.protondb.com/app/{}", game.appid), trans.view_protondb));
    let expanded_box = Box::new(Orientation::Vertical, 0);
    expanded_box.append(&details_group);
    expanded_box.append(&links_box);
    row.add_row(&expanded_box);

    // --- Button state ---
    let has_proton_target = game.game_type == GameType::Proton || game.game_type == GameType::Mixed;
    let update_row_state: Rc<dyn Fn(bool)> = {
        let apply_btn = apply_btn.clone();
        let restore_btn = restore_btn.clone();
        let copy_btn = copy_btn.clone();
        let state = state.clone();
        let appid = game.appid.clone();
        Rc::new(move |is_patched: bool| {
            let trans = state.trans();
            state.set_patched(&appid, is_patched);
            if is_patched {
                apply_btn.set_label(trans.status_patched);
                apply_btn.set_sensitive(false);
                restore_btn.set_sensitive(true);
                copy_btn.set_visible(has_proton_target);
            } else {
                apply_btn.set_label(trans.apply_patch);
                apply_btn.set_sensitive(can_patch);
                restore_btn.set_sensitive(false);
                copy_btn.set_visible(false);
            }
        })
    };
    update_row_state(game.is_patched);

    {
        let state = state.clone();
        let game = game.clone();
        copy_btn.connect_clicked(move |_| {
            if let Some(instr) = state.injector.get_proton_instructions(&game) {
                if let Some(display) = gdk::Display::default() {
                    display.clipboard().set_text(&instr);
                    state.toast(state.trans().copied);
                }
            }
        });
    }

    {
        let state = state.clone();
        let game = game.clone();
        let update_row_state = update_row_state.clone();
        apply_btn.connect_clicked(move |btn| {
            let proceed: Rc<dyn Fn()> = {
                let state = state.clone();
                let game = game.clone();
                let update_row_state = update_row_state.clone();
                let btn = btn.clone();
                Rc::new(move || {
                    let state = state.clone();
                    let game = game.clone();
                    let update_row_state = update_row_state.clone();
                    let btn = btn.clone();
                    btn.set_sensitive(false);
                    glib::spawn_future_local(async move {
                        let injector = state.injector.clone();
                        let game_t = game.clone();
                        let res = tokio::task::spawn_blocking(move || injector.backup_and_deploy(&game_t)).await;
                        let trans = state.trans();
                        if state.report_outcome(res, trans.success) {
                            update_row_state(true);
                        } else {
                            update_row_state(false);
                        }
                    });
                })
            };

            if game.is_online_multiplayer || game.has_anticheat {
                let trans = state.trans();
                let dialog = MessageDialog::builder().transient_for(&state.window).heading(trans.confirm_title).body(trans.confirm_body).build();
                dialog.add_response("cancel", trans.confirm_no);
                dialog.add_response("proceed", trans.confirm_yes);
                dialog.set_response_appearance("proceed", ResponseAppearance::Destructive);
                dialog.set_default_response(Some("cancel"));
                dialog.set_close_response("cancel");
                dialog.connect_response(None, move |d, res| { d.close(); if res == "proceed" { proceed(); } });
                dialog.present();
            } else {
                proceed();
            }
        });
    }

    {
        let state = state.clone();
        let game = game.clone();
        let update_row_state = update_row_state.clone();
        restore_btn.connect_clicked(move |btn| {
            let state = state.clone();
            let game = game.clone();
            let update_row_state = update_row_state.clone();
            let btn = btn.clone();
            btn.set_sensitive(false);
            glib::spawn_future_local(async move {
                let injector = state.injector.clone();
                let game_t = game.clone();
                let res = tokio::task::spawn_blocking(move || injector.restore_original(&game_t)).await;
                let trans = state.trans();
                if state.report_outcome(res, trans.restored) {
                    update_row_state(false);
                } else {
                    update_row_state(true);
                }
            });
        });
    }

    row
}
