extern crate lazy_static;
extern crate rltk;
extern crate serde;
extern crate specs;
#[macro_use]
extern crate specs_derive;

use rltk::{Console, GameState, Point, Rltk};\
use specs::prelude::*;
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator};

pub use hxgm30client::components::*;
pub use hxgm30client::map::*;
pub use hxgm30client::player::*;
pub use hxgm30client::rect::Rect;
pub use hxgm30client::gamesystem::*;

const SHOW_MAPGEN_VISUALIZER: bool = false;
const SHOW_FPS: bool = true;

fn main() {
    use rltk::RltkBuilder;
    let mut context = RltkBuilder::simple(80, 60)
        .with_title("Roguelike Tutorial")
        .with_font("vga8x16.png", 8, 16)
        .with_sparse_console(80, 30, "vga8x16.png")
        .with_vsync(false)
        .build();
    context.with_post_scanlines(true);
    let mut gs = State {
        ecs: World::new(),
        mapgen_next_state: Some(RunState::MainMenu {
            menu_selection: gui::MainMenuSelection::NewGame,
        }),
        mapgen_index: 0,
        mapgen_history: Vec::new(),
        mapgen_timer: 0.0,
        dispatcher: systems::build(),
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<Item>();
    gs.ecs.register::<ProvidesHealing>();
    gs.ecs.register::<InflictsDamage>();
    gs.ecs.register::<AreaOfEffect>();
    gs.ecs.register::<Consumable>();
    gs.ecs.register::<Ranged>();
    gs.ecs.register::<InBackpack>();
    gs.ecs.register::<WantsToPickupItem>();
    gs.ecs.register::<WantsToUseItem>();
    gs.ecs.register::<WantsToDropItem>();
    gs.ecs.register::<Confusion>();
    gs.ecs.register::<SimpleMarker<SerializeMe>>();
    gs.ecs.register::<SerializationHelper>();
    gs.ecs.register::<DMSerializationHelper>();
    gs.ecs.register::<Equippable>();
    gs.ecs.register::<Equipped>();
    gs.ecs.register::<Weapon>();
    gs.ecs.register::<Wearable>();
    gs.ecs.register::<WantsToRemoveItem>();
    gs.ecs.register::<ParticleLifetime>();
    gs.ecs.register::<HungerClock>();
    gs.ecs.register::<ProvidesFood>();
    gs.ecs.register::<MagicMapper>();
    gs.ecs.register::<Hidden>();
    gs.ecs.register::<EntryTrigger>();
    gs.ecs.register::<EntityMoved>();
    gs.ecs.register::<SingleActivation>();
    gs.ecs.register::<BlocksVisibility>();
    gs.ecs.register::<Door>();
    gs.ecs.register::<Quips>();
    gs.ecs.register::<Attributes>();
    gs.ecs.register::<Skills>();
    gs.ecs.register::<Pools>();
    gs.ecs.register::<NaturalAttackDefense>();
    gs.ecs.register::<LootTable>();
    gs.ecs.register::<OtherLevelPosition>();
    gs.ecs.register::<LightSource>();
    gs.ecs.register::<Initiative>();
    gs.ecs.register::<MyTurn>();
    gs.ecs.register::<Faction>();
    gs.ecs.register::<WantsToApproach>();
    gs.ecs.register::<WantsToFlee>();
    gs.ecs.register::<MoveMode>();
    gs.ecs.register::<Chasing>();
    gs.ecs.register::<EquipmentChanged>();
    gs.ecs.register::<Vendor>();
    gs.ecs.register::<TownPortal>();
    gs.ecs.register::<TeleportTo>();
    gs.ecs.register::<ApplyMove>();
    gs.ecs.register::<ApplyTeleport>();
    gs.ecs.register::<MagicItem>();
    gs.ecs.register::<ObfuscatedName>();
    gs.ecs.register::<IdentifiedItem>();
    gs.ecs.register::<SpawnParticleBurst>();
    gs.ecs.register::<SpawnParticleLine>();
    gs.ecs.register::<CursedItem>();
    gs.ecs.register::<ProvidesRemoveCurse>();
    gs.ecs.register::<ProvidesIdentification>();
    gs.ecs.register::<AttributeBonus>();
    gs.ecs.register::<Duration>();
    gs.ecs.register::<StatusEffect>();
    gs.ecs.register::<KnownSpells>();
    gs.ecs.register::<SpellTemplate>();
    gs.ecs.register::<WantsToCastSpell>();
    gs.ecs.register::<TeachesSpell>();
    gs.ecs.register::<ProvidesMana>();
    gs.ecs.register::<Slow>();
    gs.ecs.register::<DamageOverTime>();
    gs.ecs.register::<SpecialAbilities>();
    gs.ecs.register::<TileSize>();
    gs.ecs.register::<OnDeath>();
    gs.ecs.register::<AlwaysTargetsSelf>();
    gs.ecs.register::<Target>();
    gs.ecs.register::<WantsToShoot>();
    gs.ecs.insert(SimpleMarkerAllocator::<SerializeMe>::new());

    raws::load_raws();

    gs.ecs.insert(map::MasterDungeonMap::new());
    gs.ecs.insert(Map::new(1, 64, 64, "New Map"));
    gs.ecs.insert(Point::new(0, 0));
    let player_entity = spawner::player(&mut gs.ecs, 0, 0);
    gs.ecs.insert(player_entity);
    gs.ecs.insert(RunState::MapGeneration {});
    gs.ecs
        .insert(systems::particle_system::ParticleBuilder::new());
    gs.ecs.insert(rex_assets::RexAssets::new());

    gs.generate_world_map(1, 0);

    rltk::main_loop(context, gs);
}
