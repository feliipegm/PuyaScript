# PuyaScript — Documentación

PuyaScript es un lenguaje de programación interpretado, de sintaxis inspirada en la jerga andaluza. Su intérprete está escrito en Python 3 y funciona en tres fases: **lexer → parser (AST) → intérprete**.

---

## 1. Ejecución

```bash
python puyascript.py programa.puya
```

- El archivo **debe** tener extensión `.puya`, o el intérprete aborta con `¡Poriera! El archivo tiene que ser .puya`.
- Los errores se muestran como `Error fatal: <mensaje>`.
- Los errores propios del lenguaje empiezan por `¡Poriera!`.

---

## 2. Estructura básica

- Las sentencias terminan en `;`.
- Los bloques van entre llaves `{ }`.
- Los comentarios son de línea y empiezan por `//`.
- Se distinguen mayúsculas y minúsculas (`siAcaso` ≠ `siacaso`).
- Los identificadores admiten letras, dígitos y `_`, y no pueden empezar por dígito.

```puya
// Esto es un comentario
chivato("¡Ozú, qué calor!");
```

---

## 3. Tipos de datos

| Tipo     | Significado          | Ejemplo            |
|----------|----------------------|--------------------|
| `guita`  | Entero               | `42`               |
| `suelto` | Decimal (float)      | `3.14`             |
| `palique`| Texto (string)       | `"hola"`           |
| `fijo`   | Booleano             | `verdá` / `esIA`   |
| `na`     | Sin valor (retorno de funciones) | —      |

- `verdá` = verdadero, `esIA` = falso.
- Los textos van entre comillas dobles `"..."`. No hay secuencias de escape ni comillas dobles dentro de un texto.
- Los números no admiten signo en el literal (ver *Limitaciones*).

> Los tipos se usan al declarar, pero **no se comprueban** en ejecución: el intérprete no impide guardar un `palique` en una variable `guita`.

---

## 4. Variables

### Declaración
```puya
guita edad = 30;
suelto altura = 1.75;
palique nombre = "Manolo";
fijo contento = verdá;
```

### Asignación
```puya
edad = edad + 1;
```
Asignar a una variable que no existe lanza `¡Poriera! No puedes asignar a '...'`.

---

## 5. Operadores

| Categoría      | Operadores                         |
|----------------|------------------------------------|
| Aritméticos    | `+  -  *  /`                       |
| Comparación    | `==  !=  <  >  <=  >=`             |
| Lógicos        | `estoY` (y), `estoO` (o), `noNiNa` (no) |

- `+` entre texto y cualquier otro valor **concatena** (convierte a texto).
- `/` siempre devuelve decimal (`7 / 2` → `3.5`).
- `noNiNa` es unario y se aplica a la expresión primaria que le sigue: `noNiNa contento`, `noNiNa (a > b)`.

### Precedencia (¡importante!)
**No hay precedencia de operadores:** las expresiones se evalúan **de izquierda a derecha**. Usa paréntesis para agrupar.

```puya
chivato(2 + 3 * 4);     // 20, no 14
chivato(2 + (3 * 4));   // 14
```

---

## 6. Salida: `chivato`

```puya
chivato("Hola");
chivato(5 * 2);
chivato(3 > 1);   // verdá
```
Los booleanos se imprimen como `verdá` / `esIA`.

---

## 7. Condicionales: `siAcaso` / `siNo` / `oSiNo`

| Palabra   | Equivale a |
|-----------|------------|
| `siAcaso` | `if`       |
| `siNo`    | `else if`  |
| `oSiNo`   | `else`     |

```puya
siAcaso (nota >= 9) {
    chivato("Sobresaliente");
} siNo (nota >= 5) {
    chivato("Aprobado");
} oSiNo {
    chivato("Suspenso");
}
```
Las condiciones llevan paréntesis; las ramas siempre llevan llaves.

---

## 8. Bucles: `josear`

### Tipo `while`
```puya
guita i = 0;
josear (i < 5) {
    chivato(i);
    i = i + 1;
}
```

### Tipo `for`
```puya
josear (guita i = 0; i < 5; i = i + 1) {
    chivato(i);
}
```
- La inicialización debe empezar por un tipo (`guita`, `suelto`, `palique` o `fijo`).
- El paso es una asignación (`i = i + 1`); no existe `i++`.

### Control de bucle
| Palabra        | Equivale a | Efecto |
|----------------|-----------|--------|
| `alCarajoPipa` | `break`    | Sale del bucle (o del `arteCaso`) |
| `tiraPalante`  | `continue` | Salta a la siguiente iteración    |

```puya
josear (guita i = 0; i < 10; i = i + 1) {
    siAcaso (i == 3) { tiraPalante; }
    siAcaso (i == 7) { alCarajoPipa; }
    chivato(i);
}
```

---

## 9. Selección múltiple: `arteCaso` / `caso`

