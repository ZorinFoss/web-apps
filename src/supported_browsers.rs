use crate::browser::{Browser, BrowserType};

#[allow(dead_code)]
pub fn native_browsers() -> Vec<Browser> {
    vec![
        Browser::new(
            BrowserType::Firefox,
            "Firefox",
            "firefox",
            "/usr/bin/firefox",
            ".local/share/quick-webapps/firefox",
        ),
        Browser::new(
            BrowserType::Firefox,
            "Firefox Developer Edition",
            "firefox-developer-edition",
            "/usr/bin/firefox-developer-edition",
            ".local/share/quick-webapps/firefox",
        ),
        Browser::new(
            BrowserType::Firefox,
            "Firefox Nightly",
            "firefox-nightly",
            "/usr/bin/firefox-nightly",
            ".local/share/quick-webapps/firefox",
        ),
        Browser::new(
            BrowserType::Firefox,
            "Firefox ESR",
            "firefox-esr",
            "/usr/bin/firefox-esr",
            ".local/share/quick-webapps/firefox",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Brave Browser",
            "brave-browser",
            "/usr/bin/brave-browser",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Brave (bin)",
            "brave-bin",
            "/usr/bin/brave-bin",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Chrome",
            "google-chrome-stable",
            "/usr/bin/google-chrome-stable",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Chrome Beta",
            "google-chrome-beta",
            "/usr/bin/google-chrome-beta",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Chromium",
            "chromium",
            "/usr/bin/chromium",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Chromium Browser",
            "chromium-browser",
            "/usr/bin/chromium-browser",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Chromium (bin)",
            "chromium-bin",
            "/usr/bin/chromium-bin-browser",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Cromite",
            "cromite",
            "/usr/bin/cromite",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Thorium",
            "thorium-browser",
            "/usr/bin/thorium-browser",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Firefox,
            "Librewolf",
            "librewolf",
            "/usr/bin/librewolf",
            ".local/share/quick-webapps/firefox",
        ),
        Browser::new(
            BrowserType::Firefox,
            "Waterfox",
            "waterfox",
            "/usr/bin/waterfox",
            ".local/share/quick-webapps/firefox",
        ),
        Browser::new(
            BrowserType::Firefox,
            "Waterfox (current)",
            "waterfox-current",
            "/usr/bin/waterfox-current",
            ".local/share/quick-webapps/firefox",
        ),
        Browser::new(
            BrowserType::Firefox,
            "Waterfox (classic)",
            "waterfox-classic",
            "/usr/bin/waterfox-classic",
            ".local/share/quick-webapps/firefox",
        ),
        Browser::new(
            BrowserType::Firefox,
            "Waterfox 3rd Generation",
            "waterfox-g3",
            "/usr/bin/waterfox-g3",
            ".local/share/quick-webapps/firefox",
        ),
        Browser::new(
            BrowserType::Firefox,
            "Waterfox 4rd Generation",
            "waterfox-g4",
            "/usr/bin/waterfox-g4",
            ".local/share/quick-webapps/firefox",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Vivaldi",
            "vivaldi-stable",
            "/usr/bin/vivaldi-stable",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Vivaldi Snapshot",
            "vivaldi-snapshot",
            "/usr/bin/vivaldi-snapshot",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Microsoft Edge",
            "microsoft-edge-stable",
            "/usr/bin/microsoft-edge-stable",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Microsoft Edge Beta",
            "microsoft-edge-beta",
            "/usr/bin/microsoft-edge-beta",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Microsoft Edge Dev",
            "microsoft-edge-dev",
            "/usr/bin/microsoft-edge-dev",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            "FlashPeak Slimjet",
            "flashpeak-slimjet",
            "/usr/bin/flashpeak-slimjet",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Yandex",
            "yandex-browser",
            "/usr/bin/yandex-browser",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Naver Whale",
            "naver-whale-stable",
            "/usr/bin/naver-whale-stable",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Brave",
            "brave",
            "/usr/bin/brave",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Falkon,
            "Falkon",
            "falkon",
            "/usr/bin/falkon",
            ".local/share/quick-webapps/falkon",
        ),
    ]
}

