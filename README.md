# PuyaScript
![PuyaScript](logo/puyaLogo.png)
Lenguaje de programación de sintaxis andaluza. Hay dos intérpretes equivalentes:
`src/puyascript.rs` (Rust, el principal) y `src/puyascript.py` (Python).

## Instalar (Linux x86_64)

    git clone https://github.com/feliipegm/PuyaScript.git
    cd PuyaScript
    bash instalador/instalar-puyascript.sh

Abre una terminal nueva y ejecuta:

    puyascript ejemplos/prueba.puya

## Con Python

    python3 src/puyascript.py ejemplos/prueba.puya

## Compilar desde el código (Rust)

    rustc -O src/puyascript.rs -o puyascript

## Documentación

Mira `docs/DOCUMENTACION_PUYASCRIPT.md`.
