#!/usr/bin/env python3
import sys
import operator
from typing import List, Any

sys.setrecursionlimit(10000)

TIPOS = ('guita', 'suelto', 'palique', 'fijo')

# --- 1. LEXER ---
class Token:
    def __init__(self, tipo: str, valor: Any = None, linea: int = 0):
        self.tipo = tipo
        self.valor = valor
        self.linea = linea
    def __repr__(self):
        return f"Token({self.tipo}" + (f", '{self.valor}')" if self.valor is not None else ")")

PALABRAS_RESERVADAS = {
    'trincar', 'banda', 'churumbel', 'poriera', 'movida', 'suelta', 'chivato',
    'siAcaso', 'siNo', 'oSiNo', 'esto', 'josear', 'arteCaso', 'caso', 'alCarajoPipa',
    'noNiNa', 'estoY', 'estoO', 'intocable', 'mamaDe', 'privao', 'palPueblo',
    'roña', 'tiraPalante', 'verdá', 'esIA', 'guita', 'suelto', 'palique', 'fijo', 'na', 'peña'
}

OPERADORES_DOBLES = {'->': 'FLECHA', '==': 'OPERADOR', '!=': 'OPERADOR', '<=': 'OPERADOR', '>=': 'OPERADOR'}
SIMBOLOS = {
    '+': 'OPERADOR', '-': 'OPERADOR', '*': 'OPERADOR', '/': 'OPERADOR',
    '<': 'OPERADOR', '>': 'OPERADOR', '=': 'ASIGNA',
    ';': 'PUNTO_COMA', '(': 'PAREN_IZQ', ')': 'PAREN_DER',
    '{': 'LLAVE_IZQ', '}': 'LLAVE_DER', ',': 'COMA', '.': 'PUNTO',
    ':': 'DOS_PUNTOS'
}

class Lexer:
    def __init__(self, codigo: str):
        self.codigo = codigo
        self.pos = 0
        self.linea = 1
        self.caracter_actual = self.codigo[self.pos] if self.codigo else None

    def avanzar(self):
        if self.caracter_actual == '\n':
            self.linea += 1
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
        while self.caracter_actual is not None and (self.caracter_actual.isdigit() or self.caracter_actual == '.'):
            if self.caracter_actual == '.':
                sig = self.ver_siguiente()
                if puntos == 1 or sig is None or not sig.isdigit():
                    break
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
        linea_inicio = self.linea
        partes = []
        self.avanzar()
        while self.caracter_actual is not None and self.caracter_actual != '"':
            partes.append(self.caracter_actual)
            self.avanzar()
        if self.caracter_actual is None:
            raise Exception(f"¡Poriera! (línea {linea_inicio}) Texto sin cerrar: falta la comilla final.")
        self.avanzar()
        return Token('PALIQUE', ''.join(partes))

    def tokenizar(self) -> List[Token]:
        tokens = []
        while True:
            self.saltar_espacios_y_comentarios()
            if self.caracter_actual is None:
                break
            c = self.caracter_actual
            linea = self.linea

            if c.isdigit():
                tok = self.procesar_numero()
            elif c.isalpha() or c == '_':
                tok = self.procesar_identificador()
            elif c == '"':
                tok = self.procesar_texto()
            else:
                doble = c + (self.ver_siguiente() or '')
                if doble in OPERADORES_DOBLES:
                    tok = Token(OPERADORES_DOBLES[doble], doble)
                    self.avanzar(); self.avanzar()
                elif c in SIMBOLOS:
                    tok = Token(SIMBOLOS[c], c)
                    self.avanzar()
                else:
                    raise Exception(f"¡Poriera! (línea {linea}) Carácter ilegal: '{c}'")
            tok.linea = linea
            tokens.append(tok)
        tokens.append(Token('EOF', None, self.linea))
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
PRECEDENCIA = {
    'estoO': 1, 'estoY': 2,
    '==': 3, '!=': 3,
    '<': 4, '>': 4, '<=': 4, '>=': 4,
    '+': 5, '-': 5,
    '*': 6, '/': 6,
}

