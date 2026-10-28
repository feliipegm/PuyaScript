#!/usr/bin/env python3
import sys
from typing import List, Any

# --- 1. LEXER ---
class Token:
    def __init__(self, tipo: str, valor: Any = None):
        self.tipo = tipo
        self.valor = valor
    def __repr__(self):
        return f"Token({self.tipo}" + (f", '{self.valor}')" if self.valor is not None else ")")

PALABRAS_RESERVADAS = {
    'trincar', 'banda', 'churumbel', 'poriera', 'movida', 'suelta', 'chivato',
    'siAcaso', 'siNo', 'oSiNo', 'esto', 'josear', 'arteCaso', 'caso', 'alCarajoPipa',
    'noNiNa', 'estoY', 'estoO', 'intocable', 'mamaDe', 'privao', 'palPueblo',
    'roña', 'tiraPalante', 'verdá', 'esIA', 'guita', 'suelto', 'palique', 'fijo', 'na', 'peña'
}

class Lexer:
    def __init__(self, codigo: str):
        self.codigo = codigo
        self.pos = 0
        self.caracter_actual = self.codigo[self.pos] if self.codigo else None

    def avanzar(self):
        self.pos += 1
        self.caracter_actual = self.codigo[self.pos] if self.pos < len(self.codigo) else None

    def ver_siguiente(self):
        siguiente_pos = self.pos + 1
        return self.codigo[siguiente_pos] if siguiente_pos < len(self.codigo) else None

    def saltar_espacios_y_comentarios(self):
        while self.caracter_actual is not None:
            if self.caracter_actual.isspace():
                self.avanzar()
            elif self.caracter_actual == '/' and self.ver_siguiente() == '/':
                while self.caracter_actual is not None and self.caracter_actual != '\n':
                    self.avanzar()
            else:
                break

    def procesar_numero(self) -> Token:
        numero_str = ''
        puntos = 0
        while self.caracter_actual is not None and self.caracter_actual in '0123456789.':
            if self.caracter_actual == '.':
                if puntos == 1: break
                puntos += 1
            numero_str += self.caracter_actual
            self.avanzar()
        if puntos == 0:
            return Token('NUMERO_GUITA', int(numero_str))
        return Token('NUMERO_SUELTO', float(numero_str))

    def procesar_identificador(self) -> Token:
        id_str = ''
        while self.caracter_actual is not None and (self.caracter_actual.isalnum() or self.caracter_actual == '_'):
            id_str += self.caracter_actual
            self.avanzar()
        if id_str in PALABRAS_RESERVADAS:
            return Token('KEYWORD', id_str)
        return Token('IDENTIFICADOR', id_str)

    def procesar_texto(self) -> Token:
        texto_str = ''
        self.avanzar()
        while self.caracter_actual is not None and self.caracter_actual != '"':
            texto_str += self.caracter_actual
            self.avanzar()
        self.avanzar()
        return Token('PALIQUE', texto_str)

    def tokenizar(self) -> List[Token]:
        tokens = []
        while self.caracter_actual is not None:
            self.saltar_espacios_y_comentarios()
            if self.caracter_actual is None: break

            if self.caracter_actual.isdigit():
                tokens.append(self.procesar_numero())
                continue
            if self.caracter_actual.isalpha() or self.caracter_actual == '_':
                tokens.append(self.procesar_identificador())
                continue
            if self.caracter_actual == '"':
                tokens.append(self.procesar_texto())
                continue

            sig = self.ver_siguiente()
            if self.caracter_actual == '-' and sig == '>':
                tokens.append(Token('FLECHA', '->'))
                self.avanzar(); self.avanzar(); continue
            if self.caracter_actual == '=' and sig == '=':
                tokens.append(Token('OPERADOR', '=='))
                self.avanzar(); self.avanzar(); continue
            if self.caracter_actual == '!' and sig == '=':
                tokens.append(Token('OPERADOR', '!='))
                self.avanzar(); self.avanzar(); continue
            if self.caracter_actual == '<' and sig == '=':
                tokens.append(Token('OPERADOR', '<='))
                self.avanzar(); self.avanzar(); continue
            if self.caracter_actual == '>' and sig == '=':
                tokens.append(Token('OPERADOR', '>='))
                self.avanzar(); self.avanzar(); continue

            simbolos = {
                '+': 'OPERADOR', '-': 'OPERADOR', '*': 'OPERADOR', '/': 'OPERADOR',
                '<': 'OPERADOR', '>': 'OPERADOR', '=': 'ASIGNA',
                ';': 'PUNTO_COMA', '(': 'PAREN_IZQ', ')': 'PAREN_DER',
                '{': 'LLAVE_IZQ', '}': 'LLAVE_DER', ',': 'COMA', '.': 'PUNTO',
                ':': 'DOS_PUNTOS'
            }
            if self.caracter_actual in simbolos:
                tokens.append(Token(simbolos[self.caracter_actual], self.caracter_actual))
                self.avanzar()
                continue
            raise Exception(f"¡Poriera! Carácter ilegal: '{self.caracter_actual}'")
        tokens.append(Token('EOF'))
        return tokens

