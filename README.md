# VaporDose (Steam DLC Automation & Manager for Linux & Steam Deck)

[![GitHub Release](https://img.shields.io/github/v/release/vlapochkin/vapordose?style=for-the-badge&color=blue)](https://github.com/vlapochkin/vapordose/releases)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20SteamOS%20%7C%20Steam%20Deck-informational?style=for-the-badge&logo=steamos)](https://store.steampowered.com/steamos)
[![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)](LICENSE)
[![Built With](https://img.shields.io/badge/Built%20With-Rust%20%26%20GTK4%2FLibadwaita-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)

[English](#-english) | [Русский](#-русский)

---

## 🌐 English

**VaporDose** is a modern, high-performance GTK4 / Libadwaita GUI application for **Linux** and **Steam Deck (SteamOS)** designed to automatically manage, configure, and deploy **SmokeAPI** and **CreamAPI** DLC wrappers for Steam games.

It scans your Steam installations (internal SSDs, Flatpak, MicroSD cards, external drives), detects game architecture (Native Linux / Proton), identifies anti-cheat protections, fetches official DLC lists directly from Steam Store API, and deploys DLC unlocker proxies safely.

---

### ✨ Features in v0.5.x

- 🚀 **Steam Launch Options Auto-Injector**: Automatically injects and cleans up `WINEDLLOVERRIDES="steam_api64=n,b;steam_api=n,b"` in Steam `localconfig.vdf` when patching/restoring Proton games!
- 🔄 **SmokeAPI Binary Manager & Auto-Updater**: Check for SmokeAPI GitHub updates directly from the app menu and install fresh proxy binaries with 1 click.
- 🎮 **Steam Deck & SteamOS Ready**: Touchscreen friendly, compact 1280x800 resolution support, full **D-Pad / Gamepad navigation** support.
- 🎴 **MicroSD & Drive Auto-Discovery**: Automatically finds games installed on external mounts (`/run/media/`).
- 🌐 **Steam DLC Auto-Fetcher**: Fetch official DLC IDs and names directly from the Steam Store API with 1-click.
- 🖼️ **Game Icons & Artwork**: Displays high-resolution game icons and artwork cached from Steam library.
- 🛡️ **Anti-Cheat Safety Detector**: Scans game folders for Easy Anti-Cheat (EAC), BattlEye, Vanguard, Ricochet, Denuvo, and warns against multiplayer risks.
- 🐧 **Native & Proton Support**: Seamlessly patches `libsteam_api.so` (32/64-bit) for Native games and `steam_api.dll`/`steam_api64.dll` for Proton/Wine.
- 📋 **Batch Operations & Safety Dialogs**: Patch or restore all safe games simultaneously with confirmation popups.
- 📂 **Collapsible System Components**: Keeps system tools (Proton, Steam Linux Runtime, SDKs) collapsed and out of visual clutter.

---

### 🚀 Installation Guide

#### Option 1: AppImage (Recommended for Steam Deck & All Linux Distros)

1. Download `VaporDose-x86_64.AppImage` from the **[Releases](https://github.com/vlapochkin/vapordose/releases)** page.
2. Open terminal in your downloads folder and make it executable:
   ```bash
   chmod +x VaporDose-x86_64.AppImage
   ```
3. Run the AppImage:
   ```bash
   ./VaporDose-x86_64.AppImage
   ```

#### Adding VaporDose to Steam Deck (Game Mode)
1. Switch to **Desktop Mode** on your Steam Deck.
2. Download `VaporDose-x86_64.AppImage` to your Home directory or Downloads.
3. Open **Steam** in Desktop Mode -> Click **Add a Game** -> **Add a Non-Steam Game...**
4. Select `VaporDose-x86_64.AppImage`.
5. Switch back to **Gaming Mode**! Now you can manage DLCs directly using your D-Pad and touch screen.

---

### 🛠️ Building from Source

Requirements: `rustc`, `cargo`, `libgtk-4-dev`, `libadwaita-1-dev`.

```bash
# Clone repository
git clone https://github.com/vlapochkin/vapordose.git
cd vapordose

# Build and run
cargo run --release
```

---

### ❤️ Credits & Special Thanks

VaporDose is a frontend management interface. We are immensely grateful to the developers and open-source projects that make Steam DLC unlocking on Linux possible:

- **[SmokeAPI](https://github.com/acidicoala/SmokeAPI)** by **[acidicoala](https://github.com/acidicoala)** — The powerful open-source Steam API proxy wrapper for Linux & Windows.
- **[CreamAPI](https://github.com/deadbeef-dev/CreamAPI)** by **DeadBeef** & **CS.RIN.RU** community — The pioneer Steam DLC unlocker.
- **[Koalageddon](https://github.com/acidicoala/Koalageddon)** by **acidicoala** — Advanced legitimate DLC unlocker.
- **[GNOME / Libadwaita](https://gitlab.gnome.org/GNOME/libadwaita)** — Modern GTK4 user interface framework.
- **[ProtonDB](https://www.protondb.com)** & **[SteamDB](https://steamdb.info)** — Invaluable Steam compatibility databases.

---

### ⚠️ Disclaimer

*This project is an open-source automation utility provided for educational and personal use only. The authors are not affiliated with Valve, Steam, or any game developer. Use at your own risk.*

---

## 🌐 Русский

**VaporDose** — это современное приложение на GTK4 / Libadwaita для **Linux** и **Steam Deck (SteamOS)**, предназначенное для автоматического управления, настройки и внедрения патчей **SmokeAPI** и **CreamAPI** в играх Steam.

Приложение автоматически сканирует ваши библиотеки Steam (внутренний накопитель, Flatpak, карту памяти MicroSD, внешние диски), определяет архитектуру игры (Native Linux / Proton), выявляет наличие античита, загружает официальный список DLC напрямую из Steam Store API и безопасно настраивает прокси-файлы.

---

### ✨ Новые возможности v0.5.x

- 🚀 **Авто-инъекция параметров запуска Steam**: Автоматическое внесение и чистка `WINEDLLOVERRIDES="steam_api64=n,b;steam_api=n,b"` в файлах `localconfig.vdf` при патчинге/восстановлении Proton-игр!
- 🔄 **Менеджер и авто-обновление SmokeAPI**: Онлайн-проверка обновлений SmokeAPI с GitHub и скачивание актуальных бинарников прямо из меню приложения в 1 клик.
- 🎮 **Оптимизация для Steam Deck**: Поддержка разрешения 1280x800, крупный интерфейс для тачскрина и полная **навигация с геймпада / D-Pad**.
- 🎴 **Авто-поиск на MicroSD картах**: Автоматическое обнаружение игр на внешних дисках и флешках (`/run/media/`).
- 🌐 **Авто-загрузка DLC из Steam**: Импорт названий и ID всех официальных дополнений из Steam Store API в 1 клик.
- 🖼️ **Иконки и обложки игр**: Отображение оригинальных иконок игр из кэша библиотек Steam.
- 🛡️ **Детектор античита**: Анализ библиотек Easy Anti-Cheat (EAC), BattlEye, Vanguard, Ricochet, Denuvo для защиты от банов в онлайн-играх.
- 🐧 **Поддержка Native и Proton**: Патчинг `libsteam_api.so` (32/64-бит) для нативных игр Linux и `steam_api.dll`/`steam_api64.dll` для игр через Proton/Wine.
- 📋 **Пакетные операции с диалогами защиты**: Кнопки «Патчить всё» и «Восстановить всё» с подтверждающими диалоговыми окнами.
- 📂 **Сворачивание системных библиотек**: Системные компоненты (Proton, SDK, Steam Linux Runtime) убраны в отдельный свёрнутый блок.

---

### 🚀 Руководство по установке

#### Способ 1: AppImage (Рекомендуется для Steam Deck и всех Linux дистрибутивов)

1. Скачайте файл `VaporDose-x86_64.AppImage` со страницы **[Releases](https://github.com/vlapochkin/vapordose/releases)**.
2. Откройте терминал в папке с файлом и сделайте его исполняемым:
   ```bash
   chmod +x VaporDose-x86_64.AppImage
   ```
3. Запустите приложение:
   ```bash
   ./VaporDose-x86_64.AppImage
   ```

#### Добавление VaporDose в Game Mode на Steam Deck
1. Перейдите в **Режим рабочего стола (Desktop Mode)** на Steam Deck.
2. Скачайте `VaporDose-x86_64.AppImage`.
3. Откройте **Steam** -> нажмите **Добавить игру** -> **Добавить стороннюю игру...**
4. Выберите `VaporDose-x86_64.AppImage`.
5. Вернитесь в **Игровой режим (Game Mode)**. Теперь приложением можно управлять прямо с геймпада Steam Deck!

---

### 🛠️ Сборка из исходного кода

Вам потребуются: `rustc`, `cargo`, `libgtk-4-dev`, `libadwaita-1-dev`.

```bash
# Клонирование репозитория
git clone https://github.com/vlapochkin/vapordose.git
cd vapordose

# Сборка и запуск
cargo run --release
```

---

### ❤️ Благодарности и авторы

VaporDose является графическим интерфейсом и средством автоматизации. Выражаем огромную благодарность разработчикам следующих проектов:

- **[SmokeAPI](https://github.com/acidicoala/SmokeAPI)** от **[acidicoala](https://github.com/acidicoala)** — Мощный открытый прокси-клиент Steam API для Linux и Windows.
- **[CreamAPI](https://github.com/deadbeef-dev/CreamAPI)** от **DeadBeef** и сообщества **CS.RIN.RU** — Легендарный разблокировщик DLC.
- **[Koalageddon](https://github.com/acidicoala/Koalageddon)** от **acidicoala** — Продвинутый инструмент разблокировки.
- **[GNOME / Libadwaita](https://gitlab.gnome.org/GNOME/libadwaita)** — Графический фреймворк GTK4.
- **[ProtonDB](https://www.protondb.com)** & **[SteamDB](https://steamdb.info)** — Информационные базы данных Steam.

---

### ⚠️ Отказ от ответственности

*Этот проект создается исключительно в образовательных целях. Авторы не связаны с компанией Valve или разработчиками игр. Используйте приложение на свой страх и риск.*