class Parser:
    def __init__(self, tokens):
        self.tokens = tokens
        self.pos = 0
        self.token_actual = self.tokens[self.pos]

    def error(self, mensaje):
        return Exception(f"¡Poriera! (línea {self.token_actual.linea}) {mensaje}")

    def consumir(self, tipo_esperado, valor_esperado=None):
        tok = self.token_actual
        if tok.tipo != tipo_esperado:
            raise self.error(f"Se esperaba {tipo_esperado}, pero se encontró {tok}")
        if valor_esperado is not None and tok.valor != valor_esperado:
            raise self.error(f"Se esperaba '{valor_esperado}', pero se encontró '{tok.valor}'")
        if self.pos < len(self.tokens) - 1:
            self.pos += 1
            self.token_actual = self.tokens[self.pos]
        return tok

    def es_keyword(self, *valores) -> bool:
        return self.token_actual.tipo == 'KEYWORD' and self.token_actual.valor in valores

    def parse_tipo(self, permitir_na=False) -> str:
        validos = TIPOS + ('na',) if permitir_na else TIPOS
        if self.es_keyword(*validos):
            return self.consumir('KEYWORD').valor
        raise self.error(f"Se esperaba un tipo ({', '.join(validos)}), pero se encontró {self.token_actual}")

    def parse_argumentos(self) -> List[ASTNode]:
        self.consumir('PAREN_IZQ')
        args = []
        if self.token_actual.tipo != 'PAREN_DER':
            args.append(self.parse_expresion())
            while self.token_actual.tipo == 'COMA':
                self.consumir('COMA')
                args.append(self.parse_expresion())
        self.consumir('PAREN_DER')
        return args

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
        if tok.tipo == 'KEYWORD' and tok.valor in ('verdá', 'esIA'):
            self.consumir('KEYWORD')
            return NodoLiteral(tok.valor == 'verdá', 'fijo')
        if tok.tipo == 'IDENTIFICADOR':
            nombre = self.consumir('IDENTIFICADOR').valor
            if self.token_actual.tipo == 'PAREN_IZQ':
                return NodoLlamadaFuncion(nombre, self.parse_argumentos())
            return NodoVariable(nombre)
        if tok.tipo == 'PAREN_IZQ':
            self.consumir('PAREN_IZQ')
            expr = self.parse_expresion()
            self.consumir('PAREN_DER')
            return expr
        raise self.error(f"Expresión no reconocida: {tok}")

    def parse_unaria(self) -> ASTNode:
        tok = self.token_actual
        if tok.tipo == 'KEYWORD' and tok.valor == 'noNiNa':
            self.consumir('KEYWORD')
            return NodoOpUnaria('noNiNa', self.parse_unaria())
        if tok.tipo == 'OPERADOR' and tok.valor == '-':
            self.consumir('OPERADOR')
            return NodoOpUnaria('-', self.parse_unaria())
        return self.parse_expresion_primaria()

    def operador_binario_actual(self):
        tok = self.token_actual
        if tok.tipo == 'OPERADOR' and tok.valor in PRECEDENCIA:
            return tok.valor
        if tok.tipo == 'KEYWORD' and tok.valor in ('estoY', 'estoO'):
            return tok.valor
        return None

    def parse_expresion(self, prec_minima: int = 1) -> ASTNode:
        izq = self.parse_unaria()
        while True:
            op = self.operador_binario_actual()
            if op is None or PRECEDENCIA[op] < prec_minima:
                break
            self.consumir(self.token_actual.tipo)
            der = self.parse_expresion(PRECEDENCIA[op] + 1)
            izq = NodoOpBinaria(izq, op, der)
        return izq

    def parse_bloque(self) -> NodoBloque:
        self.consumir('LLAVE_IZQ')
        sentencias = []
        while self.token_actual.tipo not in ('LLAVE_DER', 'EOF'):
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
        if self.es_keyword('siNo'):
            self.consumir('KEYWORD', 'siNo')
            self.consumir('PAREN_IZQ')
            cond = self.parse_expresion()
            self.consumir('PAREN_DER')
            bloque = self.parse_bloque()
            return NodoBloque([NodoSiAcaso(cond, bloque, self.parse_sino_rest())])
        if self.es_keyword('oSiNo'):
            self.consumir('KEYWORD', 'oSiNo')
            return self.parse_bloque()
        return None

    def parse_josear(self) -> ASTNode:
        self.consumir('KEYWORD', 'josear')
        self.consumir('PAREN_IZQ')
        if self.es_keyword(*TIPOS):
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
        while self.es_keyword('caso'):
            self.consumir('KEYWORD')
            val_caso = self.parse_expresion()
            self.consumir('DOS_PUNTOS')
            sentencias = []
            while not self.es_keyword('caso') and self.token_actual.tipo not in ('LLAVE_DER', 'EOF'):
                sentencias.append(self.parse_sentencia())
            casos.append((val_caso, NodoBloque(sentencias)))
        self.consumir('LLAVE_DER')
        return NodoArteCaso(expr, casos)

    def parse_movida(self) -> NodoDeclFuncion:
        self.consumir('KEYWORD', 'movida')
        nombre = self.consumir('IDENTIFICADOR').valor
        self.consumir('PAREN_IZQ')
        params = []
        if self.token_actual.tipo != 'PAREN_DER':
            params.append((self.parse_tipo(), self.consumir('IDENTIFICADOR').valor))
            while self.token_actual.tipo == 'COMA':
                self.consumir('COMA')
                params.append((self.parse_tipo(), self.consumir('IDENTIFICADOR').valor))
        self.consumir('PAREN_DER')
        tipo_retorno = 'na'
        if self.token_actual.tipo == 'FLECHA':
            self.consumir('FLECHA')
            tipo_retorno = self.parse_tipo(permitir_na=True)
        return NodoDeclFuncion(nombre, params, tipo_retorno, self.parse_bloque())

    def parse_sentencia(self) -> ASTNode:
        tok = self.token_actual
        if tok.tipo == 'KEYWORD':
            if tok.valor in TIPOS:
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
            if self.token_actual.tipo == 'PAREN_IZQ':
                args = self.parse_argumentos()
                self.consumir('PUNTO_COMA')
                return NodoLlamadaFuncion(nombre, args)
            self.consumir('ASIGNA')
            exp = self.parse_expresion()
            self.consumir('PUNTO_COMA')
            return NodoAsignacion(nombre, exp)
        raise self.error(f"Sentencia desconocida arrancando en: {tok}")

    def parsear(self) -> NodoPrograma:
        sentencias = []
        while self.token_actual.tipo != 'EOF':
            sentencias.append(self.parse_sentencia())
        return NodoPrograma(sentencias)