# --- 2. NODOS AST ---
class ASTNode: pass
class NodoPrograma(ASTNode):
    def __init__(self, sentencias): self.sentencias = sentencias
class NodoBloque(ASTNode):
    def __init__(self, sentencias): self.sentencias = sentencias
class NodoDeclVar(ASTNode):
    def __init__(self, tipo: str, nombre: str, expresion: ASTNode):
        self.tipo = tipo; self.nombre = nombre; self.expresion = expresion
class NodoAsignacion(ASTNode):
    def __init__(self, nombre: str, expresion: ASTNode):
        self.nombre = nombre; self.expresion = expresion
class NodoSiAcaso(ASTNode):
    def __init__(self, condicion: ASTNode, bloque_si: ASTNode, bloque_sino: ASTNode = None):
        self.condicion = condicion; self.bloque_si = bloque_si; self.bloque_sino = bloque_sino
class NodoJosearWhile(ASTNode):
    def __init__(self, condicion: ASTNode, bloque: ASTNode):
        self.condicion = condicion; self.bloque = bloque
class NodoJosearFor(ASTNode):
    def __init__(self, init: ASTNode, cond: ASTNode, paso: ASTNode, bloque: ASTNode):
        self.init = init; self.cond = cond; self.paso = paso; self.bloque = bloque
class NodoArteCaso(ASTNode):
    def __init__(self, expresion: ASTNode, casos: list):
        self.expresion = expresion; self.casos = casos
class NodoBreak(ASTNode): pass
class NodoContinue(ASTNode): pass
class NodoChivato(ASTNode):
    def __init__(self, expresion: ASTNode): self.expresion = expresion
class NodoOpBinaria(ASTNode):
    def __init__(self, izq: ASTNode, op: str, der: ASTNode):
        self.izq = izq; self.op = op; self.der = der
class NodoOpUnaria(ASTNode):
    def __init__(self, op: str, expresion: ASTNode):
        self.op = op; self.expresion = expresion
class NodoVariable(ASTNode):
    def __init__(self, nombre: str): self.nombre = nombre
class NodoLiteral(ASTNode):
    def __init__(self, valor, tipo: str): self.valor = valor; self.tipo = tipo
class NodoDeclFuncion(ASTNode):
    def __init__(self, nombre: str, parametros: List[tuple], tipo_retorno: str, bloque: NodoBloque):
        self.nombre = nombre; self.parametros = parametros; self.tipo_retorno = tipo_retorno; self.bloque = bloque
class NodoLlamadaFuncion(ASTNode):
    def __init__(self, nombre: str, argumentos: List[ASTNode]):
        self.nombre = nombre; self.argumentos = argumentos
class NodoSuelta(ASTNode):
    def __init__(self, expresion: ASTNode): self.expresion = expresion

class RetornoExcepcion(Exception):
    def __init__(self, valor): self.valor = valor
class BreakExcepcion(Exception): pass
class ContinueExcepcion(Exception): pass

