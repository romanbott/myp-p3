// Codigo inicial de la practica 3: aun NO esta refactorizado.
// Rust no tiene herencia de clases como Java/Python: el enum representa los
// dos tipos de archivo, pero se mantiene el problema de responsabilidades.

struct ArchivoBase {
    nombre: String,
    tamanio: i32,
}

enum Archivo {
    PDF(ArchivoBase),
    Texto(ArchivoBase),
}

struct FabricaArchivos;

impl FabricaArchivos {
    fn crear(tipo: &str, nombre: String, tamanio: i32) -> Archivo {
        match tipo {
            "pdf" => Archivo::PDF(ArchivoBase { nombre, tamanio }),
            "txt" => Archivo::Texto(ArchivoBase { nombre, tamanio }),

            _ => unimplemented!("Tipo de archivo no soportado."),
        }
    }
}

struct Carpeta {
    nombre: String,
    elementos: Vec<Box<dyn Elemento>>,
}

impl Carpeta {
    fn new(nombre: &str) -> Self {
        Self {
            nombre: nombre.to_string(),
            elementos: Vec::new(),
        }
    }

    fn agregar(&mut self, elemento: Box<dyn Elemento>) {
        self.elementos.push(elemento);
    }
}

trait Elemento {
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

struct CorreoLegacy;

impl CorreoLegacy {
    fn send_email(&self, to: &str, body: &str) {
        println!("Para: {}", to);
        println!("{}", body);
    }
}

trait Notificador {
    fn enviar(&self, destino: &str, mensaje: &str);
}

struct AdaptadorCorreo(CorreoLegacy);

impl Notificador for AdaptadorCorreo {
    fn enviar(&self, destino: &str, mensaje: &str) {
        self.0.send_email(destino, mensaje);
    }
}

fn agregar_archivo(carpeta: &mut Carpeta, tipo: &str, nombre: &str, tamanio: i32) {
    let archivo = FabricaArchivos::crear(tipo, nombre.to_string(), tamanio);

    carpeta.agregar(Box::new(archivo));
}

fn obtener_tamanio(elemento: &dyn Elemento) -> i32 {
    elemento.get_size()
}

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
