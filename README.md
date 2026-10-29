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

## Instalar (Windows x64)
    Para instalar PuyaScript desde el código fuente, necesitas tener instalado **Rust y Cargo** en tu equipo.

    1. Clona el repositorio:
    ```bash
    git clone [https://github.com/feliipegm/PuyaScript.git](https://github.com/feliipegm/PuyaScript.git)
    cd PuyaScript
    Ejecutar en PuyasScript/src el siguiente comando:
    rustc puyascript.rs -O
    copy src/instalador/puyascript.exe /instalador o mover el .exe a la carpeta instalador
    Luego ejecutar 
        instalador/instalar-puyascript-windows.bat
    Una vez instalado, reiniciar el terminal o el IDE.```

## Con Python

    python3 src/puyascript.py ejemplos/prueba.puya

## Compilar desde el código (Rust)

    rustc -O src/puyascript.rs -o puyascript

## Documentación

[Enlace a la documentación](/docs/DOCUMENTACION_PUYASCRIPT.md)
