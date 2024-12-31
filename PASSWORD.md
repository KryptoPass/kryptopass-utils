# Especificación del Formato de Configuración  
**Versión del Documento:** 1.0  
**Última Actualización:** 28 de diciembre de 2024  

---

## Introducción  
Este documento define el formato de configuración utilizado para personalizar el comportamiento del generador de contraseñas de **KryptoPass**. La configuración se basa en el formato **TOML**, proporcionando una estructura clara y legible para definir reglas, perfiles y restricciones relacionadas con la generación de contraseñas.

El objetivo principal es permitir que los usuarios configuren longitudes, conjuntos de caracteres y restricciones específicas de manera **declarativa**, garantizando así una generación de contraseñas que cumpla con los requisitos de seguridad y usabilidad.

---

## Diseño General  
El archivo de configuración se estructura en **varias secciones**, cada una con una finalidad específica. A continuación, se describe cada una de estas secciones en detalle, junto con ejemplos prácticos para facilitar la implementación.

---

## 1. Campo `version`  
**Descripción:**  
Este campo es **obligatorio** y define la versión del formato de configuración utilizada. Se recomienda seguir el esquema de versionado semántico (**semver**) para facilitar la compatibilidad y trazabilidad de cambios.

**Formato:**  
```toml
version = "0.1.0"
```

*Semver (Semantic Versioning)* se compone de tres partes: `MAJOR.MINOR.PATCH`.

- **MAJOR:** Cambios incompatibles con versiones anteriores.  
- **MINOR:** Nuevas funcionalidades compatibles hacia atrás.  
- **PATCH:** Correcciones menores o parches de seguridad.

> A fecha de escribir este documento la última versión actual es la `v0.1.0`.

---

## 2. Sección `[profile]`  
**Descripción:**  
Define metadatos y atributos generales relacionados con el perfil de generación de contraseñas. Esta sección permite establecer identificadores únicos y nombres descriptivos para organizar y diferenciar configuraciones de generación.

**Formato:**  
```toml
[profile]
id = "123e4567-e89b-12d3-a456-426614174000"
name = "Perfil de contraseñas seguras"
```

**Parámetros:**  
- `id` *(string)* – Identificador único del perfil, preferentemente en formato **UUID**.  
- `name` *(string)* – Nombre descriptivo o amigable que identifica el perfil.

**Ejemplo:**  
```toml
[profile]
id = "c9f00f9a-bdf0-4f19-8f61-5b3c012f4f5b"
name = "Perfil Corporativo"
```

> [!NOTE]  
> Se pueden agregar más campos que sean necesarios para la identificación del archivo, pero deben ser manejados individualmente.

---

## 3. Sección `[rules]`  
**Descripción:**  
Define las reglas de construcción, validación y restricción de contraseñas. Esta sección permite especificar la longitud de las contraseñas, así como caracteres **incluidos** o **excluidos** durante la generación.

### 3.1 Campo `length`  
**Descripción:**  
Establece la longitud deseada de las contraseñas. Puede definirse como:  
- Un valor **específico** (entero).  
- Un **rango** (`min`, `max`).  
- La palabra reservada `auto`, para una longitud determinada automáticamente por el sistema.

**Formato:**  
```toml
[rules]
length = { min = 10, max = 64 }  # Rango
length = 12                      # Valor fijo
length = "auto"                  # Automático
```

**Ejemplo:**  
```toml
[rules]
length = { min = 8, max = 20 }  # La contraseña tendrá entre 8 y 20 caracteres
```

---

### 3.2 Campos `include` y `exclude`
**Descripción:**  
Permiten definir conjuntos de caracteres que deben incluirse o excluirse durante la generación de contraseñas. Se pueden utilizar rangos Unicode, caracteres individuales o expresiones al estilo ASCII.

**Parámetros:**  
- `include` *(array)* – Lista de caracteres o rangos que deben estar presentes.  
- `exclude` *(array)* – Lista de caracteres o rangos que deben evitarse.

**Formato:**  
```toml
[rules]
include = ["U+30A2", "U+02DC"]
exclude = ["U+1F600-U+1F64F"]
```

**Ejemplo:**  
```toml
[rules]
include = ["A-Z", "Ñ"]   # Incluir mayúsculas y la Ñ
exclude = ["U+1F600-U+1F64F"]  # Excluir emojis
```

---

### 3.3 Campo `max-consecutive`  
Define el número máximo de caracteres **consecutivos idénticos** permitidos.

