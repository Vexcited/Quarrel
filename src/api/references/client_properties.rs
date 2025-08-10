use serde::{Deserialize, Serialize, de::IntoDeserializer};

use crate::api::references::OperatingSystemType;

/// Due to the nature of client properties, the structure of this object is not
/// well-defined, and no field is truly required.
///
/// Additionally, types of fields are not verified, and all documented enums
/// are merely conventions. Fields are marked as required if it's observed that
/// they are sent in all official client properties.
///
/// https://github.com/discord-userdoccers/discord-userdoccers/blob/master/pages/reference.mdx#client-properties
#[derive(Serialize, Deserialize)]
pub struct ClientProperties {
  pub os: OperatingSystemType,

  /// The operating system version (kernel version for Linux, SDK version for Android)
  pub os_version: String,

  /// The operating system SDK version
  pub os_sdk_version: String,

  /// The architecture of the operating system.
  pub os_arch: String,

  /// The architecture of the desktop application.
  pub app_arch: String,

  /// The browser the client is using.
  pub browser: String,

  /// The `User-Agent` of Electron, should be blank on mobile clients.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub browser_user_agent: Option<String>,

  /// The version of Electron, should be blank on mobile clients.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub browser_version: Option<String>,

  /// The build number of the client.
  pub client_build_number: i64,

  /// The native metadata version of the desktop client.
  pub native_build_number: Option<i64>,

  /// Version of the client currently in use.
  pub client_version: String,

  /// The alternate event source this request originated from.
  pub client_event_source: Option<String>,

  /// The focus state of the client when the `Gateway` session was instantiated.
  pub client_app_state: String,

  /// A client-generated UUID used to identify the client launch
  #[serde(skip_serializing_if = "Option::is_none")]
  pub client_launch_id: Option<String>,

  /// A client-generated UUID representing the current persisted analytics
  /// heartbeat, regenerated every 30 minutes.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub client_heartbeat_session_id: Option<String>,

  /// The release channel of the client.
  pub release_channel: String,

  /// The primary system locale.
  pub system_locale: String,

  /// The model of the mobile device the client is running on.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub device: Option<String>,

  /// A unique identifier for the mobile device (UUID on Android, IdentifierForVendor on iOS)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub device_vendor_id: Option<String>,

  /// The Linux window manager ( env.XDG_CURRENT_DESKTOP ?? "unknown" + "," + env.GDMSESSION ?? "unknown" )
  #[serde(skip_serializing_if = "Option::is_none")]
  pub window_manager: Option<String>,

  /// The Linux distribution (output of lsb_release -ds )
  #[serde(skip_serializing_if = "Option::is_none")]
  pub distro: Option<String>,

  /// Whether the connecting client has modifications (e.g. BetterDiscord)
  pub has_client_mods: bool,

  /// The version of the client properties protocol.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub version: Option<String>,
}

const ELECTRON_VERSION: &str = "35.3.0";
const CHROME_VERSION: &str = "134.0.6998.205";

fn build_browser_user_agent(discord_version: String) -> String {
  #[cfg(windows)]
  let metadata = "Windows NT 10.0; Win64; x64";

  #[cfg(target_os = "macos")]
  let metadata = "Macintosh; Intel Mac OS X 10_15_7";

  // TODO: do the linux one?

  format!(
    "Mozilla/5.0 {metadata} AppleWebKit/537.36 (KHTML, like Gecko) discord/{discord_version} Chrome/{CHROME_VERSION} Electron/{ELECTRON_VERSION} Safari/537.36"
  )
}

impl ClientProperties {
  pub fn new() -> Self {
    Self {
      // app_arch: "arm64".into(),
      app_arch: "x64".into(),
      browser: "Discord Client".into(),
      // browser_user_agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) discord/0.0.356 Chrome/134.0.6998.205 Electron/35.3.0 Safari/537.36".into(),
      browser_user_agent: Some(build_browser_user_agent("1.0.9201".into())),
      browser_version: Some(ELECTRON_VERSION.into()),
      client_app_state: "focused".into(),
      // GET https://discord.com/app > HTML
      // lookup for "BUILD_NUMBER": "427496", in first <script>
      // fn extract_build_number(html: &str) -> Option<&str> {
      //   const PATTERN: &str = "\"BUILD_NUMBER\":\"";
      //
      //   let start_pos = html.find(PATTERN)? + PATTERN.len();
      //   let remaining = &html[start_pos..];
      //   let end_pos = remaining.find('"')?;
      //
      //   Some(&remaining[..end_pos])
      // }
      client_build_number: 427496,
      client_event_source: None,
      client_heartbeat_session_id: None,
      client_launch_id: None,
      // on macOS:
      // https://discord.com/api/download?platform=osx
      // https://stable.dl2.discordapp.net/apps/osx/0.0.356/Discord.dmg
      // get -1 path => 0.0.356
      // client_version: "0.0.356".into(),
      //
      // on linux:
      // https://discord.com/api/download?platform=linux&format=deb
      // https://stable.dl2.discordapp.net/apps/linux/0.0.104/discord-0.0.104.deb
      // get -1 path => 0.0.104
      // client_version: "0.0.104".into(),
      //
      // on windows:
      // https://discord.com/api/downloads/distributions/app/installers/latest?channel=stable&platform=win&arch=x64 | x86
      // ^ change ?platform and ?arch and intercept redirections
      // https://stable.dl2.discordapp.net/distro/app/stable/win/x64/1.0.9201/DiscordSetup.exe
      // get -1 path => 1.0.9201
      //
      client_version: "1.0.9201".into(),
      has_client_mods: false,
      // on macOS:
      // native_build_number: None, // keep it as `null`
      // on windows:
      // https://docs.discord.food/topics/client-distribution#get-latest-distributed-application-manifest
      // https://updates.discord.com/distributions/app/manifests/latest?channel=stable&platform=win&arch=x64&install_id=0&platform_version=10.0.26100
      // ^ #metadata_version
      native_build_number: Some(66940),
      os: OperatingSystemType::Windows,
      // os_arch: "arm64".into(),
      os_arch: "x64".into(),
      // on macOS:
      // os_sdk_version: "24.5.0".into(),
      os_sdk_version: "24".into(),
      // on macOS:
      // os_version: "24.5.0".into(),
      // on windows:
      os_version: "10.0.26100".into(),
      release_channel: "stable".into(),
      // system_locale: "fr".into(),
      // or could be also...
      system_locale: "en-US".into(),
      version: None,

      // We're not concerned about these properties, since they're mobile only.
      device: None,
      device_vendor_id: None,

      // TODO: add properties if we're on Linux.
      distro: None,
      window_manager: None,
    }
  }
}
