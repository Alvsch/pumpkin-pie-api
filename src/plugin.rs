use pumpkin_plugin_api::permissions;

#[macro_export]
macro_rules! plugin_metadata_from_cargo {
    (dependencies = $dependencies:expr, permissions = $permissions:expr $(,)?) => {{
        PluginMetadata {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            authors: env!("CARGO_PKG_AUTHORS")
                .split(':')
                .map(ToString::to_string)
                .collect(),
            description: env!("CARGO_PKG_DESCRIPTION").to_string(),
            dependencies: $dependencies.into_iter().map(Into::into).collect(),
            permissions: $permissions.into_iter().map(Into::into).collect(),
        }
    }};
    (permissions = $perms:expr, dependencies = $deps:expr $(,)?) => {
        $crate::plugin_metadata_from_cargo!(dependencies = $deps, permissions = $perms)
    };
    () => {
        $crate::plugin_metadata_from_cargo!(
            dependencies = Vec::<String>::new(),
            permissions = Vec::<$crate::PluginPermission>::new(),
        )
    };
    (dependencies = $dependencies:expr $(,)?) => {
        $crate::plugin_metadata_from_cargo!(
            dependencies = $dependencies,
            permissions = Vec::<$crate::PluginPermission>::new(),
        )
    };
    (permissions = $permissions:expr $(,)?) => {
        $crate::plugin_metadata_from_cargo!(
            dependencies = Vec::<String>::new(),
            permissions = $permissions,
        )
    };
}

pub enum PluginPermission {
    /// Allows the plugin to perform DNS resolution.
    NetworkDns,

    /// Allows the plugin to use TCP sockets.
    NetworkTcp,

    /// Allows the plugin to use UDP sockets.
    NetworkUdp,

    /// Allows the plugin to initiate TCP connections.
    NetworkTcpConnect,

    /// Allows the plugin to bind TCP listeners (accept inbound connections).
    NetworkTcpBind,

    /// Allows the plugin to send and receive UDP packets to specific destinations.
    NetworkUdpConnect,

    /// Allows the plugin to bind UDP sockets to local ports.
    NetworkUdpBind,

    /// Allows the plugin to send datagram on non-connected UDP socket.
    NetworkUdpOutgoingDatagram,

    /// Restricts all networking permissions to loopback addresses (localhost) only.
    NetworkLoopback,

    /// Allows the plugin to make outbound TCP/UDP connections.
    /// **Warning:** This is a powerful permission.
    NetworkOutbound,

    /// Allows the plugin to read files from the server's file system outside of its data folder.
    FsRead,

    /// Allows the plugin to write files to the server's file system outside of its data folder.
    FsWrite,

    /// Allows the plugin to read files within its own data folder (`plugins/<name>`).
    FsReadData,

    /// Allows the plugin to write files within its own data folder (`plugins/<name>`).
    FsWriteData,

    /// Allows the plugin to read all environment variables.
    SysEnv,

    /// Allows the plugin to read specific environment variables.
    /// Used with a prefix like "sys.env.PATH".
    SysEnvPrefix,

    /// Allows the plugin to read system information (CPU, Memory, OS).
    SysInfo,

    /// Allows the plugin to read CPU information.
    SysInfoCpu,

    /// Allows the plugin to read RAM information.
    SysInfoRam,

    /// Allows the plugin to read OS information.
    SysInfoOs,
}

impl From<PluginPermission> for String {
    fn from(value: PluginPermission) -> Self {
        match value {
            PluginPermission::NetworkDns => permissions::NETWORK_DNS,
            PluginPermission::NetworkTcp => permissions::NETWORK_TCP,
            PluginPermission::NetworkUdp => permissions::NETWORK_UDP,
            PluginPermission::NetworkTcpConnect => permissions::NETWORK_TCP_CONNECT,
            PluginPermission::NetworkTcpBind => permissions::NETWORK_TCP_BIND,
            PluginPermission::NetworkUdpConnect => permissions::NETWORK_UDP_CONNECT,
            PluginPermission::NetworkUdpBind => permissions::NETWORK_UDP_BIND,
            PluginPermission::NetworkUdpOutgoingDatagram => {
                permissions::NETWORK_UDP_OUTGOING_DATAGRAM
            }
            PluginPermission::NetworkLoopback => permissions::NETWORK_LOOPBACK,
            PluginPermission::NetworkOutbound => permissions::NETWORK_OUTBOUND,
            PluginPermission::FsRead => permissions::FS_READ,
            PluginPermission::FsWrite => permissions::FS_WRITE,
            PluginPermission::FsReadData => permissions::FS_READ_DATA,
            PluginPermission::FsWriteData => permissions::FS_WRITE_DATA,
            PluginPermission::SysEnv => permissions::SYS_ENV,
            PluginPermission::SysEnvPrefix => permissions::SYS_ENV_PREFIX,
            PluginPermission::SysInfo => permissions::SYS_INFO,
            PluginPermission::SysInfoCpu => permissions::SYS_INFO_CPU,
            PluginPermission::SysInfoRam => permissions::SYS_INFO_RAM,
            PluginPermission::SysInfoOs => permissions::SYS_INFO_OS,
        }
        .to_string()
    }
}
