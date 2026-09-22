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

struct Carpeta {
    nombre: String,
    archivos: Vec<Archivo>,
    subcarpetas: Vec<Carpeta>,
}

impl Carpeta {
    fn new(nombre: &str) -> Self {
        Self {
            nombre: nombre.to_string(),
            archivos: Vec::new(),
            subcarpetas: Vec::new(),
        }
    }
}

struct CorreoLegacy;

impl CorreoLegacy {
    fn send_email(&self, to: &str, body: &str) {
        println!("Para: {}", to);
        println!("{}", body);
    }
}

fn agregar_archivo(carpeta: &mut Carpeta, tipo: &str, nombre: &str, tamanio: i32) {
    if tipo == "pdf" {
        carpeta.archivos.push(Archivo::PDF(ArchivoBase {
            nombre: nombre.to_string(),
            tamanio,
        }));
    } else if tipo == "txt" {
        carpeta.archivos.push(Archivo::Texto(ArchivoBase {
            nombre: nombre.to_string(),
            tamanio,
        }));
    }
}

fn obtener_tamanio(carpeta: &Carpeta) -> i32 {
    let mut total = 0;
    for archivo in &carpeta.archivos {
        total += match archivo {
            Archivo::PDF(datos) => datos.tamanio,
            Archivo::Texto(datos) => datos.tamanio,
        };
    }
    for subcarpeta in &carpeta.subcarpetas {
        total += obtener_tamanio(subcarpeta);
    }
    total
}

fn enviar_resultado(carpeta: &Carpeta, destino: &str) {
    let correo = CorreoLegacy;
    correo.send_email(
        destino,
        &format!("Tamanio total: {}", obtener_tamanio(carpeta)),
    );
}

fn main() {
    let mut clase = Carpeta::new("MyP");
    agregar_archivo(&mut clase, "pdf", "practica.pdf", 120);
    agregar_archivo(&mut clase, "txt", "notas.txt", 80);

    let mut ejemplos = Carpeta::new("Ejemplos");
    agregar_archivo(&mut ejemplos, "txt", "ejemplo.txt", 50);
    clase.subcarpetas.push(ejemplos);

    println!("{}", obtener_tamanio(&clase));
    enviar_resultado(&clase, "profesor@universidad.edu");
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
        clase.subcarpetas.push(ejemplos);

        assert_eq!(obtener_tamanio(&clase), 250);
    }

    #[test]
    fn carpeta_unico_vacio() {
        let mut prueba = Carpeta::new("prueba");

        agregar_archivo(&mut prueba, "pdf", "prueba.pdf", 0);

        assert_eq!(obtener_tamanio(&prueba), 0);
    }
}
