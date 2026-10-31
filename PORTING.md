# Informe de portado: Sodium

## Resumen
```
Sodium  (sodium 0.9.2+mc26.3)
Cargador: Fabric
Clases Java: 794
Esfuerzo de portado estimado: ALTO (puntaje 155.7)
Clase(s) de entrada: no detectada (revisa fabric.mod.json / entrypoints)
Bloques: 0  Items: 0  Entidades: 0

```

## Codigo
```
[DIFICIL] Block entities / maquinas: 21 clases
          Estado por bloque e inventarios (API experimental).
          - net.caffeinemc.mods.sodium.fabric.block.FabricBlockAccess
          - net.caffeinemc.mods.sodium.fabric.level.FabricLevelAccess
          - net.caffeinemc.mods.sodium.mixin.fabric.LevelSliceMixin
          - net.caffeinemc.mods.sodium.client.render.SodiumWorldRenderer
          - net.caffeinemc.mods.sodium.client.render.chunk.BlockEntityRenderHandlerImpl

[DIFICIL] Entidades y mobs: 1 clases
          IA y sincronizacion de entidades.
          - net.caffeinemc.mods.sodium.mixin.features.render.world.sky.LevelRendererMixin

[DIFICIL] Pantallas y menus (GUI): 19 clases
          Interfaces de contenedor: reescritura completa.
          - net.caffeinemc.mods.sodium.client.config.builder.ExternalButtonOptionBuilderImpl
          - net.caffeinemc.mods.sodium.client.config.builder.ExternalPageBuilderImpl
          - net.caffeinemc.mods.sodium.client.config.structure.ExternalButtonOption
          - net.caffeinemc.mods.sodium.client.config.structure.ExternalPage
          - net.caffeinemc.mods.sodium.client.gui.VideoSettingsScreen

[DIFICIL] Mixins / coremods: 82 clases
          Parchean Java Minecraft, que en Horizon no existe.
          - net.caffeinemc.mods.sodium.mixin.core.model.quad.BakedQuadMixin
          - net.caffeinemc.mods.sodium.mixin.fabric.LevelSliceMixin
          - net.caffeinemc.mods.sodium.mixin.features.render.model.ItemModelGeneratorMixin
          - net.caffeinemc.mods.sodium.mixin.features.world.biome.BiomeMixin
          - net.caffeinemc.mods.sodium.mixin.SodiumMixinPlugin

[PARCIAL] Registro de bloques/items: 5 clases
          Depende del soporte de contenido de la API.
          - net.caffeinemc.mods.sodium.client.render.model.ItemGlintRenderType
          - net.caffeinemc.mods.sodium.client.render.model.ItemGlintSpecialRenderType
          - net.caffeinemc.mods.sodium.client.render.model.ItemRenderType
          - net.caffeinemc.mods.sodium.client.world.biome.LevelBiomeSlice
          - net.caffeinemc.mods.sodium.mixin.core.model.colors.BlockColorsMixin

[PARCIAL] HUD y render: 74 clases
          UI simple si; render propio depende de la API.
          - net.caffeinemc.mods.sodium.fabric.level.FabricLevelRenderHooks
          - net.caffeinemc.mods.sodium.fabric.render.FluidRendererImpl
          - net.caffeinemc.mods.sodium.client.gpu.arena.ArenaAggregator
          - net.caffeinemc.mods.sodium.client.gpu.arena.BufferArena
          - net.caffeinemc.mods.sodium.client.gpu.arena.DefragmentingBufferArena

```

## Datos
```
Datos reutilizables sin tocar codigo (se copian al exportar):
  Texturas              3
  Modelos/blockstates   44
  Idiomas               1
  Sonidos               0
  Recetas               0
  Loot tables           0
  Tags                  0
  Logros                0
  Worldgen JSON         0
```

## API de Minecraft usada
```
Clases de Minecraft mas referenciadas (hay que buscar su equivalente en Horizon):
     40 net.minecraft.client.Minecraft
     34 net.minecraft.util.Mth
     34 net.minecraft.core.SectionPos
     33 net.minecraft.core.Direction
     33 net.minecraft.network.chat.Component
     26 net.minecraft.core.BlockPos
     24 net.minecraft.client.gui.GuiGraphicsExtractor
     20 net.minecraft.resources.Identifier
     13 net.minecraft.client.renderer.texture.TextureAtlasSprite
     12 net.minecraft.world.level.block.state.BlockState
     12 net.minecraft.network.chat.MutableComponent
     11 net.minecraft.core.BlockPos$MutableBlockPos
     10 net.minecraft.client.resources.model.geometry.BakedQuad
     10 net.minecraft.ChatFormatting
     10 net.minecraft.client.input.KeyEvent
      9 net.minecraft.client.input.MouseButtonEvent
      9 net.minecraft.client.renderer.chunk.ChunkSectionLayer
      8 net.minecraft.client.renderer.block.BlockAndTintGetter
      8 net.minecraft.client.resources.model.geometry.BakedQuad$MaterialInfo
      8 net.minecraft.util.LightCoordsUtil
      8 net.minecraft.client.OptionInstance
      8 net.minecraft.client.Options
      8 net.minecraft.client.gui.Font
      8 net.minecraft.client.multiplayer.ClientLevel
      7 net.minecraft.world.level.material.FluidState

APIs del cargador (se descartan al portar):
      4 net.fabricmc.loader.api.FabricLoader
      4 net.fabricmc.fabric.api.client.renderer.v1.mesh.QuadEmitter
      3 net.fabricmc.loader.api.metadata.ModMetadata
      3 net.fabricmc.loader.api.ModContainer
      2 net.fabricmc.loader.api.Version
      2 net.fabricmc.fabric.api.client.render.fluid.v1.FluidRenderingRegistry
      2 net.fabricmc.fabric.api.client.rendering.v1.BlockTintsFactory
      2 net.fabricmc.fabric.api.client.rendering.v1.BlockColorRegistry
      2 net.fabricmc.fabric.api.client.render.fluid.v1.FluidRendering$DefaultRenderer
      2 net.fabricmc.fabric.api.client.render.fluid.v1.FluidRendering
```