pub fn flatpak_browsers() -> Vec<Browser> {
    vec![
        Browser::new(
            BrowserType::FirefoxFlatpak,
            "Firefox",
            "/var/lib/flatpak/exports/bin/org.mozilla.firefox",
            "/var/lib/flatpak/exports/bin/org.mozilla.firefox",
            ".var/app/org.mozilla.firefox/data/profiles",
        ),
        Browser::new(
            BrowserType::FirefoxFlatpak,
            "Firefox",
            ".local/share/flatpak/exports/bin/org.mozilla.firefox",
            ".local/share/flatpak/exports/bin/org.mozilla.firefox",
            ".var/app/org.mozilla.firefox/data/profiles",
        ),
        Browser::new(
            BrowserType::ChromiumFlatpak,
            "Chrome",
            "/var/lib/flatpak/exports/bin/com.google.Chrome",
            "/var/lib/flatpak/exports/bin/com.google.Chrome",
            ".var/app/com.google.Chrome/data/profiles",
        ),
        Browser::new(
            BrowserType::ChromiumFlatpak,
            "Chrome",
            ".local/share/flatpak/exports/bin/com.google.Chrome",
            ".local/share/flatpak/exports/bin/com.google.Chrome",
            ".var/app/com.google.Chrome/data/profiles",
        ),
        Browser::new(
            BrowserType::FirefoxFlatpak,
            "Librewolf",
            "/var/lib/flatpak/exports/bin/io.gitlab.librewolf-community",
            "/var/lib/flatpak/exports/bin/io.gitlab.librewolf-community",
            ".var/app/io.gitlab.librewolf-community/data/profiles",
        ),
        Browser::new(
            BrowserType::FirefoxFlatpak,
            "Librewolf",
            ".local/share/flatpak/exports/bin/io.gitlab.librewolf-community",
            ".local/share/flatpak/exports/bin/io.gitlab.librewolf-community",
            ".var/app/io.gitlab.librewolf-community/data/profiles",
        ),
        Browser::new(
            BrowserType::FirefoxFlatpak,
            "Waterfox",
            "/var/lib/flatpak/exports/bin/net.waterfox.waterfox",
            "/var/lib/flatpak/exports/bin/net.waterfox.waterfox",
            ".var/app/net.waterfox.waterfox/data/profiles",
        ),
        Browser::new(
            BrowserType::FirefoxFlatpak,
            "Waterfox",
            ".local/share/flatpak/exports/bin/net.waterfox.waterfox",
            ".local/share/flatpak/exports/bin/net.waterfox.waterfox",
            ".var/app/net.waterfox.waterfox/data/profiles",
        ),
        Browser::new(
            BrowserType::ChromiumFlatpak,
            "Vivaldi",
            "/var/lib/flatpak/exports/bin/com.vivaldi.Vivaldi",
            "/var/lib/flatpak/exports/bin/com.vivaldi.Vivaldi",
            ".var/app/com.vivaldi.Vivaldi/data/profiles",
        ),
        Browser::new(
            BrowserType::ChromiumFlatpak,
            "Vivaldi",
            ".local/share/flatpak/exports/bin/com.vivaldi.Vivaldi",
            ".local/share/flatpak/exports/bin/com.vivaldi.Vivaldi",
            ".var/app/com.vivaldi.Vivaldi/data/profiles",
        ),
        Browser::new(
            BrowserType::ChromiumFlatpak,
            "Ungoogled Chromium",
            "/var/lib/flatpak/exports/bin/io.github.ungoogled_software.ungoogled_chromium",
            "/var/lib/flatpak/exports/bin/io.github.ungoogled_software.ungoogled_chromium",
            ".var/app/io.github.ungoogled_software.ungoogled_chromium/data/profiles",
        ),
        Browser::new(
            BrowserType::ChromiumFlatpak,
            "Ungoogled Chromium",
            ".local/share/flatpak/exports/bin/io.github.ungoogled_software.ungoogled_chromium",
            ".local/share/flatpak/exports/bin/io.github.ungoogled_software.ungoogled_chromium",
            ".var/app/io.github.ungoogled_software.ungoogled_chromium/data/profiles",
        ),
        Browser::new(
            BrowserType::ChromiumFlatpak,
            "Chromium",
            "/var/lib/flatpak/exports/bin/org.chromium.Chromium",
            "/var/lib/flatpak/exports/bin/org.chromium.Chromium",
            ".var/app/org.chromium.Chromium/data/profiles",
        ),
        Browser::new(
            BrowserType::ChromiumFlatpak,
            "Chromium",
            ".local/share/flatpak/exports/bin/org.chromium.Chromium",
            ".local/share/flatpak/exports/bin/org.chromium.Chromium",
            ".var/app/org.chromium.Chromium/data/profiles",
        ),
        Browser::new(
            BrowserType::ChromiumFlatpak,
            "Microsoft Edge",
            "/var/lib/flatpak/exports/bin/com.microsoft.Edge",
            "/var/lib/flatpak/exports/bin/com.microsoft.Edge",
            ".var/app/com.microsoft.Edge/data/profiles",
        ),
        Browser::new(
            BrowserType::ChromiumFlatpak,
            "Microsoft Edge",
            ".local/share/flatpak/exports/bin/com.microsoft.Edge",
            ".local/share/flatpak/exports/bin/com.microsoft.Edge",
            ".var/app/com.microsoft.Edge/data/profiles",
        ),
        Browser::new(
            BrowserType::ChromiumFlatpak,
            "Brave",
            "/var/lib/flatpak/exports/bin/com.brave.Browser",
            "/var/lib/flatpak/exports/bin/com.brave.Browser",
            ".var/app/com.brave.Browser/data/profiles",
        ),
        Browser::new(
            BrowserType::ChromiumFlatpak,
            "Brave",
            ".local/share/flatpak/exports/bin/com.brave.Browser",
            ".local/share/flatpak/exports/bin/com.brave.Browser",
            ".var/app/com.brave.Browser/data/profiles",
        ),
        Browser::new(
            BrowserType::FalkonFlatpak,
            "Falkon",
            "/var/lib/flatpak/exports/bin/org.kde.falkon",
            "/var/lib/flatpak/exports/bin/org.kde.falkon",
            ".var/app/org.kde.falkon/data/profiles",
        ),
        Browser::new(
            BrowserType::FalkonFlatpak,
            "Falkon",
            ".local/share/flatpak/exports/bin/org.kde.falkon",
            ".local/share/flatpak/exports/bin/org.kde.falkon",
            ".var/app/org.kde.falkon/data/profiles",
        ),
        Browser::new(
            BrowserType::ChromiumFlatpak,
            "Yandex",
            "/var/lib/flatpak/exports/bin/ru.yandex.Browser",
            "/var/lib/flatpak/exports/bin/ru.yandex.Browser",
            ".var/app/ru.yandex.Browser/data/profiles",
        ),
        Browser::new(
            BrowserType::ChromiumFlatpak,
            "Yandex",
            ".local/share/flatpak/exports/bin/ru.yandex.Browser",
            ".local/share/flatpak/exports/bin/ru.yandex.Browser",
            ".var/app/ru.yandex.Browser/data/profiles",
        ),
        Browser::new(
            BrowserType::FirefoxFlatpak,
            "Floorp",
            "/var/lib/flatpak/exports/bin/one.ablaze.floorp",
            "/var/lib/flatpak/exports/bin/one.ablaze.floorp",
            ".var/app/one.ablaze.floorp/data/profiles",
        ),
        Browser::new(
            BrowserType::FirefoxFlatpak,
            "Floorp",
            ".local/share/flatpak/exports/bin/one.ablaze.floorp",
            ".local/share/flatpak/exports/bin/one.ablaze.floorp",
            ".var/app/one.ablaze.floorp/data/profiles",
        ),
        Browser::new(
            BrowserType::ZenFlatpak,
            "Zen Browser",
            "/var/lib/flatpak/exports/bin/io.github.zen_browser.zen",
            "/var/lib/flatpak/exports/bin/io.github.zen_browser.zen",
            ".var/app/io.github.zen_browser.zen/data/profiles",
        ),
        Browser::new(
            BrowserType::ZenFlatpak,
            "Zen Browser",
            ".local/share/flatpak/exports/bin/io.github.zen_browser.zen",
            ".local/share/flatpak/exports/bin/io.github.zen_browser.zen",
            ".var/app/io.github.zen_browser.zen/data/profiles",
        ),
    ]
}