# --- 3. PARSER ---
class Parser:
    def __init__(self, tokens):
        self.tokens = tokens
        self.pos = 0
        self.token_actual = self.tokens[self.pos] if self.tokens else None

    def consumir(self, tipo_esperado, valor_esperado=None):
        if self.token_actual.tipo == tipo_esperado:
            if valor_esperado is not None and self.token_actual.valor != valor_esperado:
                raise Exception(f"¡Poriera! Se esperaba '{valor_esperado}', pero se encontró '{self.token_actual.valor}'")
            tok = self.token_actual
            self.pos += 1
            self.token_actual = self.tokens[self.pos] if self.pos < len(self.tokens) else None
            return tok
        raise Exception(f"¡Poriera! Se esperaba {tipo_esperado}, pero se encontró {self.token_actual}")

    def parse_expresion_primaria(self) -> ASTNode:
        tok = self.token_actual
        if tok.tipo == 'NUMERO_GUITA':
            self.consumir('NUMERO_GUITA')
            return NodoLiteral(tok.valor, 'guita')
        if tok.tipo == 'NUMERO_SUELTO':
            self.consumir('NUMERO_SUELTO')
            return NodoLiteral(tok.valor, 'suelto')
        if tok.tipo == 'PALIQUE':
            self.consumir('PALIQUE')
            return NodoLiteral(tok.valor, 'palique')
        if tok.tipo == 'KEYWORD' and tok.valor in ['verdá', 'esIA']:
            self.consumir('KEYWORD')
            return NodoLiteral(tok.valor == 'verdá', 'fijo')
        if tok.tipo == 'KEYWORD' and tok.valor == 'noNiNa':
            self.consumir('KEYWORD')
            return NodoOpUnaria('noNiNa', self.parse_expresion_primaria())
        if tok.tipo == 'IDENTIFICADOR':
            nombre = self.consumir('IDENTIFICADOR').valor
            if self.token_actual and self.token_actual.tipo == 'PAREN_IZQ':
                self.consumir('PAREN_IZQ')
                args = []
                if self.token_actual.tipo != 'PAREN_DER':
                    args.append(self.parse_expresion())
                    while self.token_actual and self.token_actual.tipo == 'COMA':
                        self.consumir('COMA')
                        args.append(self.parse_expresion())
                self.consumir('PAREN_DER')
                return NodoLlamadaFuncion(nombre, args)
            return NodoVariable(nombre)
        if tok.tipo == 'PAREN_IZQ':
            self.consumir('PAREN_IZQ')
            expr = self.parse_expresion()
            self.consumir('PAREN_DER')
            return expr
        raise Exception(f"¡Poriera! Expresión no reconocida: {tok}")

    def parse_expresion(self) -> ASTNode:
        izq = self.parse_expresion_primaria()
        while self.token_actual and (self.token_actual.tipo == 'OPERADOR' or 
            (self.token_actual.tipo == 'KEYWORD' and self.token_actual.valor in ['estoY', 'estoO'])):
            op_tok = self.token_actual
            self.consumir(op_tok.tipo)
            der = self.parse_expresion_primaria()
            izq = NodoOpBinaria(izq, op_tok.valor, der)
        return izq

    def parse_bloque(self) -> NodoBloque:
        self.consumir('LLAVE_IZQ')
        sentencias = []
        while self.token_actual and self.token_actual.tipo != 'LLAVE_DER' and self.token_actual.tipo != 'EOF':
            sentencias.append(self.parse_sentencia())
        self.consumir('LLAVE_DER')
        return NodoBloque(sentencias)

    def parse_si_acaso(self) -> NodoSiAcaso:
        self.consumir('KEYWORD', 'siAcaso')
        self.consumir('PAREN_IZQ')
        condicion = self.parse_expresion()
        self.consumir('PAREN_DER')
        bloque_si = self.parse_bloque()
        return NodoSiAcaso(condicion, bloque_si, self.parse_sino_rest())

    def parse_sino_rest(self) -> ASTNode:
        if self.token_actual and self.token_actual.tipo == 'KEYWORD' and self.token_actual.valor == 'siNo':
            self.consumir('KEYWORD', 'siNo')
            self.consumir('PAREN_IZQ')
            cond = self.parse_expresion()
            self.consumir('PAREN_DER')
            return NodoBloque([NodoSiAcaso(cond, self.parse_bloque(), self.parse_sino_rest())])
        elif self.token_actual and self.token_actual.tipo == 'KEYWORD' and self.token_actual.valor == 'oSiNo':
            self.consumir('KEYWORD', 'oSiNo')
            return self.parse_bloque()
        return None

    def parse_josear(self) -> ASTNode:
        self.consumir('KEYWORD', 'josear')
        self.consumir('PAREN_IZQ')
        if self.token_actual.tipo == 'KEYWORD' and self.token_actual.valor in ['guita', 'suelto', 'palique', 'fijo']:
            tipo = self.consumir('KEYWORD').valor
            nombre = self.consumir('IDENTIFICADOR').valor
            self.consumir('ASIGNA')
            init = NodoDeclVar(tipo, nombre, self.parse_expresion())
            self.consumir('PUNTO_COMA')
            cond = self.parse_expresion()
            self.consumir('PUNTO_COMA')
            nombre_paso = self.consumir('IDENTIFICADOR').valor
            self.consumir('ASIGNA')
            paso = NodoAsignacion(nombre_paso, self.parse_expresion())
            self.consumir('PAREN_DER')
            return NodoJosearFor(init, cond, paso, self.parse_bloque())
        else:
            cond = self.parse_expresion()
            self.consumir('PAREN_DER')
            return NodoJosearWhile(cond, self.parse_bloque())

    def parse_arte_caso(self) -> NodoArteCaso:
        self.consumir('KEYWORD', 'arteCaso')
        self.consumir('PAREN_IZQ')
        expr = self.parse_expresion()
        self.consumir('PAREN_DER')
        self.consumir('LLAVE_IZQ')
        casos = []
        while self.token_actual and self.token_actual.tipo == 'KEYWORD' and self.token_actual.valor == 'caso':
            self.consumir('KEYWORD')
            val_caso = self.parse_expresion()
            self.consumir('DOS_PUNTOS')
            sentencias = []
            while self.token_actual and not (self.token_actual.tipo == 'KEYWORD' and self.token_actual.valor == 'caso') and self.token_actual.tipo != 'LLAVE_DER':
                sentencias.append(self.parse_sentencia())
            casos.append((val_caso, NodoBloque(sentencias)))
        self.consumir('LLAVE_DER')
        return NodoArteCaso(expr, casos)

    def parse_movida(self) -> NodoDeclFuncion:
        self.consumir('KEYWORD', 'movida')
        nombre = self.consumir('IDENTIFICADOR').valor
        self.consumir('PAREN_IZQ')
        params = []
        if self.token_actual and self.token_actual.tipo == 'KEYWORD' and self.token_actual.valor in ['guita', 'suelto', 'palique', 'fijo']:
            params.append((self.consumir('KEYWORD').valor, self.consumir('IDENTIFICADOR').valor))
            while self.token_actual and self.token_actual.tipo == 'COMA':
                self.consumir('COMA')
                params.append((self.consumir('KEYWORD').valor, self.consumir('IDENTIFICADOR').valor))
        self.consumir('PAREN_DER')
        tipo_retorno = 'na'
        if self.token_actual and self.token_actual.tipo == 'FLECHA':
            self.consumir('FLECHA')
            tipo_retorno = self.consumir('KEYWORD').valor
        return NodoDeclFuncion(nombre, params, tipo_retorno, self.parse_bloque())

    def parse_sentencia(self) -> ASTNode:
        tok = self.token_actual
        if tok.tipo == 'KEYWORD':
            if tok.valor in ['guita', 'suelto', 'palique', 'fijo']:
                tipo = self.consumir('KEYWORD').valor
                nombre = self.consumir('IDENTIFICADOR').valor
                self.consumir('ASIGNA')
                exp = self.parse_expresion()
                self.consumir('PUNTO_COMA')
                return NodoDeclVar(tipo, nombre, exp)
            if tok.valor == 'chivato':
                self.consumir('KEYWORD')
                self.consumir('PAREN_IZQ')
                exp = self.parse_expresion()
                self.consumir('PAREN_DER')
                self.consumir('PUNTO_COMA')
                return NodoChivato(exp)
            if tok.valor == 'alCarajoPipa':
                self.consumir('KEYWORD')
                self.consumir('PUNTO_COMA')
                return NodoBreak()
            if tok.valor == 'tiraPalante':
                self.consumir('KEYWORD')
                self.consumir('PUNTO_COMA')
                return NodoContinue()
            if tok.valor == 'arteCaso': return self.parse_arte_caso()
            if tok.valor == 'siAcaso': return self.parse_si_acaso()
            if tok.valor == 'josear': return self.parse_josear()
            if tok.valor == 'movida': return self.parse_movida()
            if tok.valor == 'suelta':
                self.consumir('KEYWORD')
                exp = self.parse_expresion()
                self.consumir('PUNTO_COMA')
                return NodoSuelta(exp)

        if tok.tipo == 'IDENTIFICADOR':
            nombre = self.consumir('IDENTIFICADOR').valor
            if self.token_actual and self.token_actual.tipo == 'PAREN_IZQ':
                self.consumir('PAREN_IZQ')
                args = []
                if self.token_actual.tipo != 'PAREN_DER':
                    args.append(self.parse_expresion())
                    while self.token_actual and self.token_actual.tipo == 'COMA':
                        self.consumir('COMA')
                        args.append(self.parse_expresion())
                self.consumir('PAREN_DER')
                self.consumir('PUNTO_COMA')
                return NodoLlamadaFuncion(nombre, args)
            self.consumir('ASIGNA')
            exp = self.parse_expresion()
            self.consumir('PUNTO_COMA')
            return NodoAsignacion(nombre, exp)
        raise Exception(f"¡Poriera! Sentencia desconocida arrancando en: {tok}")

    def parsear(self) -> NodoPrograma:
        sentencias = []
        while self.token_actual and self.token_actual.tipo != 'EOF':
            sentencias.append(self.parse_sentencia())
        return NodoPrograma(sentencias)

