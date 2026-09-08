//! Sondeo de hardware. Todo best-effort: si algo no se puede leer (por
//! ejemplo `/proc/cpuinfo` en un sistema no-Linux, o `lspci` ausente), se
//! degrada a `None`/a un valor conservador en vez de fallar el comando
//! entero — un dato faltante no debería tirar abajo toda la recomendación.

use std::collections::HashSet;
use std::time::Instant;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct HardwareSnapshot {
    pub physical_cores: usize,
    pub logical_cores: usize,
    pub ram_mb: Option<u64>,
    pub vulkan_icd_found: bool,
    pub gpu_hint: Option<String>,
    /// Milisegundos que tardó un trabajo de CPU fijo (ver `cpu_bench_ms`).
    /// Es una heurística relativa, no un puntaje comparable entre máquinas
    /// con distinta arquitectura.
    pub cpu_bench_ms: f64,
}

/// Núcleos *físicos*, no hilos. Cuenta pares únicos (physical id, core id)
/// de `/proc/cpuinfo`, igual que hace Cordial en su propio
/// `flags.rs::physical_cores` (confirmado leyendo su código fuente). Si no
/// se puede leer `/proc/cpuinfo`, cae a `logical_cores()`.
pub fn physical_cores() -> usize {
    let Ok(text) = std::fs::read_to_string("/proc/cpuinfo") else {
        return logical_cores();
    };

    let mut pairs: HashSet<(String, String)> = HashSet::new();
    let mut physical_id = String::new();
    let mut core_id = String::new();

    for line in text.lines() {
        if let Some((key, value)) = line.split_once(':') {
            match key.trim() {
                "physical id" => physical_id = value.trim().to_string(),
                "core id" => {
                    core_id = value.trim().to_string();
                    pairs.insert((physical_id.clone(), core_id.clone()));
                }
                _ => {}
            }
        }
    }

    if pairs.is_empty() {
        logical_cores()
    } else {
        pairs.len()
    }
}

pub fn logical_cores() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
}

/// RAM total en MB, leyendo `MemTotal` de `/proc/meminfo`.
pub fn ram_mb() -> Option<u64> {
    let text = std::fs::read_to_string("/proc/meminfo").ok()?;
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("MemTotal:") {
            let kb_text = rest.trim().trim_end_matches("kB").trim();
            let kb: u64 = kb_text.parse().ok()?;
            return Some(kb / 1024);
        }
    }
    None
}

/// True si se encontró al menos un ICD (Installable Client Driver) de
/// Vulkan en las rutas estándar. No garantiza que Vulkan *funcione*
/// (podría estar roto), pero su ausencia total sí es una señal fuerte de
/// que el `dlopen("libvulkan.so")` que hace el motor de Cordial va a
/// fallar y caer a GLES3 (ver `engine::cordial` / `graphics.rs` de Cordial).
pub fn vulkan_icd_found() -> bool {
    const CANDIDATE_DIRS: &[&str] = &[
        "/usr/share/vulkan/icd.d",
        "/etc/vulkan/icd.d",
        "/usr/local/share/vulkan/icd.d",
    ];

    CANDIDATE_DIRS.iter().any(|dir| {
        std::fs::read_dir(dir)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .any(|e| e.path().extension().and_then(|s| s.to_str()) == Some("json"))
            })
            .unwrap_or(false)
    })
}

/// Primera línea de `lspci` que mencione una GPU, si `lspci` está
/// disponible. Puramente informativo para mostrar en la UI — no se usa
/// para decidir nada en `recommend`.
pub fn gpu_hint() -> Option<String> {
    let output = std::process::Command::new("lspci").output().ok()?;
    if !output.status.success() {
        return None;
    }

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .find(|line| {
            let lower = line.to_ascii_lowercase();
            lower.contains("vga compatible controller") || lower.contains("3d controller")
        })
        .map(|line| line.to_string())
}

/// Microbenchmark deliberadamente simple y determinista: un trabajo fijo de
/// aritmética entera, cronometrado. Sirve para distinguir "núcleo lento" de
/// "núcleo rápido" en la misma arquitectura, no como puntaje absoluto ni
/// para comparar contra benchmarks de terceros.
pub fn cpu_bench_ms() -> f64 {
    let start = Instant::now();

    let mut acc: u64 = 0xdead_beef_c0ffee;
    for i in 0..20_000_000u64 {
        acc = acc.wrapping_mul(2_862_933_555_777_941_757).wrapping_add(i);
    }
    std::hint::black_box(acc);

    start.elapsed().as_secs_f64() * 1000.0
}

pub fn snapshot() -> HardwareSnapshot {
    HardwareSnapshot {
        physical_cores: physical_cores(),
        logical_cores: logical_cores(),
        ram_mb: ram_mb(),
        vulkan_icd_found: vulkan_icd_found(),
        gpu_hint: gpu_hint(),
        cpu_bench_ms: cpu_bench_ms(),
    }
}
