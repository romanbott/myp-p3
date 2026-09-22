## Análisis del estado inicial del programa
1. El método agregarArchivo agrega archivos a carpetas.
  Para esto recibe un tipo de archivo, nombre, tamaño y carpeta, y decide qué tipo de archivo crear según el tipo, lo crea con el nombre y el tamaño, y posteriormente lo agrega a la carpeta.

2. El método obtenerTamanio calcula el tamaño total de una carpeta.
  Para esto recorre recursivamente todos los archivos y carpetas, y dependiendo del tipo de archivo, suma el tamaño.

3. El método enviarResultado se encarga de enviar el tamaño total de una carpeta por correo.
