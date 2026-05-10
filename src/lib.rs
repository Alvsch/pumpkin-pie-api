pub mod command {
    pub use pumpkin_plugin_api::{
        command::{Command, CommandError, CommandNode, CommandSender, ConsumedArgs},
        commands::CommandHandler,
    };
    // pub use pumpkin_plugin_api::server::CommandSender;
}
#[cfg(feature = "config")]
pub mod config;
pub mod event {
    use std::marker::PhantomData;

    pub use pumpkin_plugin_api::events::{
        BoxFuture, Event, EventData, EventHandler, EventPriority, FromIntoEvent,
    };
    pub mod events {
        pub use pumpkin_plugin_api::events::{
            BlockBreakEvent, BlockBurnEvent, BlockCanBuildEvent, BlockGrowEvent, BlockPlaceEvent,
            BlockRedstoneEvent, PlayerChangeWorldEvent, PlayerChangedMainHandEvent,
            PlayerChatEvent, PlayerCommandSendEvent, PlayerCustomPayloadEvent, PlayerEggThrowEvent,
            PlayerExpChangeEvent, PlayerFishEvent, PlayerGamemodeChangeEvent, PlayerInteractEvent,
            PlayerInteractUnknownEntityEvent, PlayerItemHeldEvent, PlayerJoinEvent,
            PlayerLeaveEvent, PlayerLoginEvent, PlayerMoveEvent, PlayerPermissionCheckEvent,
            PlayerTeleportEvent, PlayerToggleFlightEvent, PlayerToggleSneakEvent,
            PlayerToggleSprintEvent, ServerBroadcastEvent, ServerCommandEvent, SpawnChangeEvent,
        };
    }

    pub struct HandlerImpl<E, T>(T, PhantomData<fn() -> E>)
    where
        E: FromIntoEvent,
        T: Fn(crate::Server, E::Data) -> E::Data;

    impl<E, T> HandlerImpl<E, T>
    where
        E: FromIntoEvent,
        T: Fn(crate::Server, E::Data) -> E::Data,
    {
        pub fn new(f: T) -> Self {
            Self(f, PhantomData)
        }
    }

    impl<E, T> EventHandler<E> for HandlerImpl<E, T>
    where
        E: FromIntoEvent,
        T: Fn(crate::Server, E::Data) -> E::Data,
    {
        fn handle(&self, server: crate::Server, event: E::Data) -> E::Data {
            (self.0)(server, event)
        }
    }
}
pub mod plugin;
pub mod prelude {
    pub use crate::{PumpkinResult, scheduler::SchedulerExt};
}
pub mod player {
    pub use pumpkin_plugin_api::{
        common::{GameMode, Hand},
        player::{Player, PlayerAbilities, PlayerSkin, SkinParts},
    };
}
pub mod inventory {
    pub use pumpkin_plugin_api::{
        common::{ClickType, ItemStack},
        gui::{Gui, GuiType},
    };
}
pub mod text {
    pub use pumpkin_plugin_api::{
        common::{ArgbColor, NamedColor, RgbColor},
        text::TextComponent,
    };
}
pub mod block_entity {
    pub use pumpkin_plugin_api::block_entity::{BlockEntity, CommandBlockEntity};
}
pub mod block {
    // NOTE: we got BlockPosition and BlockPos
    pub use pumpkin_plugin_api::{
        common::BlockPosition,
        world::{BlockDirection, BlockFlags, BlockPos, BlockState},
    };
}
pub mod entity {
    pub use pumpkin_plugin_api::{common::EntityPose, entity_types::EntityType, world::Entity};
}
pub mod permission {
    pub use pumpkin_plugin_api::permission::{
        Permission, PermissionChild, PermissionDefault, PermissionLevel,
    };

    pub fn permission(
        node: impl Into<String>,
        description: impl Into<String>,
        default: PermissionDefault,
        children: impl Into<Vec<PermissionChild>>,
    ) -> Permission {
        Permission {
            node: node.into(),
            description: description.into(),
            default,
            children: children.into(),
        }
    }

    pub fn permission_child(node: impl Into<String>, value: bool) -> PermissionChild {
        PermissionChild {
            node: node.into(),
            value,
        }
    }
}
pub mod sound {
    pub use pumpkin_plugin_api::world::{Sound, SoundCategory};
}
pub mod boss_bar {
    pub use pumpkin_plugin_api::boss_bar::{
        BossBar, BossBarColor, BossBarDivision, BossBarMetadata,
    };
}
pub mod i18n {
    pub use pumpkin_plugin_api::{
        common::Locale,
        i18n::{load_translations, translate},
    };
}
pub mod particle {
    pub use pumpkin_plugin_api::particles::Particle;
}
pub mod scoreboard {
    pub use pumpkin_plugin_api::scoreboard::{
        CollisionRule, DisplaySlot, NametagVisibility, RenderType, Scoreboard, TeamSettings,
    };
}
pub mod packet {
    pub use pumpkin_plugin_api::bedrock_packets;
    pub use pumpkin_plugin_api::java_packets;
}
pub mod server {
    pub use pumpkin_plugin_api::server::{Difficulty, SysInfo};
}
pub mod world {
    pub use pumpkin_plugin_api::{server::Dimension, world::World};
}

pub use pumpkin_plugin_api::{
    Context, Plugin, PluginMetadata, Result as PumpkinResult, register_plugin, scheduler,
    server::Server,
};
