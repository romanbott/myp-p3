// Práctica 3: refactorización con patrones de diseño.
// Aplica Composite (`Elemento`/`Carpeta`), Factory Method (`FabricaArchivos`)
// y Adapter (`Notificador`/`AdaptadorCorreo`). Ver ANALISIS.md.

/// Datos comunes de un archivo: nombre y tamaño en unidades arbitrarias.
struct ArchivoBase {
    nombre: String,
    tamanio: i32,
}

/// Archivo del sistema; el tipo concreto se elige con la fábrica (Factory Method).
enum Archivo {
    PDF(ArchivoBase),
    Texto(ArchivoBase),
}

/// Fábrica (Factory Method) que decide el tipo concreto de archivo según su extensión.
struct FabricaArchivos;

impl FabricaArchivos {
    /// Crea un `Archivo` del tipo indicado ("pdf" o "txt").
    fn crear(tipo: &str, nombre: String, tamanio: i32) -> Archivo {
        match tipo {
            "pdf" => Archivo::PDF(ArchivoBase { nombre, tamanio }),
            "txt" => Archivo::Texto(ArchivoBase { nombre, tamanio }),

            _ => panic!("Tipo de archivo no soportado."),
        }
    }
}

/// Carpeta del sistema; actúa como Composite almacenando archivos y otras carpetas.
struct Carpeta {
    nombre: String,
    elementos: Vec<Box<dyn Elemento>>,
}

impl Carpeta {
    /// Crea una carpeta vacía con el nombre dado.
    fn new(nombre: &str) -> Self {
        Self {
            nombre: nombre.to_string(),
            elementos: Vec::new(),
        }
    }

    /// Agrega un elemento (archivo o carpeta) a la carpeta.
    fn agregar(&mut self, elemento: Box<dyn Elemento>) {
        self.elementos.push(elemento);
    }
}

/// Interfaz común del patrón Composite para archivos y carpetas.
trait Elemento {
    /// Devuelve el tamaño total del elemento (recursivo en carpetas).
    fn get_size(&self) -> i32;
}

impl Elemento for Archivo {
    fn get_size(&self) -> i32 {
        match self {
            Archivo::PDF(archivo_base) => archivo_base.tamanio,
            Archivo::Texto(archivo_base) => archivo_base.tamanio,
        }
    }
}

impl Elemento for Carpeta {
    fn get_size(&self) -> i32 {
        let mut total = 0;
        for archivo in &self.elementos {
            total += archivo.get_size();
        }
        total
    }
}

/// Correo heredado con una interfaz distinta a la del resto del programa.
struct CorreoLegacy;

impl CorreoLegacy {
    /// Envía un correo con la interfaz antigua.
    fn send_email(&self, to: &str, body: &str) {
        println!("Para: {}", to);
        println!("{}", body);
    }
}

/// Interfaz objetivo del patrón Adapter para enviar notificaciones.
trait Notificador {
    fn enviar(&self, destino: &str, mensaje: &str);
}

/// Adapta `CorreoLegacy` a la interfaz `Notificador` (patrón Adapter).
struct AdaptadorCorreo(CorreoLegacy);

impl Notificador for AdaptadorCorreo {
    /// Traduce `Notificador::enviar` a `CorreoLegacy::send_email`.
    fn enviar(&self, destino: &str, mensaje: &str) {
        self.0.send_email(destino, mensaje);
    }
}

/// Crea un archivo con `FabricaArchivos` y lo agrega a la carpeta.
fn agregar_archivo(carpeta: &mut Carpeta, tipo: &str, nombre: &str, tamanio: i32) {
    let archivo = FabricaArchivos::crear(tipo, nombre.to_string(), tamanio);

    carpeta.agregar(Box::new(archivo));
}

/// Devuelve el tamaño de cualquier `Elemento`.
fn obtener_tamanio(elemento: &dyn Elemento) -> i32 {
    elemento.get_size()
}

/// Envía por un `Notificador` el tamaño total de la carpeta.
fn enviar_resultado(carpeta: &Carpeta, destino: &str, notificador: &dyn Notificador) {
    notificador.enviar(
        destino,
        &format!("Tamanio total: {}", obtener_tamanio(carpeta)),
    );
}

fn main() {
    let notificador = AdaptadorCorreo(CorreoLegacy);
    let mut clase = Carpeta::new("MyP");
    agregar_archivo(&mut clase, "pdf", "practica.pdf", 120);
    agregar_archivo(&mut clase, "txt", "notas.txt", 80);

    let mut ejemplos = Carpeta::new("Ejemplos");
    agregar_archivo(&mut ejemplos, "txt", "ejemplo.txt", 50);
    clase.agregar(Box::new(ejemplos));

    println!("{}", obtener_tamanio(&clase));
    enviar_resultado(&clase, "profesor@universidad.edu", &notificador);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn carpeta_vacia() {
        let vacia = Carpeta::new("prueba");
        assert_eq!(obtener_tamanio(&vacia), 0);
    }

    #[test]
    fn carpeta_unico_pdf() {
        let mut prueba = Carpeta::new("prueba");

        agregar_archivo(&mut prueba, "pdf", "prueba.pdf", 120);

        assert_eq!(obtener_tamanio(&prueba), 120);
    }

    #[test]
    fn carpeta_pdf_y_texto() {
        let mut prueba = Carpeta::new("prueba");

        agregar_archivo(&mut prueba, "pdf", "prueba.pdf", 120);
        agregar_archivo(&mut prueba, "txt", "prueba.txt", 80);

        assert_eq!(obtener_tamanio(&prueba), 200);
    }

    #[test]
    fn ejemplo_anidado() {
        let mut clase = Carpeta::new("MyP");
        agregar_archivo(&mut clase, "pdf", "practica.pdf", 120);
        agregar_archivo(&mut clase, "txt", "notas.txt", 80);

        let mut ejemplos = Carpeta::new("Ejemplos");
        agregar_archivo(&mut ejemplos, "txt", "ejemplo.txt", 50);
        clase.agregar(Box::new(ejemplos));

        assert_eq!(obtener_tamanio(&clase), 250);
    }

    #[test]
    fn carpeta_unico_vacio() {
        let mut prueba = Carpeta::new("prueba");

        agregar_archivo(&mut prueba, "pdf", "prueba.pdf", 0);

        assert_eq!(obtener_tamanio(&prueba), 0);
    }
}
