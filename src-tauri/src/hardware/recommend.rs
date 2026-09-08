//! Traduce un [`HardwareSnapshot`] a una recomendación concreta.
//!
//! Reglas simples a propósito — esto es una heurística de arranque, no un
//! sistema de tuning con datos de miles de máquinas. `reasoning` existe para
//! que quien la lea pueda decidir si tiene sentido para su caso, no para que
//! confíe a ciegas.

use serde::{Deserialize, Serialize};

use super::probe::HardwareSnapshot;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    /// Uno de "balanced" | "throughput" | "latency" — valor nativo de
    /// `CORDIAL_PERFORMANCE` (ver `crates/cordial-runtime/src/flags.rs`).
    pub performance: String,
    /// Uno de "automatic" | "gles" — valor nativo de `CORDIAL_GRAPHICS`.
    /// No recomendamos forzar "vulkan" explícitamente: si hay un ICD
    /// disponible, "automatic" ya lo intenta primero y cae a GLES3 solo si
    /// falla, que es más seguro que forzar.
    pub graphics: String,
    /// Se aplica como `DFIntTaskSchedulerTargetFps` en el `flags.json` del
    /// perfil elegido.
    pub fps_cap: i64,
    /// 1-10. Se aplica como `FIntRenderGraphicsQualityOverride`.
    pub graphics_quality: i64,
    /// Explicación en texto plano de cada decisión, en el mismo orden que
    /// los campos de arriba.
    pub reasoning: Vec<String>,
}

pub fn recommend(snap: &HardwareSnapshot) -> Recommendation {
    let mut reasoning = Vec::new();

    let performance = if snap.physical_cores >= 6 {
        reasoning.push(format!(
            "{} núcleos físicos: modo 'throughput' (más hilos de trabajo para renderizado/física).",
            snap.physical_cores
        ));
        "throughput"
    } else if snap.physical_cores <= 2 {
        reasoning.push(format!(
            "Solo {} núcleo(s) físico(s): modo 'latency' (menos overhead de scheduling en vez de repartir en más hilos de los que hay).",
            snap.physical_cores
        ));
        "latency"
    } else {
        reasoning.push(format!(
            "{} núcleos físicos: modo 'balanced', el default de Cordial.",
            snap.physical_cores
        ));
        "balanced"
    };

    let graphics = if !snap.vulkan_icd_found {
        reasoning.push(
            "No se encontró ningún ICD de Vulkan en las rutas estándar del sistema: se fuerza GLES3 para que el motor no pierda tiempo intentando Vulkan.".to_string(),
        );
        "gles"
    } else {
        reasoning.push(
            "Se encontró al menos un ICD de Vulkan: se deja 'automatic' (Cordial intenta Vulkan primero y cae a GLES3 si falla).".to_string(),
        );
        "automatic"
    };

    let ram_gb = snap.ram_mb.map(|mb| mb as f64 / 1024.0);
    let modest = snap.physical_cores <= 2
        || ram_gb.map(|gb| gb < 6.0).unwrap_or(false)
        || snap.cpu_bench_ms > 900.0;
    let generous = snap.physical_cores >= 6 && ram_gb.map(|gb| gb >= 12.0).unwrap_or(false);

    let (fps_cap, graphics_quality) = if modest {
        reasoning.push(format!(
            "Hardware modesto (núcleos, RAM{} o el microbenchmark de CPU tardó {:.0} ms): FPS cap 30 y calidad gráfica baja (3/10).",
            ram_gb.map(|gb| format!(" de {gb:.1} GB")).unwrap_or_default(),
            snap.cpu_bench_ms
        ));
        (30, 3)
    } else if generous {
        reasoning.push("Hardware holgado: FPS cap 60 y calidad gráfica alta (8/10).".to_string());
        (60, 8)
    } else {
        reasoning.push("Hardware intermedio: FPS cap 60 y calidad gráfica media (6/10).".to_string());
        (60, 6)
    };

    Recommendation {
        performance: performance.to_string(),
        graphics: graphics.to_string(),
        fps_cap,
        graphics_quality,
        reasoning,
    }
}
