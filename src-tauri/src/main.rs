// Evita que aparezca una consola extra en Windows.
// En Linux no tiene efecto, pero se mantiene por si se soporta otro OS en el futuro.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    vaporstrap_lib::run();
}
