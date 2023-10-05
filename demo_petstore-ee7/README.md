# Correr la demo con docker costumizado

Dentro de customizations se encuentra toda la configuración necesaria para correr move2kube y generar docker personalizados.

## Comando para correrla
Eliminar todos los archivos m2k que existen en el proyecto

Nos posicionamos en la raiz
```bash
move2kube transform agoncal-application-petstore-ee7 -c customizations/
```

Esto detectara los servicios y generara los artefactos todo con el transformer personalizado que utiliza el dockerfile personalizado y devuelve todo en otra carpeta llamda mis ymls.