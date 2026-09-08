//! "Optimizar mi laptop": sondea el hardware y recomienda ajustes.
//!
//! **Decisión de diseño importante:** Cordial ya tiene su propio sistema de
//! modos de rendimiento (`CORDIAL_PERFORMANCE=balanced|throughput|latency`,
//! ver `crates/cordial-runtime/src/flags.rs`) y de backend gráfico
//! (`CORDIAL_GRAPHICS=automatic|vulkan|gles`, ver
//! `crates/cordial-runtime/src/graphics.rs`), verificados contra su código
//! fuente. Vaporstrap **no reimplementa** esa lógica: sondea el hardware y
//! elige uno de los valores que Cordial ya entiende, más un par de FastFlags
//! (FPS cap y calidad gráfica) que sí pasan por el editor del punto 3.
//!
//! Este módulo hace tres cosas:
//! 1. `probe` — sondeo de hardware (núcleos físicos, RAM, si hay algún ICD
//!    de Vulkan instalado, y un microbenchmark de CPU deliberadamente
//!    simple). Heurística, no un benchmark certificado — se lo dice a la UI.
//! 2. `recommend` — reglas simples que traducen ese sondeo a una
//!    recomendación, con el razonamiento en texto plano para que el usuario
//!    entienda por qué.
//! 3. `settings` — persiste la recomendación aplicada
//!    (`<app_data_dir>/launch-env.json`) para que `engine::commands::engine_launch`
//!    la use la próxima vez que se inicie Cordial.

pub mod commands;
pub mod probe;
pub mod recommend;
pub mod settings;
