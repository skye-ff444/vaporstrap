//! Catálogo incorporado, solo para tener algo con qué buscar/explorar al
//! abrir el editor por primera vez.
//!
//! **Disclaimer importante:** esta lista es un punto de partida, no una base
//! de datos verificada contra la build actual de Roblox — los nombres de
//! flags cambian entre versiones y algunos de estos pueden estar
//! desactualizados o ya no tener efecto. Para lo que no está acá: usá
//! "agregar flag personalizada" o importá un JSON (p. ej. un export de
//! Bloxstrap o de algún tracker de flags de la comunidad).

use super::models::{CatalogEntry, FlagValue};

pub fn builtin_catalog() -> Vec<CatalogEntry> {
    vec![
        CatalogEntry {
            name: "DFIntTaskSchedulerTargetFps".into(),
            description: "Límite de FPS objetivo del cliente.".into(),
            default_value: FlagValue::Int(60),
            category: "Rendimiento".into(),
        },
        CatalogEntry {
            name: "FFlagFastGPULightCulling3".into(),
            description: "Variante más rápida del culling de luces en GPU.".into(),
            default_value: FlagValue::Bool(true),
            category: "Rendimiento".into(),
        },
        CatalogEntry {
            name: "FFlagDisablePostFx".into(),
            description: "Desactiva efectos de post-procesado (bloom, color correction, etc.).".into(),
            default_value: FlagValue::Bool(true),
            category: "Gráficos".into(),
        },
        CatalogEntry {
            name: "FIntRenderShadowmapBias".into(),
            description: "Ajusta el sesgo del shadow mapping; valores altos reducen artefactos a costa de precisión.".into(),
            default_value: FlagValue::Int(0),
            category: "Gráficos".into(),
        },
        CatalogEntry {
            name: "FFlagDebugSkyGray".into(),
            description: "Reemplaza el cielo por un gris plano, útil para aislar el costo de renderizar el skybox.".into(),
            default_value: FlagValue::Bool(false),
            category: "Debug".into(),
        },
        CatalogEntry {
            name: "DFIntCSGLevelOfDetailSwitchingDistance".into(),
            description: "Distancia a la que las uniones/CSG bajan de nivel de detalle.".into(),
            default_value: FlagValue::Int(300),
            category: "Rendimiento".into(),
        },
        CatalogEntry {
            name: "FFlagUserVoiceChatEnabled".into(),
            description: "Habilita/deshabilita el chat de voz en el cliente.".into(),
            default_value: FlagValue::Bool(true),
            category: "Social".into(),
        },
        CatalogEntry {
            name: "FIntRenderGraphicsQualityOverride".into(),
            description: "Fuerza un nivel de calidad gráfica (1-10), saltándose la detección automática.".into(),
            default_value: FlagValue::Int(1),
            category: "Gráficos".into(),
        },
        CatalogEntry {
            name: "FFlagDebugForceFRMQualityLevel".into(),
            description: "Fuerza el nivel de calidad del Future Rendering Mode.".into(),
            default_value: FlagValue::Bool(false),
            category: "Gráficos".into(),
        },
        CatalogEntry {
            name: "DFFlagDebugPrintFPS".into(),
            description: "Muestra un contador de FPS en pantalla (familia DF: se puede cambiar en caliente).".into(),
            default_value: FlagValue::Bool(false),
            category: "Debug".into(),
        },
    ]
}
