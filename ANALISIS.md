## Análisis del estado inicial del programa
1. El método agregarArchivo agrega archivos a carpetas.
  Para esto recibe un tipo de archivo, nombre, tamaño y carpeta, y decide qué tipo de archivo crear según el tipo, lo crea con el nombre y el tamaño, y posteriormente lo agrega a la carpeta.

2. El método obtenerTamanio calcula el tamaño total de una carpeta.
  Para esto recorre recursivamente todos los archivos y carpetas, y dependiendo del tipo de archivo, suma el tamaño.

3. El método enviarResultado se encarga de enviar el tamaño total de una carpeta por correo.


## Problemas identificados

- Varias funciones tienen que considerar cada caso de tipo de archivo
- Demasiado acoplamiento entre la función `enviar_resultados` y la interfaz de `CorreoLegacy` 
- Al manejar los archivos como un enum, hay poca extensibilidad si se quiere agregar nuevos tipos de archivo.


## Pruebas unitarias necesarias

- Carpeta vacia -> tamaño cero
- Carpeta con pdf de 120 -> tamaño 120
- Carpeta con pdf de 120 y texto de 80 -> tamaño 200
- Ejemplo con subcarpeta de 50 y tamaño total 250
- Carpeta con archivo de tamaño 0 -> tamaño 0


# Refactorización

## Patrón _Composite_
Usar el patrón _composite_ para tener una interfaz unificada para manejar archivos y carpetas.

Usar un _trait_ para abstraer los elementos del sistema de archivos y usar _trait objects_ para los argumentos en las funciones.

La función `obtener_tamanio` ahora es más natural que sea un método del _trait_.

## Patrón _Factory Method_

Usar el patrón _factory method_ para abstraer la creación de archivos, de esta forma desacoplando el manejo de casos para archivos pdf y txt.

Definiremos una estructura `FabricaArchivos` que implementa un método `crear` a donde delegaremos la decisión de qué tipo de archivo crear.



## Patrón _Adapter_

Usar el patrón _adapter_ para cambiar la interfaz que se usa en `main` para enviar correos, conservando la interfaz que expone `CorreoLegacy`.

Definiremos un nuevo trait `Notificador` para definir la interfaz buscada, y posteriormente basta encapsular un `CorreoLegacy` en una estructura `AdapatadorCorreo` que implementa `Notificador` e internamente usa `CorreoLegacy`.

La función `enviar_resultado` ahora recibe un `Notificador` como parámetro adicional, y en `main` se crea una instancia de `AdaptadorCorreo`.