```toml
[rules]
max-consecutive = 2  # No permite más de 2 caracteres iguales consecutivos
```

---

### 3.4 Campo `min-entropy-bits`  
Establece el nivel mínimo de entropía requerido para las contraseñas, medido en **bits**.

```toml
[rules]
min-entropy-bits = 24  # La contraseña debe tener al menos 24 bits de entropía
```

> [!NOTE]  
> El cálculo de entropía puede variar según la implementación, pero típicamente se usa la fórmula aproximada:
> \[
>   \text{Entropía} \approx \log_2(\text{alfabeto}^{\text{longitud}})
> \]
> donde `alfabeto` es la cantidad de caracteres finales permitidos tras aplicar `include`/`exclude`, y `longitud` corresponde al tamaño de la contraseña (fijo o mínimo).

---

### 3.5 Campo `pattern`
Permite definir una **estructura específica** de generación mediante patrones.

**Sintaxis del Patrón:**
- **Bloques**: `( )` – Define un bloque de caracteres.  
- **Cantidad**: `{}` – Indica cuántas veces debe repetirse el bloque.  
- **Negación**: `!` – Excluye ciertos caracteres o conjuntos.  
- **Control Universal**: `*` – El resto de la contraseña puede tener cualquier combinación de caracteres permitidos.

**Ejemplo:**  
```toml
[rules]
pattern = "(uppercase){1}(lowercase){3}(digits){2}*"
# Inicia con 1 mayúscula, seguido de 3 minúsculas, 2 dígitos y cualquier combinación.
```

```toml
[rules]
pattern = "(uppercase){1}(lowercase){3}(!digits){2}*"
# Donde `(!digits){2}`: Dos repeticiones de no-dígitos
```

> [!NOTE]  
> Puedes usar alias definidos en `[charset]` dentro del patrón (por ejemplo, `(uppercase)`, `(digits)`) si así lo desea la implementación.

---

## 4. Sección `[charset]`  
**Descripción:**  
Define **conjuntos de caracteres personalizados** que pueden utilizarse en las reglas de generación. Esto permite un mayor control sobre los caracteres disponibles, facilitando la creación de políticas específicas de seguridad.

**Formato:**  
```toml
[charset]
uppercase_1 = ["U+0041-U+005A", "U+00D1"]
uppercase_2 = ["A-Z", "Ñ"]
uppercase_3 = "ascii_uppercase"
```

**Ejemplos de Interpretación:**  
```text
uppercase_1 = ["U+0041-U+005A", "U+00D1"] -> A..Z y Ñ (Unicode)
uppercase_3 = "ascii_uppercase"           -> "ABCDEFGHIJKLMNOPQRSTUVWXYZ" (ASCII)
```

**Tabla de palabras reservadas completas:**
| Preset               | Valor                                                                |
|----------------------|----------------------------------------------------------------------|
| ascii_lowercase      | abcdefghijklmnopqrstuvwxyz                                           |
| ascii_uppercase      | ABCDEFGHIJKLMNOPQRSTUVWXYZ                                           |
| ascii_letters        | abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ                 |
| digits               | 0123456789                                                           |
| hexdigits            | 0123456789abcdefABCDEF                                               |
| octdigits            | 01234567                                                             |
| punctuation          | !"#$%&'()*+,-./:;<=>?@[\]^_`{|}~                                     |
| printable            | 0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ       |
|                      | !"#$%&'()*+,-./:;<=>?@[\]^_`{|}~                                     |

---

### 4.1 Ejemplo de Configuración Completa  
```toml
version = "0.1.0"

[profile]
id = "aabbccdd-1122-3344-5566-778899aabbcc"
name = "Perfil Personalizado"

[rules]
length = { min = 12, max = 24 }
include = ["A-Z", "0-9", "Ñ"]
exclude = ["U+1F600-U+1F64F"]

[charset]
uppercase = ["A-Z", "Ñ"]
digits = "0-9"
symbols = ["!@#$%^&*()"]
```

Este ejemplo define un perfil con una longitud de contraseña entre 12 y 24 caracteres, incluyendo mayúsculas, dígitos y la letra **Ñ**, mientras excluye emojis.

---

## Manejo de Conflictos y Precedencia

A continuación se explican las reglas y la secuencia en que se aplican distintas propiedades para la generación de contraseñas, con el fin de evitar ambigüedades.

