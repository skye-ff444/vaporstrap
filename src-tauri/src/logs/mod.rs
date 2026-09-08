//! Logs y diagnóstico, todo local — nada de esto se envía a ningún lado.
//! Se guarda en `<directorio de datos de la app>/logs/` para que el usuario
//! lo revise o lo adjunte a un reporte de bug si quiere.
//!
//! Dos fuentes:
//! - `vaporstrap.log`: eventos que dispara Vaporstrap mismo (instalar/
//!   lanzar Cordial, aplicar FastFlags u optimización de hardware, errores).
//! - `cordial-<fecha>.log`, uno por lanzamiento: antes de este módulo,
//!   `engine::cordial::launch` heredaba la salida estándar del proceso de
//!   Cordial, así que se perdía en la terminal donde corriera Vaporstrap (o
//!   a ningún lado, si se abrió desde un lanzador gráfico sin terminal).
//!   Ahora se redirige a un archivo por lanzamiento.

pub mod commands;
pub mod logger;
pub mod paths;