pub fn nix_browsers() -> Vec<Browser> {
    vec![
        Browser::new(
            BrowserType::Firefox,
            "Firefox",
            "firefox",
            "/run/current-system/sw/bin/firefox",
            ".local/share/quick-webapps/firefox",
        ),
        Browser::new(
            BrowserType::Firefox,
            "Floorp",
            "floorp",
            "/run/current-system/sw/bin/floorp",
            ".local/share/quick-webapps/firefox",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Brave",
            "brave",
            "/run/current-system/sw/bin/brave",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Chrome",
            "google-chrome-stable",
            "/run/current-system/sw/bin/google-chrome-stable",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Chromium",
            "chromium",
            "/run/current-system/sw/bin/chromium",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Firefox,
            "Librewolf",
            "librewolf",
            "/run/current-system/sw/bin/librewolf",
            ".local/share/quick-webapps/firefox",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Vivaldi",
            "vivaldi-stable",
            "/run/current-system/sw/bin/vivaldi",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Microsoft Edge",
            "microsoft-edge-stable",
            "/run/current-system/sw/bin/microsoft-edge-stable",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Microsoft Edge Beta",
            "microsoft-edge-beta",
            "/run/current-system/sw/bin/microsoft-edge-beta",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Microsoft Edge Dev",
            "microsoft-edge-dev",
            "/run/current-system/sw/bin/microsoft-edge-dev",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Yandex",
            "yandex-browser",
            "/run/current-system/sw/bin/yandex-browser",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Falkon,
            "Falkon",
            "falkon",
            "/run/current-system/sw/bin/falkon",
            ".local/share/quick-webapps/falkon",
        ),
    ]
}
