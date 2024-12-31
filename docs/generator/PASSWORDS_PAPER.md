# Documentación del Generador de Contraseñas - Versión Simplificada

## Introducción

Este generador de contraseñas está diseñado para proporcionar flexibilidad y seguridad en la generación de contraseñas, a la vez que simplifica la administración interna de los requisitos, validaciones de rangos, y patrones. Esta documentación explica el diseño del sistema para desarrolladores y cómo abordar las validaciones de longitud, rangos y patrones de forma cohesiva.

## Diseño General

El generador maneja los siguientes aspectos de manera simplificada:

1. **Rangos y longitud de la contraseña**: Los valores mínimos y máximos para cada tipo de carácter deben ser consistentes con la longitud total de la contraseña.
2. **Integración de patrones**: Los patrones deben respetar los requisitos de rangos y proveer una estructura opcional para la generación de contraseñas.
3. **Validación y generación**: Se valida que los requisitos sean coherentes antes de proceder con la generación de la contraseña.

## Flujo del Generador

### Paso 1: Leer y procesar la configuración

El archivo de configuración define los requisitos de tipos de caracteres, rangos mínimos y máximos, así como un patrón opcional para la generación de contraseñas. El patrón, si está presente, debe ser compatible con los requisitos.

### Paso 2: Validar los requisitos y el patrón

Antes de generar una contraseña, se valida lo siguiente:

- La suma de los valores mínimos de cada tipo de carácter debe ser menor o igual a la longitud máxima de la contraseña.
- La suma de los valores máximos de cada tipo de carácter debe ser mayor o igual a la longitud mínima de la contraseña.
- Si hay un patrón, se asegura que no contradiga los rangos definidos.

### Paso 3: Generación de la contraseña

Dependiendo de si existe un patrón, el proceso de generación sigue dos rutas:

1. **Generación con patrón**: La contraseña sigue la estructura definida por el patrón, completando cualquier carácter restante de manera aleatoria si es necesario.
2. **Generación aleatoria**: Si no hay un patrón, la contraseña se genera de forma aleatoria respetando los rangos definidos.

### Paso 4: Validar la contraseña generada

Una vez generada, la contraseña es validada para asegurar que cumple con los requisitos de longitud, cantidad de tipos de caracteres, y reglas adicionales (por ejemplo, entropía mínima).

---

## Validación de Rangos y Longitud

La validación de rangos y longitud se realiza asegurando que:

1. La suma de los valores mínimos de cada tipo de carácter no exceda la longitud máxima.
2. La suma de los valores máximos de cada tipo de carácter no sea menor a la longitud mínima.

### Ejemplo de validación:

```toml
[requirements]
digits = {min = 2, max = 6}
lowercase = {min = 2, max = inf}
uppercase = {min = 2, max = inf}
symbols = {min = 1, max = 4}
length = {min = 10, max = 64}
```

Validación:

- **Suma de mínimos**: 2 (digits) + 2 (lowercase) + 2 (uppercase) + 1 (symbols) = 7
- **Suma de máximos**: 6 (digits) + inf (lowercase) + inf (uppercase) + 4 (symbols) = inf

La suma de mínimos (7) es menor o igual a la longitud máxima (64), y la suma de máximos (inf) es mayor o igual a la longitud mínima (10), por lo tanto los requisitos son válidos.

### Reglas para los rangos infinitos:

- Si el valor `max` es `inf`, se considera que el límite real es la longitud máxima de la contraseña.
- Si `max` excede la longitud máxima de la contraseña, se ajusta automáticamente al valor de la longitud máxima.

---

## Integración de Patrones

Los patrones permiten definir una estructura específica para la contraseña. Los patrones deben cumplir con los siguientes principios:

- Los bloques del patrón deben respetar los valores mínimos y máximos definidos en los requisitos.
- Un patrón no puede generar más caracteres de los permitidos por los rangos establecidos.

### Ejemplo de un patrón:

```toml
[rules]
pattern = "(uppercase){1}(lowercase){3}(digits){2}*"
```

Este patrón indica que la contraseña debe comenzar con 1 carácter en mayúsculas, seguido de 3 caracteres en minúsculas, luego 2 dígitos. El resto de la contraseña se completa con cualquier carácter permitido, respetando los requisitos.

### Validación del patrón:

1. Se verifica que el patrón no exceda los valores máximos permitidos por los requisitos.
2. Se ajustan los mínimos y máximos de los tipos de caracteres si el patrón impone cantidades fijas.

---

## Ejemplo de Código Simplificado

Aquí se muestra un pseudocódigo que describe cómo implementar las validaciones y generación de contraseñas de manera sencilla:

```python
def cargar_configuración(archivo):
    config = leer_toml(archivo)
    requisitos = procesar_requisitos(config['requirements'])
    patrón = procesar_patron(config.get('rules', {}).get('pattern', None))
    return requisitos, patrón

def procesar_requisitos(requisitos_raw):
    requisitos = {}
    for tipo, valores in requisitos_raw.items():
        min_val = valores.get('min', 0)
        max_val = valores.get('max', float('inf'))
        requisitos[tipo] = Requisito(tipo, min_val, max_val)
    return requisitos

def validar_requisitos(requisitos, length_min, length_max):
    suma_min = sum(r.min for r in requisitos.values())
    suma_max = sum(r.max for r in requisitos.values())
    if suma_min > length_max:
        raise ValueError("La suma de los mínimos excede la longitud máxima.")
    if suma_max < length_min:
        raise ValueError("La suma de los máximos es menor que la longitud mínima.")

def generar_contraseña(requisitos, patrón, length):
    if patrón:
        contraseña = generar_contraseña_con_patron(requisitos, patrón, length)
    else:
        contraseña = generar_contraseña_aleatoria(requisitos, length)
    return contraseña

def generar_contraseña_con_patron(requisitos, patrón, length):
    contraseña = ""
    total_caracteres = 0
    for tipo, cantidad in patrón.secuencia:
        caracteres = obtener_caracteres(tipo, cantidad)
        contraseña += caracteres
        total_caracteres += cantidad
        requisitos[tipo].min -= cantidad
        requisitos[tipo].max -= cantidad
    if total_caracteres < length:
        contraseña += completar_con_caracteres_aleatorios(requisitos, length - total_caracteres)
    return contraseña

def generar_contraseña_aleatoria(requisitos, length):
    contraseña = ""
    tipos = list(requisitos.keys())
    while len(contraseña) < length:
        tipo = seleccionar_tipo(requisitos)
        carácter = obtener_caracter_aleatorio(tipo)
        contraseña += carácter
        actualizar_requisitos(requisitos, tipo)
    return contraseña
```

---

## Manejo de Errores

El sistema devuelve errores claros cuando los requisitos no son consistentes:

- "La suma de los mínimos excede la longitud máxima de la contraseña."
- "El patrón especificado requiere más caracteres de los permitidos por los máximos definidos."

---

## Conclusión

Este diseño simplificado mantiene la flexibilidad y poder del generador de contraseñas, a la vez que facilita la implementación y la validación. Al unificar las validaciones de rangos y longitud, y al integrar patrones de manera cohesiva, se reduce la complejidad del sistema, permitiendo a los desarrolladores enfocarse en mejorar otras áreas críticas del generador.