### 8.1 Relación entre `length` y `pattern`  
En caso de que exista la propiedad `pattern` y `length` al mismo tiempo:

1. Se construye la estructura base a partir de `pattern`.  
2. Se verifica que la longitud **resultante** no exceda el `max` ni sea menor que el `min` definidos en `length`.  
3. Si el patrón base ya excede `max`, se considera un **error de configuración**.  
4. Si el patrón base es menor que `min`, se puede usar el comodín `*` para rellenar (en caso de estar definido así el patrón) hasta alcanzar la longitud mínima.

> [!NOTE]  
> Si no se define `length`, la longitud se infiere del patrón o de un valor por defecto de la implementación.

---

### 8.2 Precedencia entre `include`, `exclude`, `pattern` y `[charset]`
1. **Definición de Conjuntos Base**: Se parte de los conjuntos definidos en `[charset]`.  
2. **Aplicar `exclude`**: Se eliminan del conjunto base los caracteres listados en `exclude`.  
3. **Aplicar `include`**: Se reintroducen (o añaden) los caracteres listados en `include`.  
4. **Generación con `pattern`**:  
   - El `pattern` especifica **cómo** se construye la contraseña. Puede referenciar alias de `[charset]` (ej.: `(uppercase)`) o usar rangos directos (ej.: `(A-Z)`).  
   - Si se usa el comodín `*`, se aplica con la **suma global** de todos los caracteres que queden disponibles tras `exclude` y `include`.  

> [!NOTE]  
> Cada bloque o alias en el `pattern` puede, a su vez, estar sujeto a la lógica de exclusión/inclusión si la implementación lo permite. Sin embargo, se recomienda aclarar si `exclude` siempre anula ciertos caracteres aun dentro de un alias de `[charset]`.

---

### 8.3 Regla **`exclude` > `include`**  
Como estrategia de ejemplo, se puede optar por:

1. **Primero** se aplican las **exclusiones** (`exclude`) sobre los `[charset]`.  
2. **Después** se añaden los caracteres de `include`, incluso si estaban previamente en `exclude`.  

**Ventajas**  
- Asegura que cualquier elemento listado en `include` **siempre esté presente**, sin importar las exclusiones previas.  
- Útil para “recuperar” caracteres que se hayan excluido por error.

**Desventajas**  
- Puede reintroducir elementos que **deliberadamente** fueron excluidos, lo cual podría generar confusión o inseguridad si dichos caracteres eran peligrosos.

**Ejemplo**  
```toml
[charset]
my_specials = ["A-Z", "Ñ"]

[rules]
exclude = ["A-Z", "U+1F600"]    # Eliminar letras A-Z y el emoji 😀
include = ["U+1F600"]           # Volver a añadir el emoji
```
**Resultado final**:  
- Se excluyen todas las letras A-Z y el emoji 😀 inicialmente.  
- Luego se reintroduce 😀 mediante `include`.  
- El conjunto final contiene `Ñ` y el emoji 😀.

> [!TIP]  
> El sistema puede emitir una **advertencia** si detecta que un carácter se incluyó tras haberse excluido, para que el usuario sea consciente de la colisión.

---

## Validaciones y Errores
- **Inconsistencia de Rango**: Si `min` > `max` en `length`, debe reportarse un error y abortar la carga de configuración.  
- **Patrón Excedido**: Si la suma de bloques en `pattern` excede el `max` de `length` sin usar comodín `*`, se marca error de configuración.  
- **Referencias Inválidas**: Si un `pattern` usa un alias no definido en `[charset]`, se genera un error.  
- **Rangos Unicode Malformados**: Si un rango `U+XXXX-U+YYYY` está invertido o es inválido, se debe notificar.  

---

## Conclusiones
Esta especificación brinda la estructura y la **secuencia de reglas** necesarias para configurar el generador de contraseñas de **KryptoPass** de forma ordenada y transparente. Al definir claramente:

1. Cómo se determina la **longitud** (`length`).  
2. Cómo se aplican **exclusiones** (`exclude`) e **inclusiones** (`include`).  
3. Cómo interactúa el **patrón** (`pattern`) con los conjuntos definidos en `[charset]`.  
4. Qué ocurre ante **conflictos** (por ejemplo, `exclude` vs. `include`, o `pattern` vs. `length`).  

se logra una configuración **coherente**, **flexible** y ajustada a las necesidades de seguridad de cada organización o usuario.