# --- 4. INTÉRPRETE ---
def formatear(valor) -> str:
    if isinstance(valor, bool):
        return 'verdá' if valor else 'esIA'
    if valor is None:
        return 'na'
    return str(valor)

def _suma(izq, der):
    if isinstance(izq, str) or isinstance(der, str):
        return formatear(izq) + formatear(der)
    return izq + der

OPERACIONES = {
    '+': _suma, '-': operator.sub, '*': operator.mul, '/': operator.truediv,
    '<': operator.lt, '>': operator.gt, '<=': operator.le, '>=': operator.ge,
    '==': operator.eq, '!=': operator.ne,
}

class Interprete:
    def __init__(self):
        self.memoria_global = {}
        self.funciones = {}
        self.pila_memoria = [self.memoria_global]
        self._expresiones = {
            NodoLiteral: self._ev_literal,
            NodoVariable: self._ev_variable,
            NodoOpUnaria: self._ev_unaria,
            NodoOpBinaria: self._ev_binaria,
            NodoLlamadaFuncion: self._ev_llamada,
        }
        self._sentencias = {
            NodoDeclFuncion: self._ej_decl_funcion,
            NodoDeclVar: self._ej_decl_var,
            NodoAsignacion: self._ej_asignacion,
            NodoChivato: self._ej_chivato,
            NodoSuelta: self._ej_suelta,
            NodoBreak: self._ej_break,
            NodoContinue: self._ej_continue,
            NodoLlamadaFuncion: self.evaluar_expresion,
            NodoSiAcaso: self._ej_si_acaso,
            NodoArteCaso: self._ej_arte_caso,
            NodoJosearWhile: self._ej_while,
            NodoJosearFor: self._ej_for,
        }

    @property
    def memoria_actual(self):
        return self.pila_memoria[-1]

    # --- Expresiones ---
    def evaluar_expresion(self, nodo: ASTNode) -> Any:
        manejador = self._expresiones.get(type(nodo))
        if manejador is None:
            raise Exception("¡Poriera! Expresión no soportada.")
        return manejador(nodo)

    def _ev_literal(self, nodo):
        return nodo.valor

    def _ev_variable(self, nodo):
        for mem in reversed(self.pila_memoria):
            if nodo.nombre in mem:
                return mem[nodo.nombre]
        raise Exception(f"¡Poriera! La variable '{nodo.nombre}' no existe.")

    def _ev_unaria(self, nodo):
        val = self.evaluar_expresion(nodo.expresion)
        if nodo.op == 'noNiNa':
            return not bool(val)
        if nodo.op == '-':
            try:
                return -val
            except TypeError:
                raise Exception("¡Poriera! No puedes cambiar de signo algo que no es un número.")
        raise Exception(f"¡Poriera! Operador unario '{nodo.op}' no soportado.")

    def _ev_binaria(self, nodo):
        op = nodo.op
        if op == 'estoY':
            return bool(self.evaluar_expresion(nodo.izq)) and bool(self.evaluar_expresion(nodo.der))
        if op == 'estoO':
            return bool(self.evaluar_expresion(nodo.izq)) or bool(self.evaluar_expresion(nodo.der))
        izq = self.evaluar_expresion(nodo.izq)
        der = self.evaluar_expresion(nodo.der)
        try:
            return OPERACIONES[op](izq, der)
        except ZeroDivisionError:
            raise Exception("¡Poriera! No se puede dividir entre cero.")
        except TypeError:
            raise Exception(f"¡Poriera! No puedes usar '{op}' con esos tipos de valor.")

    def _ev_llamada(self, nodo):
        func = self.funciones.get(nodo.nombre)
        if func is None:
            raise Exception(f"¡Poriera! La movida '{nodo.nombre}' no está definida.")
        if len(nodo.argumentos) != len(func.parametros):
            raise Exception(
                f"¡Poriera! La movida '{nodo.nombre}' espera {len(func.parametros)} argumento(s) "
                f"y se le pasaron {len(nodo.argumentos)}.")
        valores = [self.evaluar_expresion(arg) for arg in nodo.argumentos]
        memoria_local = {nom: val for (_, nom), val in zip(func.parametros, valores)}
        pila_previa = self.pila_memoria
        # La función solo ve sus variables locales y las globales.
        self.pila_memoria = [self.memoria_global, memoria_local]
        resultado = None
        try:
            self.ejecutar_bloque(func.bloque.sentencias)
        except RetornoExcepcion as ret:
            resultado = ret.valor
        except (BreakExcepcion, ContinueExcepcion):
            raise Exception("¡Poriera! 'alCarajoPipa' o 'tiraPalante' fuera de un bucle.")
        except RecursionError:
            raise Exception(f"¡Poriera! Demasiada recursión en la movida '{nodo.nombre}'.")
        finally:
            self.pila_memoria = pila_previa
        return resultado

    # --- Sentencias ---
    def ejecutar_bloque(self, sentencias: List[ASTNode]):
        for sentencia in sentencias:
            self.ejecutar_sentencia(sentencia)

    def ejecutar_sentencia(self, nodo: ASTNode):
        manejador = self._sentencias.get(type(nodo))
        if manejador is None:
            raise Exception("¡Poriera! Sentencia no soportada.")
        manejador(nodo)

    def _ej_decl_funcion(self, nodo):
        self.funciones[nodo.nombre] = nodo

    def _ej_decl_var(self, nodo):
        self.memoria_actual[nodo.nombre] = self.evaluar_expresion(nodo.expresion)

    def _ej_asignacion(self, nodo):
        valor = self.evaluar_expresion(nodo.expresion)
        for mem in reversed(self.pila_memoria):
            if nodo.nombre in mem:
                mem[nodo.nombre] = valor
                return
        raise Exception(f"¡Poriera! No puedes asignar a '{nodo.nombre}': no existe.")

    def _ej_chivato(self, nodo):
        print(formatear(self.evaluar_expresion(nodo.expresion)))

    def _ej_suelta(self, nodo):
        raise RetornoExcepcion(self.evaluar_expresion(nodo.expresion))

    def _ej_break(self, nodo):
        raise BreakExcepcion()

    def _ej_continue(self, nodo):
        raise ContinueExcepcion()

    def _ej_si_acaso(self, nodo):
        if self.evaluar_expresion(nodo.condicion):
            self.ejecutar_bloque(nodo.bloque_si.sentencias)
        elif nodo.bloque_sino:
            self.ejecutar_bloque(nodo.bloque_sino.sentencias)

    def _ej_arte_caso(self, nodo):
        val_match = self.evaluar_expresion(nodo.expresion)
        cae_en_casos = False
        try:
            for val_caso_nodo, bloque in nodo.casos:
                if not cae_en_casos and val_match == self.evaluar_expresion(val_caso_nodo):
                    cae_en_casos = True
                if cae_en_casos:
                    self.ejecutar_bloque(bloque.sentencias)
        except BreakExcepcion:
            pass

    def _ej_while(self, nodo):
        sentencias = nodo.bloque.sentencias
        while self.evaluar_expresion(nodo.condicion):
            try:
                self.ejecutar_bloque(sentencias)
            except BreakExcepcion:
                break
            except ContinueExcepcion:
                continue

    def _ej_for(self, nodo):
        sentencias = nodo.bloque.sentencias
        self.ejecutar_sentencia(nodo.init)
        while self.evaluar_expresion(nodo.cond):
            try:
                self.ejecutar_bloque(sentencias)
            except BreakExcepcion:
                break
            except ContinueExcepcion:
                pass
            self.ejecutar_sentencia(nodo.paso)

    def ejecutar(self, ast: NodoPrograma):
        # Las movidas del nivel principal se registran antes de ejecutar nada,
        # así se pueden llamar aunque estén declaradas más abajo.
        for sentencia in ast.sentencias:
            if isinstance(sentencia, NodoDeclFuncion):
                self.funciones[sentencia.nombre] = sentencia
        try:
            self.ejecutar_bloque(ast.sentencias)
        except RetornoExcepcion:
            raise Exception("¡Poriera! 'suelta' solo se puede usar dentro de una movida.")
        except (BreakExcepcion, ContinueExcepcion):
            raise Exception("¡Poriera! 'alCarajoPipa' o 'tiraPalante' fuera de un bucle.")

def main():
    if len(sys.argv) < 2:
        print("Uso: puyascript <archivo.puya>")
        sys.exit(1)
    archivo = sys.argv[1]
    if not archivo.endswith('.puya'):
        print("¡Poriera! El archivo tiene que ser .puya")
        sys.exit(1)
    try:
        with open(archivo, 'r', encoding='utf-8') as f:
            codigo = f.read()
        tokens = Lexer(codigo).tokenizar()
        ast = Parser(tokens).parsear()
        Interprete().ejecutar(ast)
    except FileNotFoundError:
        print(f"¡Poriera! No existe el archivo '{archivo}'")
        sys.exit(1)
    except Exception as e:
        print(f"Error fatal: {e}")
        sys.exit(1)

if __name__ == '__main__':
    main()
