// Datos detectados en el mod original. No esta conectado a ningun hook: usalo como referencia.
// Port de Sodium (Fabric) a Horizon. Generado por Jar2Horizon: punto de partida, no traduccion.
// TODO: declara los hooks en horizon-mixin.toml y mueve aqui la logica (docs de Horizon: Make a mod).

pub const MOD_ID: &str = "sodium";
pub const MOD_NAME: &str = "Sodium";

/// (id, nombre) detectados en los archivos de idioma/modelos del original.
pub const BLOCKS: &[(&str, &str)] = &[
];

/// (id, nombre) detectados en los archivos de idioma/modelos del original.
pub const ITEMS: &[(&str, &str)] = &[
];

/// (id, nombre) detectados en los archivos de idioma/modelos del original.
pub const ENTITIES: &[(&str, &str)] = &[
];

pub fn on_load() {
    // TODO [PARCIAL] Registro de bloques/items: Depende del soporte de contenido de la API.
    //   Java: net.caffeinemc.mods.sodium.client.render.model.ItemGlintRenderType
    //   Java: net.caffeinemc.mods.sodium.client.render.model.ItemGlintSpecialRenderType
    //   Java: net.caffeinemc.mods.sodium.client.render.model.ItemRenderType
    //   Java: net.caffeinemc.mods.sodium.client.world.biome.LevelBiomeSlice
    // TODO [PARCIAL] HUD y render: UI simple si; render propio depende de la API.
    //   Java: net.caffeinemc.mods.sodium.fabric.level.FabricLevelRenderHooks
    //   Java: net.caffeinemc.mods.sodium.fabric.render.FluidRendererImpl
    //   Java: net.caffeinemc.mods.sodium.client.gpu.arena.ArenaAggregator
    //   Java: net.caffeinemc.mods.sodium.client.gpu.arena.BufferArena
    // TODO [DIFICIL] Block entities / maquinas: Estado por bloque e inventarios (API experimental).
    //   Java: net.caffeinemc.mods.sodium.fabric.block.FabricBlockAccess
    //   Java: net.caffeinemc.mods.sodium.fabric.level.FabricLevelAccess
    //   Java: net.caffeinemc.mods.sodium.mixin.fabric.LevelSliceMixin
    //   Java: net.caffeinemc.mods.sodium.client.render.SodiumWorldRenderer
    // TODO [DIFICIL] Entidades y mobs: IA y sincronizacion de entidades.
    //   Java: net.caffeinemc.mods.sodium.mixin.features.render.world.sky.LevelRendererMixin
    // TODO [DIFICIL] Pantallas y menus (GUI): Interfaces de contenedor: reescritura completa.
    //   Java: net.caffeinemc.mods.sodium.client.config.builder.ExternalButtonOptionBuilderImpl
    //   Java: net.caffeinemc.mods.sodium.client.config.builder.ExternalPageBuilderImpl
    //   Java: net.caffeinemc.mods.sodium.client.config.structure.ExternalButtonOption
    //   Java: net.caffeinemc.mods.sodium.client.config.structure.ExternalPage
    // TODO [DIFICIL] Mixins / coremods: Parchean Java Minecraft, que en Horizon no existe.
    //   Java: net.caffeinemc.mods.sodium.mixin.core.model.quad.BakedQuadMixin
    //   Java: net.caffeinemc.mods.sodium.mixin.fabric.LevelSliceMixin
    //   Java: net.caffeinemc.mods.sodium.mixin.features.render.model.ItemModelGeneratorMixin
    //   Java: net.caffeinemc.mods.sodium.mixin.features.world.biome.BiomeMixin
}