Equivale a `switch`. **Tiene fall-through**: una vez que un `caso` coincide, se ejecutan los siguientes hasta encontrar `alCarajoPipa`. No existe `default`.

```puya
guita dia = 2;
arteCaso (dia) {
    caso 1:
        chivato("Lunes");
        alCarajoPipa;
    caso 2:
        chivato("Martes");
        alCarajoPipa;
    caso 3:
        chivato("Miércoles");
        alCarajoPipa;
}
```

---

## 10. Funciones: `movida` y `suelta`

```puya
movida nombre(tipo param1, tipo param2) -> tipoRetorno {
    ...
    suelta valor;
}
```

- `suelta` devuelve un valor (equivale a `return`).
- `-> tipo` es opcional; si se omite, es `na`. El tipo de retorno no se comprueba.
- Se pueden llamar como expresión o como sentencia (`saluda("Ana");`).
- Se admite recursividad.

```puya
movida suma(guita a, guita b) -> guita {
    suelta a + b;
}

movida factorial(guita n) -> guita {
    siAcaso (n <= 1) { suelta 1; }
    suelta n * factorial(n - 1);
}

movida saluda(palique quien) {
    chivato("Quillo, " + quien);
}

chivato(suma(2, 3));       // 5
chivato(factorial(5));     // 120
saluda("Paco");
```

La función debe haberse **declarado antes** de ejecutarse la llamada (las declaraciones se registran al ejecutarse, no al leer el archivo).

---

## 11. Ámbito de variables

- Las variables globales son visibles desde las funciones.
- Cada llamada a función crea una memoria local con sus parámetros.
- **No hay ámbito de bloque**: una variable declarada dentro de un `siAcaso` o `josear` pertenece a la función (o al programa) que lo contiene.
- La búsqueda de variables recorre la pila de llamadas, por lo que una función también puede ver las variables locales de quien la llamó (ámbito dinámico).

---

## 12. Palabras reservadas

**En uso:**
`guita`, `suelto`, `palique`, `fijo`, `na`, `verdá`, `esIA`, `chivato`, `siAcaso`, `siNo`, `oSiNo`, `josear`, `alCarajoPipa`, `tiraPalante`, `arteCaso`, `caso`, `movida`, `suelta`, `estoY`, `estoO`, `noNiNa`

**Reservadas sin uso actual** (reservadas para futuras versiones):
`trincar`, `banda`, `churumbel`, `poriera`, `esto`, `intocable`, `mamaDe`, `privao`, `palPueblo`, `roña`, `peña`

---

## 13. Gramática resumida

```
programa     := sentencia*
sentencia    := declVar | asignacion | llamada ';' | chivato | siAcaso
              | josear | arteCaso | movida | suelta | alCarajoPipa ';'
              | tiraPalante ';'
declVar      := tipo ID '=' expresion ';'
asignacion   := ID '=' expresion ';'
chivato      := 'chivato' '(' expresion ')' ';'
siAcaso      := 'siAcaso' '(' expr ')' bloque
                ( 'siNo' '(' expr ')' bloque )* ( 'oSiNo' bloque )?
josear       := 'josear' '(' expr ')' bloque
              | 'josear' '(' tipo ID '=' expr ';' expr ';' ID '=' expr ')' bloque
arteCaso     := 'arteCaso' '(' expr ')' '{' ( 'caso' expr ':' sentencia* )* '}'
movida       := 'movida' ID '(' (tipo ID (',' tipo ID)*)? ')' ('->' tipo)? bloque
suelta       := 'suelta' expresion ';'
bloque       := '{' sentencia* '}'
expresion    := primaria ( operador primaria )*      // izquierda a derecha
primaria     := NUMERO | TEXTO | 'verdá' | 'esIA' | ID | ID '(' args ')'
              | '(' expresion ')' | 'noNiNa' primaria
```

---

## 14. Limitaciones conocidas

- **Sin números negativos literales:** `-5` da error. Usa `0 - 5`.
- **Sin precedencia de operadores** (ver sección 5).
- **Sin `default`** en `arteCaso`.
- **Sin entrada de usuario**, listas, objetos ni librería estándar (solo `chivato` para imprimir).
- **`suelta` fuera de una función** produce un `Error fatal:` con mensaje vacío.
- Al concatenar un `fijo` con texto, se muestra `True`/`False` (de Python) en vez de `verdá`/`esIA`.
- La división por cero y otros fallos de tipo (p. ej. restar un texto) muestran el error interno de Python tras `Error fatal:`.
- Los errores no indican línea ni columna.

---

## 15. Ejemplo completo

```puya
// Tabla de multiplicar y números pares
movida tabla(guita n) {
    josear (guita i = 1; i <= 10; i = i + 1) {
        chivato(n + " x " + i + " = " + (n * i));
    }
}

tabla(7);

josear (guita i = 0; i < 10; i = i + 1) {
    siAcaso ((i / 2) * 2 == i) {
        chivato(i + " es par");
    } oSiNo {
        chivato(i + " es impar");
    }
}
```
