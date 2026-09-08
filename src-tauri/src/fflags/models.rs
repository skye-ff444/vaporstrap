use serde::{Deserialize, Serialize};

/// Valor de una FastFlag. Roblox internamente los guarda todos como texto,
/// pero mantenemos el tipo para que la UI pueda mostrar un checkbox en vez
/// de un input de texto cuando corresponde.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum FlagValue {
    Bool(bool),
    Int(i64),
    Str(String),
}

impl FlagValue {
    /// Convierte a `serde_json::Value` tal como se escribe en `flags.json`.
    pub fn to_json(&self) -> serde_json::Value {
        match self {
            FlagValue::Bool(b) => serde_json::Value::Bool(*b),
            FlagValue::Int(i) => serde_json::Value::Number((*i).into()),
            FlagValue::Str(s) => serde_json::Value::String(s.clone()),
        }
    }

    /// Reconstruye un `FlagValue` a partir de lo leído de un `flags.json`
    /// existente (propio o importado). Todo lo que no sea bool/número se
    /// trata como string, igual que hace Cordial.
    pub fn from_json(value: &serde_json::Value) -> Self {
        match value {
            serde_json::Value::Bool(b) => FlagValue::Bool(*b),
            serde_json::Value::Number(n) => n
                .as_i64()
                .map(FlagValue::Int)
                .unwrap_or_else(|| FlagValue::Str(n.to_string())),
            other => FlagValue::Str(other.as_str().unwrap_or_default().to_string()),
        }
    }
}

/// Una fila del editor: nombre de la flag, si está habilitada, y su valor.
/// `enabled = false` significa "está en el preset pero no se aplica" — así
/// se puede desactivar una flag temporalmente sin perder el valor cargado.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlagEntry {
    pub name: String,
    pub enabled: bool,
    pub value: FlagValue,
    /// Descripción opcional (viene del catálogo, o la escribe el usuario).
    #[serde(default)]
    pub description: Option<String>,
}

/// Un conjunto de flags con nombre, guardado por el usuario.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    pub name: String,
    pub flags: Vec<FlagEntry>,
}

/// Entrada del catálogo incorporado: solo metadata para buscar/explorar.
/// No tiene `enabled` porque el catálogo no se aplica directo — el usuario
/// arrastra entradas del catálogo a un preset.
#[derive(Debug, Clone, Serialize)]
pub struct CatalogEntry {
    pub name: String,
    pub description: String,
    pub default_value: FlagValue,
    pub category: String,
}