# --- 4. INTÉRPRETE ---
class Interprete:
    def __init__(self):
        self.memoria_global = {}
        self.funciones = {}
        self.pila_memoria = [self.memoria_global]

    @property
    def memoria_actual(self):
        return self.pila_memoria[-1]

    def evaluar_expresion(self, nodo: ASTNode) -> Any:
        if isinstance(nodo, NodoLiteral): return nodo.valor
        if isinstance(nodo, NodoVariable):
            for mem in reversed(self.pila_memoria):
                if nodo.nombre in mem: return mem[nodo.nombre]
            raise Exception(f"¡Poriera! La variable '{nodo.nombre}' no existe.")
        if isinstance(nodo, NodoOpUnaria):
            val = self.evaluar_expresion(nodo.expresion)
            if nodo.op == 'noNiNa': return not bool(val)
        if isinstance(nodo, NodoLlamadaFuncion):
            if nodo.nombre not in self.funciones:
                raise Exception(f"¡Poriera! La movida '{nodo.nombre}' no está definida.")
            func = self.funciones[nodo.nombre]
            valores_args = [self.evaluar_expresion(arg) for arg in nodo.argumentos]
            memoria_local = {nom_p: val for (tipo_p, nom_p), val in zip(func.parametros, valores_args)}
            self.pila_memoria.append(memoria_local)
            resultado = None
            try: self.ejecutar_bloque(func.bloque.sentencias)
            except RetornoExcepcion as ret: resultado = ret.valor
            finally: self.pila_memoria.pop()
            return resultado
        if isinstance(nodo, NodoOpBinaria):
            izq = self.evaluar_expresion(nodo.izq)
            der = self.evaluar_expresion(nodo.der)
            if nodo.op == '+': return str(izq) + str(der) if isinstance(izq, str) or isinstance(der, str) else izq + der
            if nodo.op == '-': return izq - der
            if nodo.op == '*': return izq * der
            if nodo.op == '/': return izq / der
            if nodo.op == '<': return izq < der
            if nodo.op == '>': return izq > der
            if nodo.op == '<=': return izq <= der
            if nodo.op == '>=': return izq >= der
            if nodo.op == '==': return izq == der
            if nodo.op == '!=': return izq != der
            if nodo.op == 'estoY': return bool(izq and der)
            if nodo.op == 'estoO': return bool(izq or der)
        raise Exception(f"Expresión no soportada.")

    def ejecutar_bloque(self, sentencias: List[ASTNode]):
        for sentencia in sentencias: self.ejecutar_sentencia(sentencia)

    def ejecutar_sentencia(self, nodo: ASTNode):
        if isinstance(nodo, NodoDeclFuncion): self.funciones[nodo.nombre] = nodo
        elif isinstance(nodo, NodoDeclVar): self.memoria_actual[nodo.nombre] = self.evaluar_expresion(nodo.expresion)
        elif isinstance(nodo, NodoAsignacion):
            encontrado = False
            for mem in reversed(self.pila_memoria):
                if nodo.nombre in mem:
                    mem[nodo.nombre] = self.evaluar_expresion(nodo.expresion)
                    encontrado = True
                    break
            if not encontrado: raise Exception(f"¡Poriera! No puedes asignar a '{nodo.nombre}'.")
        elif isinstance(nodo, NodoChivato):
            v = self.evaluar_expresion(nodo.expresion)
            print("verdá" if isinstance(v, bool) and v else "esIA" if isinstance(v, bool) else v)
        elif isinstance(nodo, NodoSuelta): raise RetornoExcepcion(self.evaluar_expresion(nodo.expresion))
        elif isinstance(nodo, NodoBreak): raise BreakExcepcion()
        elif isinstance(nodo, NodoContinue): raise ContinueExcepcion()
        elif isinstance(nodo, NodoLlamadaFuncion): self.evaluar_expresion(nodo)
        elif isinstance(nodo, NodoSiAcaso):
            if self.evaluar_expresion(nodo.condicion): self.ejecutar_bloque(nodo.bloque_si.sentencias)
            elif nodo.bloque_sino:
                if isinstance(nodo.bloque_sino, NodoBloque): self.ejecutar_bloque(nodo.bloque_sino.sentencias)
                elif isinstance(nodo.bloque_sino, ASTNode): self.ejecutar_sentencia(nodo.bloque_sino)
        elif isinstance(nodo, NodoArteCaso):
            val_match = self.evaluar_expresion(nodo.expresion)
            cae_en_casos = False
            for val_caso_nodo, bloque in nodo.casos:
                if not cae_en_casos and val_match == self.evaluar_expresion(val_caso_nodo):
                    cae_en_casos = True
                if cae_en_casos:
                    try: self.ejecutar_bloque(bloque.sentencias)
                    except BreakExcepcion: break
        elif isinstance(nodo, NodoJosearWhile):
            while self.evaluar_expresion(nodo.condicion):
                try: self.ejecutar_bloque(nodo.bloque.sentencias)
                except BreakExcepcion: break
                except ContinueExcepcion: continue
        elif isinstance(nodo, NodoJosearFor):
            self.ejecutar_sentencia(nodo.init)
            while self.evaluar_expresion(nodo.cond):
                try: self.ejecutar_bloque(nodo.bloque.sentencias)
                except BreakExcepcion: break
                except ContinueExcepcion: pass
                self.ejecutar_sentencia(nodo.paso)

    def ejecutar(self, ast: NodoPrograma): self.ejecutar_bloque(ast.sentencias)

def main():
    if len(sys.argv) < 2:
        print("Uso: puyascript <archivo.puya>")
        sys.exit(1)
    archivo = sys.argv[1]
    if not archivo.endswith('.puya'):
        print("¡Poriera! El archivo tiene que ser .puya")
        sys.exit(1)
    try:
        with open(archivo, 'r', encoding='utf-8') as f: codigo = f.read()
        lexer = Lexer(codigo)
        tokens = lexer.tokenizar()
        parser = Parser(tokens)
        ast = parser.parsear()
        interprete = Interprete()
        interprete.ejecutar(ast)
    except Exception as e:
        print(f"Error fatal: {e}")

if __name__ == '__main__':
    main()
