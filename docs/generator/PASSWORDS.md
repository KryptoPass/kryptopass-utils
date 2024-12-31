# Documentación del Generador de Contraseñas de KryptoPass

## Introducción

El generador de contraseñas de KryptoPass está diseñado para proporcionar flexibilidad y seguridad en la generación de contraseñas personalizadas, permitiendo una administración simplificada de los requisitos, validaciones de rangos, y patrones. Este documento está dirigido a desarrolladores que deseen entender y modificar el generador de contraseñas, explicando su diseño interno y cómo funciona la validación de longitud, rangos, y patrones de manera cohesiva.

El formato de configuración es TOML, que permite una estructura clara, fácil de leer y escribir, ideal para definir los perfiles de generación de contraseñas.

## Diseño General

El generador maneja los siguientes aspectos fundamentales:

1. **Rangos y longitud de la contraseña**: Los valores mínimos y máximos para cada tipo de carácter deben ser consistentes con la longitud total de la contraseña.
2. **Integración de patrones**: Los patrones deben respetar los requisitos de rangos y proveer una estructura opcional para la generación de contraseñas.
3. **Validación y generación**: Se valida que los requisitos sean coherentes antes de proceder con la generación de la contraseña.

El archivo de configuración se compone de varias secciones, algunas de ellas opcionales, que permiten a los usuarios definir solo los aspectos relevantes:

- `[properties]`: Define las propiedades generales del perfil de generación.
- `[requirements]`: Establece los requisitos mínimos y máximos para tipos de caracteres y longitud.
- `[allowed]` y [not_allowed]: CConjuntos de caracteres permitidos y excluidos, definidos por sus códigos Unicode.
- `[rules]`: Especifica reglas adicionales y/o avanzadas diseñadas para brindar inteligencia y seguridad a la generación de contraseñas.
- `[custom]`: Definición de conjuntos de caracteres personalizados basados en los idiomas especificados en `[properties].[lang]`.

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

## Sección `[properties]`
Define las propiedades generales del perfil de generación de contraseñas. Esta sección es obligatoria y debe incluir los siguientes campos:

```toml
[properties]
version = "0.1.0"                   # Versión del archivo de configuración
lang = ["es"]                       # Idiomas soportados (ISO 639-1)
name = "Perfil Seguro Corporativo"  # Nombre descriptivo del perfil
type = "password"                   # Tipo de generación (password, passphrase)
```

## Sección `[requirements]`

Aquí se definen los tipos de caracteres permitidos y sus rangos mínimos y máximos. El campo `length` es obligatorio y establece la longitud mínima, máxima o literal de la contraseña. Los rangos `{min, max}` o números fijos determinan cuántos caracteres de cada tipo deben incluirse en la contraseña.

```toml
[requirements]
length = {min = 10, max = 64}     # Longitud total entre 10 y 64 caracteres (obligatorio)
lowercase = {min = 2, max = inf}  # Definido por el usuario (ejemplo: letras minúsculas)
uppercase = {min = 2, max = inf}  # Definido por el usuario (ejemplo: letras mayúsculas)
custom_set_3 = 3                  # Definido por el usuario, número literal (ejemplo: exactamente 3 caracteres del conjunto)
```

> **Nota**: En los ejemplos puede llevar a ver los nombres como *lowercase, uppercase, digits*, etc., Estos son ejemplos de conjuntos que el usuario define en la sección correspondiente y **no deben interpretarse de manera literal**.

### Validación de Rangos y Longitud

La validación de rangos y longitud se realiza asegurando que:

1. La suma de los `valores mínimos` de cada tipo de carácter no exceda la longitud máxima.
2. La suma de los `valores máximos` de cada tipo de carácter no sea menor a la longitud mínima.

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

**Restricciones:**
- No se puede usar `length` como nombre de conjunto. Esto se debe a que length tiene un propósito específico en el sistema y es necesario para definir la longitud total de la contraseña.
- No se pueden repetir nombres de conjuntos. Cada conjunto debe tener un nombre único para evitar conflictos en la generación de contraseñas.

## Secciones `[allowed]` y `[not_allowed]`

Define caracteres adicionales permitidos o excluidos mediante códigos Unicode. Los caracteres en la lista `not_allowed` tienen prioridad y serán excluidos incluso si están permitidos en `allowed`.

```toml
[allowed]
include = ["U+30A2", "U+02DC"]    # Caracteres permitidos (ejemplo: Katakana Letter A y virgulilla)

[not_allowed]
exclude = ["U+1F600-U+1F64F"]     # Caracteres excluidos (ejemplo: Emoticonos)
```

## Sección [rules]

Aquí se especifican reglas avanzadas como la estructura de patrones, el número máximo de caracteres consecutivos, y la entropía mínima requerida.


### Ejemplo de reglas:

```toml
[rules]
max-consecutive = 2          # Máximo de caracteres consecutivos iguales
min-entropy-bits = 24        # Entropía mínima requerida (en bits)
pattern = "(uppercase){1}(lowercase){3}(digits){2}*"  # Patrón opcional
```

### Patrones

Los patrones permiten definir una estructura específica para la contraseña. La sintaxis incluye bloques, conjuntos de caracteres, cantidades y negaciones:

- Bloques: Se definen entre paréntesis `()`.
- Cantidad: Indicada entre llaves `{}`, puede ser un valor fijo, un rango, o ilimitado.
- Negación: Utilizando `^` para excluir ciertos conjuntos.
- Carácter de control: El `*` indica que el resto de la contraseña puede ser cualquier combinación de caracteres permitidos.

```toml
[rules]
pattern = "(uppercase){1}(lowercase){3}(digits){2}*"  # Inicia con 1 mayúscula, seguido de 3 minúsculas, 2 dígitos y cualquier combinación para completar la longitud.
```

### Validación de Patrones

Los patrones deben respetar los requisitos definidos en `[requirements]`. La cantidad de caracteres especificada en el patrón debe estar dentro de los rangos mínimos y máximos permitidos.

1. Se verifica que el patrón no exceda los valores máximos permitidos por los requisitos.
2. Se ajustan los mínimos y máximos de los tipos de caracteres si el patrón impone cantidades fijas.

# Secciones `[langauge_code]`

Estas seccies permiten definir conjuntos de caracteres personalizados basados en el idioma especificado en [properties].[lang]. Esto es útil para soportar múltiples idiomas o crear perfiles específicos.

```toml
[iso_639_1_code]
uppercase = ["U+0041-U+005A", "U+00D1"]     # A-Z y Ñ
my_custom_set = ["U+0061-U+007A", "U+00F1"] # a-z y ñ
digits = ["U+0030-U+0039"]                  # 0-9
hi = ["U+0021-U+002F", "U+00A1"]            # !"#$%&'()*+,-./ y ¡
```
