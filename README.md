# Práctica 3 — Refactorización con patrones de diseño

## Qué hace

Modela un sistema de archivos simple: archivos PDF y de texto dentro de carpetas (que pueden anidarse).
Calcula el tamaño total de una carpeta y envía ese resultado por correo.

## Qué se refactorizó

- **Composite**: el trait `Elemento` unifica archivos y carpetas, y `Carpeta` calcula su tamaño delegando en sus hijos.
- **Factory Method**: `FabricaArchivos` concentra la creación de archivos según su tipo, quitando esa decisión de `agregar_archivo`.
- **Adapter**: `Notificador` define la interfaz buscada y `AdaptadorCorreo` adapta la interfaz heredada de `CorreoLegacy`, desacoplando `enviar_resultado` de ella.

El comportamiento observable no cambió: el tamaño total y el correo simulado son los mismos que antes de refactorizar. Ver `ANALISIS.md` para el detalle.

## Cómo correr

```sh
cargo run     # ejecuta main
cargo test    # corre las pruebas unitarias
```
