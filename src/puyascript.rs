use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs;
use std::io::{self, BufWriter, Write};
use std::process;
use std::rc::Rc;
use std::cell::RefCell;

const TIPOS: [&str; 5] = ["guita", "suelto", "palique", "fijo", "peña"];
const MAX_PROFUNDIDAD: usize = 10_000;
const TAMANO_PILA: usize = 512 * 1024 * 1024;

const PALABRAS_RESERVADAS: &[&str] = &[
    "trincar", "banda", "churumbel", "poriera", "movida", "suelta", "chivato",
    "siAcaso", "siNo", "oSiNo", "esto", "josear", "arteCaso", "caso", "alCarajoPipa",
    "noNiNa", "estoY", "estoO", "intocable", "mamaDe", "privao", "palPueblo",
    "roña", "tiraPalante", "verdá", "esIA", "guita", "suelto", "palique", "fijo", "na", "peña", "dame",
];

#[derive(Clone, Copy, PartialEq, Debug)]
enum Tipo {
    Guita,
    Suelto,
    Palique,
    Fijo,
    Pena,
    Na,
}

impl fmt::Display for Tipo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tipo::Guita => write!(f, "guita"),
            Tipo::Suelto => write!(f, "suelto"),
            Tipo::Palique => write!(f, "palique"),
            Tipo::Fijo => write!(f, "fijo"),
            Tipo::Pena => write!(f, "peña"),
            Tipo::Na => write!(f, "na"),
        }
    }
}

// --- 1. VALORES ---
#[derive(Clone)]
enum Value {
    Guita(i64),
    Suelto(f64),
    Palique(Rc<str>),
    Fijo(bool),
    Pena(Rc<RefCell<Vec<Value>>>),
    Na,
}

impl Value {
    fn tipo(&self) -> Tipo {
        match self {
            Value::Guita(_) => Tipo::Guita,
            Value::Suelto(_) => Tipo::Suelto,
            Value::Palique(_) => Tipo::Palique,
            Value::Fijo(_) => Tipo::Fijo,
            Value::Pena(_) => Tipo::Pena,
            Value::Na => Tipo::Na,
        }
    }

    fn es_cierto(&self) -> bool {
        match self {
            Value::Fijo(b) => *b,
            Value::Guita(n) => *n != 0,
            Value::Suelto(f) => *f != 0.0,
            Value::Palique(s) => !s.is_empty(),
            Value::Pena(arr) => !arr.borrow().is_empty(),
            Value::Na => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Guita(v) => write!(f, "{}", v),
            Value::Suelto(v) => write!(f, "{:?}", v),
            Value::Palique(v) => write!(f, "{}", v),
            Value::Fijo(v) => write!(f, "{}", if *v { "verdá" } else { "esIA" }),
            Value::Pena(arr) => {
                let elementos = arr.borrow();
                let reps: Vec<String> = elementos.iter().map(|e| e.to_string()).collect();
                write!(f, "[{}]", reps.join(", "))
            }
            Value::Na => write!(f, "na"),
        }
    }
}

fn iguales(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Guita(x), Value::Guita(y)) => x == y,
        (Value::Suelto(x), Value::Suelto(y)) => x == y,
        (Value::Guita(x), Value::Suelto(y)) | (Value::Suelto(y), Value::Guita(x)) => (*x as f64) == *y,
        (Value::Palique(x), Value::Palique(y)) => x == y,
        (Value::Fijo(x), Value::Fijo(y)) => x == y,
        (Value::Na, Value::Na) => true,
        _ => false,
    }
}

// --- 2. LEXER ---
#[derive(Clone, Copy, PartialEq)]
enum TokenType {
    NumeroGuita,
    NumeroSuelto,
    Palique,
    Keyword,
    Identificador,
    Operador,
    Asigna,
    PuntoComa,
    ParenIzq,
    ParenDer,
    LlaveIzq,
    LlaveDer,
    CorcheteIzq,
    CorcheteDer,
    Coma,
    Punto,
    DosPuntos,
    Flecha,
    Eof,
}

impl TokenType {
    fn nombre(self) -> &'static str {
        match self {
            TokenType::NumeroGuita | TokenType::NumeroSuelto => "un número",
            TokenType::Palique => "un texto",
            TokenType::Keyword => "una palabra reservada",
            TokenType::Identificador => "un nombre",
            TokenType::Operador => "un operador",
            TokenType::Asigna => "'='",
            TokenType::PuntoComa => "';'",
            TokenType::ParenIzq => "'('",
            TokenType::ParenDer => "')'",
            TokenType::LlaveIzq => "'{'",
            TokenType::LlaveDer => "'}'",
            TokenType::CorcheteIzq => "'['",
            TokenType::CorcheteDer => "']'",
            TokenType::Coma => "','",
            TokenType::Punto => "'.'",
            TokenType::DosPuntos => "':'",
            TokenType::Flecha => "'->'",
            TokenType::Eof => "fin de archivo",
        }
    }
}

struct Token {
    tipo: TokenType,
    valor: String,
    linea: usize,
}

fn describir(t: &Token) -> String {
    if t.tipo == TokenType::Eof {
        "fin de archivo".to_string()
    } else if t.tipo == TokenType::Palique {
        format!("\"{}\"", t.valor)
    } else {
        format!("'{}'", t.valor)
    }
}

struct Lexer {
    codigo: Vec<char>,
    pos: usize,
    linea: usize,
}

impl Lexer {
    fn new(codigo: &str) -> Self {
        Self { codigo: codigo.chars().collect(), pos: 0, linea: 1 }
    }

    fn actual(&self) -> Option<char> {
        self.codigo.get(self.pos).copied()
    }

    fn siguiente(&self) -> Option<char> {
        self.codigo.get(self.pos + 1).copied()
    }

    fn avanzar(&mut self) {
        if self.actual() == Some('\n') {
            self.linea += 1;
        }
        self.pos += 1;
    }

    fn saltar_espacios_y_comentarios(&mut self) {
        while let Some(c) = self.actual() {
            if c.is_whitespace() {
                self.avanzar();
            } else if c == '/' && self.siguiente() == Some('/') {
                while let Some(nc) = self.actual() {
                    if nc == '\n' {
                        break;
                    }
                    self.avanzar();
                }
            } else {
                break;
            }
        }
    }

    fn numero(&mut self) -> Result<(TokenType, String), String> {
        let mut s = String::new();
        while let Some(c) = self.actual().filter(|c| c.is_ascii_digit()) {
            s.push(c);
            self.avanzar();
        }
        if self.actual() == Some('.') && self.siguiente().map_or(false, |c| c.is_ascii_digit()) {
            s.push('.');
            self.avanzar();
            while let Some(c) = self.actual().filter(|c| c.is_ascii_digit()) {
                s.push(c);
                self.avanzar();
            }
            if s.parse::<f64>().is_err() {
                return Err(format!("¡Poriera! (línea {}) Número no válido: {}", self.linea, s));
            }
            Ok((TokenType::NumeroSuelto, s))
        } else {
            if s.parse::<i64>().is_err() {
                return Err(format!("¡Poriera! (línea {}) Número entero demasiado grande: {}", self.linea, s));
            }
            Ok((TokenType::NumeroGuita, s))
        }
    }

    fn identificador(&mut self) -> (TokenType, String) {
        let mut s = String::new();
        while let Some(c) = self.actual().filter(|c| c.is_alphanumeric() || *c == '_') {
            s.push(c);
            self.avanzar();
        }
        let tipo = if PALABRAS_RESERVADAS.contains(&s.as_str()) {
            TokenType::Keyword
        } else {
            TokenType::Identificador
        };
        (tipo, s)
    }

    fn texto(&mut self) -> Result<(TokenType, String), String> {
        let linea_inicio = self.linea;
        self.avanzar();
        let mut s = String::new();
        loop {
            match self.actual() {
                Some('"') => {
                    self.avanzar();
                    return Ok((TokenType::Palique, s));
                }
                Some(c) => {
                    s.push(c);
                    self.avanzar();
                }
                None => {
                    return Err(format!(
                        "¡Poriera! (línea {}) Texto sin cerrar: falta la comilla final.",
                        linea_inicio
                    ))
                }
            }
        }
    }

    fn tokenizar(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        loop {
            self.saltar_espacios_y_comentarios();
            let c = match self.actual() {
                Some(c) => c,
                None => break,
            };
            let linea = self.linea;

            let (tipo, valor) = if c.is_ascii_digit() {
                self.numero()?
            } else if c.is_alphabetic() || c == '_' {
                self.identificador()
            } else if c == '"' {
                self.texto()?
            } else {
                let doble = match (c, self.siguiente()) {
                    ('-', Some('>')) => Some((TokenType::Flecha, "->")),
                    ('=', Some('=')) => Some((TokenType::Operador, "==")),
                    ('!', Some('=')) => Some((TokenType::Operador, "!=")),
                    ('<', Some('=')) => Some((TokenType::Operador, "<=")),
                    ('>', Some('=')) => Some((TokenType::Operador, ">=")),
                    _ => None,
                };
                if let Some((t, v)) = doble {
                    self.avanzar();
                    self.avanzar();
                    (t, v.to_string())
                } else {
                    let t = match c {
                        '+' | '-' | '*' | '/' | '<' | '>' => TokenType::Operador,
                        '=' => TokenType::Asigna,
                        ';' => TokenType::PuntoComa,
                        '(' => TokenType::ParenIzq,
                        ')' => TokenType::ParenDer,
                        '{' => TokenType::LlaveIzq,
                        '}' => TokenType::LlaveDer,
                        '[' => TokenType::CorcheteIzq,
                        ']' => TokenType::CorcheteDer,
                        ',' => TokenType::Coma,
                        '.' => TokenType::Punto,
                        ':' => TokenType::DosPuntos,
                        _ => return Err(format!("¡Poriera! (línea {}) Carácter ilegal: '{}'", linea, c)),
                    };
                    self.avanzar();
                    (t, c.to_string())
                }
            };
            tokens.push(Token { tipo, valor, linea });
        }
        tokens.push(Token { tipo: TokenType::Eof, valor: String::new(), linea: self.linea });
        Ok(tokens)
    }
}

// --- 3. AST ---
#[derive(Clone, Copy, PartialEq)]
enum Op {
    Or,
    And,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn precedencia(self) -> u8 {
        match self {
            Op::Or => 1,
            Op::And => 2,
            Op::Eq | Op::Ne => 3,
            Op::Lt | Op::Gt | Op::Le | Op::Ge => 4,
            Op::Add | Op::Sub => 5,
            Op::Mul | Op::Div => 6,
        }
    }

    fn simbolo(self) -> &'static str {
        match self {
            Op::Or => "estoO",
            Op::And => "estoY",
            Op::Eq => "==",
            Op::Ne => "!=",
            Op::Lt => "<",
            Op::Gt => ">",
            Op::Le => "<=",
            Op::Ge => ">=",
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/",
        }
    }
}

enum Expr {
    Literal(Value),
    Variable(String),
    Arreglo(Vec<Expr>),
    Indexacion(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    No(Box<Expr>),
    Bin(Box<Expr>, Op, Box<Expr>),
    Llamada(String, Vec<Expr>),
    Dame(Option<Box<Expr>>),
}

struct Func {
    parametros: Vec<(Tipo, String)>,
    bloque: Vec<Stmt>,
}

enum Stmt {
    DeclVar { tipo: Tipo, nombre: String, expr: Expr, es_constante: bool },
    Asignacion(String, Expr),
    AsignacionIndexada(String, Expr, Expr),
    Chivato(Expr),
    Expr(Expr),
    Si {
        condicion: Expr,
        bloque_si: Vec<Stmt>,
        ramas: Vec<(Expr, Vec<Stmt>)>,
        bloque_osino: Option<Vec<Stmt>>,
    },
    While(Expr, Vec<Stmt>),
    For { init: Box<Stmt>, cond: Expr, paso: Box<Stmt>, bloque: Vec<Stmt> },
    ArteCaso(Expr, Vec<(Expr, Vec<Stmt>)>),
    Break,
    Continue,
    Suelta(Expr),
    DeclFuncion(String, Rc<Func>),
}

// --- 4. PARSER ---
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn actual(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn error<T>(&self, msg: &str) -> Result<T, String> {
        Err(format!("¡Poriera! (línea {}) {}", self.actual().linea, msg))
    }

    fn avanzar(&mut self) {
        if self.pos + 1 < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn es(&self, t: TokenType) -> bool {
        self.actual().tipo == t
    }

    fn es_kw(&self, kws: &[&str]) -> bool {
        let t = self.actual();
        t.tipo == TokenType::Keyword && kws.contains(&t.valor.as_str())
    }

    fn consumir(&mut self, t: TokenType) -> Result<String, String> {
        if self.es(t) {
            let v = std::mem::take(&mut self.tokens[self.pos].valor);
            self.avanzar();
            Ok(v)
        } else {
            self.error(&format!(
                "Se esperaba {}, pero se encontró {}",
                t.nombre(),
                describir(self.actual())
            ))
        }
    }

    fn consumir_kw(&mut self, kw: &str) -> Result<(), String> {
        if self.es_kw(&[kw]) {
            self.avanzar();
            Ok(())
        } else {
            self.error(&format!("Se esperaba '{}', pero se encontró {}", kw, describir(self.actual())))
        }
    }

    fn parse_tipo(&mut self, permitir_na: bool) -> Result<Tipo, String> {
        if self.es_kw(&TIPOS) || (permitir_na && self.es_kw(&["na"])) {
            let t_str = self.consumir(TokenType::Keyword)?;
            match t_str.as_str() {
                "guita" => Ok(Tipo::Guita),
                "suelto" => Ok(Tipo::Suelto),
                "palique" => Ok(Tipo::Palique),
                "fijo" => Ok(Tipo::Fijo),
                "peña" => Ok(Tipo::Pena),
                "na" => Ok(Tipo::Na),
                _ => self.error("Tipo desconocido."),
            }
        } else {
            self.error(&format!("Se esperaba un tipo, pero se encontró {}", describir(self.actual())))
        }
    }

    fn argumentos(&mut self) -> Result<Vec<Expr>, String> {
        self.consumir(TokenType::ParenIzq)?;
        let mut args = Vec::new();
        if !self.es(TokenType::ParenDer) {
            args.push(self.expresion(1)?);
            while self.es(TokenType::Coma) {
                self.avanzar();
                args.push(self.expresion(1)?);
            }
        }
        self.consumir(TokenType::ParenDer)?;
        Ok(args)
    }

    fn primaria(&mut self) -> Result<Expr, String> {
        let mut expr = match self.actual().tipo {
            TokenType::NumeroGuita => {
                let v = self.consumir(TokenType::NumeroGuita)?;
                match v.parse::<i64>() {
                    Ok(n) => Expr::Literal(Value::Guita(n)),
                    Err(_) => return self.error("Número entero demasiado grande."),
                }
            }
            TokenType::NumeroSuelto => {
                let v = self.consumir(TokenType::NumeroSuelto)?;
                match v.parse::<f64>() {
                    Ok(n) => Expr::Literal(Value::Suelto(n)),
                    Err(_) => return self.error("Número no válido."),
                }
            }
            TokenType::Palique => {
                let v = self.consumir(TokenType::Palique)?;
                Expr::Literal(Value::Palique(v.into()))
            }
            TokenType::Keyword if self.es_kw(&["verdá", "esIA"]) => {
                let v = self.consumir(TokenType::Keyword)?;
                Expr::Literal(Value::Fijo(v == "verdá"))
            }
            TokenType::Keyword if self.es_kw(&["dame"]) => {
                self.avanzar();
                let prompt = if self.es(TokenType::ParenIzq) {
                    self.consumir(TokenType::ParenIzq)?;
                    let p = if !self.es(TokenType::ParenDer) {
                        Some(Box::new(self.expresion(1)?))
                    } else {
                        None
                    };
                    self.consumir(TokenType::ParenDer)?;
                    p
                } else {
                    None
                };
                Expr::Dame(prompt)
            }
            TokenType::Identificador => {
                let nombre = self.consumir(TokenType::Identificador)?;
                if self.es(TokenType::ParenIzq) {
                    Expr::Llamada(nombre, self.argumentos()?)
                } else {
                    Expr::Variable(nombre)
                }
            }
            TokenType::CorcheteIzq => {
                self.avanzar();
                let mut elems = Vec::new();
                if !self.es(TokenType::CorcheteDer) {
                    elems.push(self.expresion(1)?);
                    while self.es(TokenType::Coma) {
                        self.avanzar();
                        elems.push(self.expresion(1)?);
                    }
                }
                self.consumir(TokenType::CorcheteDer)?;
                Expr::Arreglo(elems)
            }
            TokenType::ParenIzq => {
                self.avanzar();
                let e = self.expresion(1)?;
                self.consumir(TokenType::ParenDer)?;
                e
            }
            _ => return self.error(&format!("Expresión no reconocida: {}", describir(self.actual()))),
        };

        while self.es(TokenType::CorcheteIzq) {
            self.avanzar();
            let idx = self.expresion(1)?;
            self.consumir(TokenType::CorcheteDer)?;
            expr = Expr::Indexacion(Box::new(expr), Box::new(idx));
        }

        Ok(expr)
    }

    fn unaria(&mut self) -> Result<Expr, String> {
        if self.es_kw(&["noNiNa"]) {
            self.avanzar();
            return Ok(Expr::No(Box::new(self.unaria()?)));
        }
        if self.es(TokenType::Operador) && self.actual().valor == "-" {
            self.avanzar();
            return Ok(Expr::Neg(Box::new(self.unaria()?)));
        }
        self.primaria()
    }

    fn op_actual(&self) -> Option<Op> {
        let t = self.actual();
        match t.tipo {
            TokenType::Operador => match t.valor.as_str() {
                "+" => Some(Op::Add),
                "-" => Some(Op::Sub),
                "*" => Some(Op::Mul),
                "/" => Some(Op::Div),
                "<" => Some(Op::Lt),
                ">" => Some(Op::Gt),
                "<=" => Some(Op::Le),
                ">=" => Some(Op::Ge),
                "==" => Some(Op::Eq),
                "!=" => Some(Op::Ne),
                _ => None,
            },
            TokenType::Keyword => match t.valor.as_str() {
                "estoY" => Some(Op::And),
                "estoO" => Some(Op::Or),
                _ => None,
            },
            _ => None,
        }
    }

    fn expresion(&mut self, minima: u8) -> Result<Expr, String> {
        let mut izq = self.unaria()?;
        while let Some(op) = self.op_actual() {
            if op.precedencia() < minima {
                break;
            }
            self.avanzar();
            let der = self.expresion(op.precedencia() + 1)?;
            izq = Expr::Bin(Box::new(izq), op, Box::new(der));
        }
        Ok(izq)
    }

    fn bloque(&mut self) -> Result<Vec<Stmt>, String> {
        self.consumir(TokenType::LlaveIzq)?;
        let mut sentencias = Vec::new();
        while !self.es(TokenType::LlaveDer) && !self.es(TokenType::Eof) {
            sentencias.push(self.sentencia()?);
        }
        self.consumir(TokenType::LlaveDer)?;
        Ok(sentencias)
    }

    fn condicion(&mut self) -> Result<Expr, String> {
        self.consumir(TokenType::ParenIzq)?;
        let c = self.expresion(1)?;
        self.consumir(TokenType::ParenDer)?;
        Ok(c)
    }

    fn si_acaso(&mut self) -> Result<Stmt, String> {
        self.consumir_kw("siAcaso")?;
        let condicion = self.condicion()?;
        let bloque_si = self.bloque()?;
        let mut ramas = Vec::new();
        while self.es_kw(&["siNo"]) {
            self.avanzar();
            let c = self.condicion()?;
            ramas.push((c, self.bloque()?));
        }
        let bloque_osino = if self.es_kw(&["oSiNo"]) {
            self.avanzar();
            Some(self.bloque()?)
        } else {
            None
        };
        Ok(Stmt::Si { condicion, bloque_si, ramas, bloque_osino })
    }

    fn josear(&mut self) -> Result<Stmt, String> {
        self.consumir_kw("josear")?;
        self.consumir(TokenType::ParenIzq)?;
        if self.es_kw(&TIPOS) {
            let tipo = self.parse_tipo(false)?;
            let nombre = self.consumir(TokenType::Identificador)?;
            self.consumir(TokenType::Asigna)?;
            let init = Stmt::DeclVar { tipo, nombre, expr: self.expresion(1)?, es_constante: false };
            self.consumir(TokenType::PuntoComa)?;
            let cond = self.expresion(1)?;
            self.consumir(TokenType::PuntoComa)?;
            let nombre_paso = self.consumir(TokenType::Identificador)?;
            self.consumir(TokenType::Asigna)?;
            let paso = Stmt::Asignacion(nombre_paso, self.expresion(1)?);
            self.consumir(TokenType::ParenDer)?;
            let bloque = self.bloque()?;
            Ok(Stmt::For { init: Box::new(init), cond, paso: Box::new(paso), bloque })
        } else {
            let cond = self.expresion(1)?;
            self.consumir(TokenType::ParenDer)?;
            Ok(Stmt::While(cond, self.bloque()?))
        }
    }

    fn arte_caso(&mut self) -> Result<Stmt, String> {
        self.consumir_kw("arteCaso")?;
        let expr = self.condicion()?;
        self.consumir(TokenType::LlaveIzq)?;
        let mut casos = Vec::new();
        while self.es_kw(&["caso"]) {
            self.avanzar();
            let valor = self.expresion(1)?;
            self.consumir(TokenType::DosPuntos)?;
            let mut sentencias = Vec::new();
            while !self.es_kw(&["caso"]) && !self.es(TokenType::LlaveDer) && !self.es(TokenType::Eof) {
                sentencias.push(self.sentencia()?);
            }
            casos.push((valor, sentencias));
        }
        self.consumir(TokenType::LlaveDer)?;
        Ok(Stmt::ArteCaso(expr, casos))
    }

    fn movida(&mut self) -> Result<Stmt, String> {
        self.consumir_kw("movida")?;
        let nombre = self.consumir(TokenType::Identificador)?;
        self.consumir(TokenType::ParenIzq)?;
        let mut parametros = Vec::new();
        if !self.es(TokenType::ParenDer) {
            let t = self.parse_tipo(false)?;
            let p_nombre = self.consumir(TokenType::Identificador)?;
            parametros.push((t, p_nombre));
            while self.es(TokenType::Coma) {
                self.avanzar();
                let t = self.parse_tipo(false)?;
                let p_nombre = self.consumir(TokenType::Identificador)?;
                parametros.push((t, p_nombre));
            }
        }
        self.consumir(TokenType::ParenDer)?;
        if self.es(TokenType::Flecha) {
            self.avanzar();
            self.parse_tipo(true)?;
        }
        let bloque = self.bloque()?;
        Ok(Stmt::DeclFuncion(nombre, Rc::new(Func { parametros, bloque })))
    }

    fn sentencia(&mut self) -> Result<Stmt, String> {
        match self.actual().tipo {
            TokenType::Keyword => {
                let kw = self.actual().valor.clone();
                match kw.as_str() {
                    "intocable" => {
                        self.avanzar();
                        let tipo = self.parse_tipo(false)?;
                        let nombre = self.consumir(TokenType::Identificador)?;
                        self.consumir(TokenType::Asigna)?;
                        let expr = self.expresion(1)?;
                        self.consumir(TokenType::PuntoComa)?;
                        Ok(Stmt::DeclVar { tipo, nombre, expr, es_constante: true })
                    }
                    "guita" | "suelto" | "palique" | "fijo" | "peña" => {
                        let tipo = self.parse_tipo(false)?;
                        let nombre = self.consumir(TokenType::Identificador)?;
                        self.consumir(TokenType::Asigna)?;
                        let expr = self.expresion(1)?;
                        self.consumir(TokenType::PuntoComa)?;
                        Ok(Stmt::DeclVar { tipo, nombre, expr, es_constante: false })
                    }
                    "chivato" => {
                        self.avanzar();
                        self.consumir(TokenType::ParenIzq)?;
                        let e = self.expresion(1)?;
                        self.consumir(TokenType::ParenDer)?;
                        self.consumir(TokenType::PuntoComa)?;
                        Ok(Stmt::Chivato(e))
                    }
                    "alCarajoPipa" => {
                        self.avanzar();
                        self.consumir(TokenType::PuntoComa)?;
                        Ok(Stmt::Break)
                    }
                    "tiraPalante" => {
                        self.avanzar();
                        self.consumir(TokenType::PuntoComa)?;
                        Ok(Stmt::Continue)
                    }
                    "suelta" => {
                        self.avanzar();
                        let e = self.expresion(1)?;
                        self.consumir(TokenType::PuntoComa)?;
                        Ok(Stmt::Suelta(e))
                    }
                    "siAcaso" => self.si_acaso(),
                    "arteCaso" => self.arte_caso(),
                    "josear" => self.josear(),
                    "movida" => self.movida(),
                    _ => self.error(&format!("Sentencia desconocida arrancando en: '{}'", kw)),
                }
            }
            TokenType::Identificador => {
                let nombre = self.consumir(TokenType::Identificador)?;
                if self.es(TokenType::ParenIzq) {
                    let args = self.argumentos()?;
                    self.consumir(TokenType::PuntoComa)?;
                    return Ok(Stmt::Expr(Expr::Llamada(nombre, args)));
                }
                if self.es(TokenType::CorcheteIzq) {
                    self.avanzar();
                    let idx = self.expresion(1)?;
                    self.consumir(TokenType::CorcheteDer)?;
                    self.consumir(TokenType::Asigna)?;
                    let e = self.expresion(1)?;
                    self.consumir(TokenType::PuntoComa)?;
                    return Ok(Stmt::AsignacionIndexada(nombre, idx, e));
                }
                self.consumir(TokenType::Asigna)?;
                let e = self.expresion(1)?;
                self.consumir(TokenType::PuntoComa)?;
                Ok(Stmt::Asignacion(nombre, e))
            }
            _ => self.error(&format!("Sentencia desconocida arrancando en: {}", describir(self.actual()))),
        }
    }

    fn parsear(&mut self) -> Result<Vec<Stmt>, String> {
        let mut sentencias = Vec::new();
        while !self.es(TokenType::Eof) {
            sentencias.push(self.sentencia()?);
        }
        Ok(sentencias)
    }
}

// --- 5. INTÉRPRETE ---
#[derive(Clone)]
struct Variable {
    tipo: Tipo,
    valor: Value,
    es_constante: bool,
}

enum Flujo {
    Retorno(Value),
    Break,
    Continue,
    Error(String),
}

type Res<T> = Result<T, Flujo>;

fn err<T>(msg: impl Into<String>) -> Res<T> {
    Err(Flujo::Error(msg.into()))
}

fn tipos_invalidos<T>(op: Op) -> Res<T> {
    err(format!("¡Poriera! No puedes usar '{}' con esos tipos de valor.", op.simbolo()))
}

fn desbordamiento<T>() -> Res<T> {
    err("¡Poriera! El número es demasiado grande (desbordamiento).")
}

fn operar_reales(op: Op, x: f64, y: f64) -> Res<Value> {
    match op {
        Op::Add => Ok(Value::Suelto(x + y)),
        Op::Sub => Ok(Value::Suelto(x - y)),
        Op::Mul => Ok(Value::Suelto(x * y)),
        Op::Div => {
            if y == 0.0 {
                err("¡Poriera! No se puede dividir entre cero.")
            } else {
                Ok(Value::Suelto(x / y))
            }
        }
        Op::Lt => Ok(Value::Fijo(x < y)),
        Op::Gt => Ok(Value::Fijo(x > y)),
        Op::Le => Ok(Value::Fijo(x <= y)),
        Op::Ge => Ok(Value::Fijo(x >= y)),
        _ => tipos_invalidos(op),
    }
}

fn operar(op: Op, a: Value, b: Value) -> Res<Value> {
    match op {
        Op::Eq => return Ok(Value::Fijo(iguales(&a, &b))),
        Op::Ne => return Ok(Value::Fijo(!iguales(&a, &b))),
        Op::Add if matches!(a, Value::Palique(_)) || matches!(b, Value::Palique(_)) => {
            return Ok(Value::Palique(format!("{}{}", a, b).into()));
        }
        _ => {}
    }
    match (&a, &b) {
        (Value::Guita(x), Value::Guita(y)) => {
            let (x, y) = (*x, *y);
            match op {
                Op::Add => x.checked_add(y).map(Value::Guita).map_or_else(desbordamiento, Ok),
                Op::Sub => x.checked_sub(y).map(Value::Guita).map_or_else(desbordamiento, Ok),
                Op::Mul => x.checked_mul(y).map(Value::Guita).map_or_else(desbordamiento, Ok),
                Op::Div => operar_reales(op, x as f64, y as f64),
                Op::Lt => Ok(Value::Fijo(x < y)),
                Op::Gt => Ok(Value::Fijo(x > y)),
                Op::Le => Ok(Value::Fijo(x <= y)),
                Op::Ge => Ok(Value::Fijo(x >= y)),
                _ => tipos_invalidos(op),
            }
        }
        (Value::Guita(x), Value::Suelto(y)) => operar_reales(op, *x as f64, *y),
        (Value::Suelto(x), Value::Guita(y)) => operar_reales(op, *x, *y as f64),
        (Value::Suelto(x), Value::Suelto(y)) => operar_reales(op, *x, *y),
        (Value::Palique(x), Value::Palique(y)) => match op {
            Op::Lt => Ok(Value::Fijo(x < y)),
            Op::Gt => Ok(Value::Fijo(x > y)),
            Op::Le => Ok(Value::Fijo(x <= y)),
            Op::Ge => Ok(Value::Fijo(x >= y)),
            _ => tipos_invalidos(op),
        },
        _ => tipos_invalidos(op),
    }
}

struct Interprete {
    global: HashMap<String, Variable>,
    marcos: Vec<HashMap<String, Variable>>,
    funciones: HashMap<String, Rc<Func>>,
    out: BufWriter<io::Stdout>,
}

impl Interprete {
    fn new() -> Self {
        Self {
            global: HashMap::new(),
            marcos: Vec::new(),
            funciones: HashMap::new(),
            out: BufWriter::new(io::stdout()),
        }
    }

    fn buscar(&self, nombre: &str) -> Option<&Variable> {
        self.marcos.last().and_then(|m| m.get(nombre)).or_else(|| self.global.get(nombre))
    }

    fn declarar(&mut self, tipo: Tipo, nombre: &str, valor: Value, es_constante: bool) -> Res<()> {
        if valor.tipo() != tipo {
            return err(format!(
                "¡Poriera! Se esperaba un valor de tipo '{}' para '{}', pero se obtuvo un '{}'.",
                tipo,
                nombre,
                valor.tipo()
            ));
        }
        let var = Variable { tipo, valor, es_constante };
        match self.marcos.last_mut() {
            Some(m) => m.insert(nombre.to_string(), var),
            None => self.global.insert(nombre.to_string(), var),
        };
        Ok(())
    }

    fn asignar(&mut self, nombre: &str, valor: Value) -> Res<()> {
        let mut variable = if let Some(m) = self.marcos.last_mut() {
            if let Some(var) = m.get_mut(nombre) {
                Some(var)
            } else {
                self.global.get_mut(nombre)
            }
        } else {
            self.global.get_mut(nombre)
        };

        if let Some(ref mut var) = variable {
            if var.es_constante {
                return err(format!("¡Poriera! La constante 'intocable' '{}' no se puede sobreescribir.", nombre));
            }
            if var.tipo != valor.tipo() {
                return err(format!(
                    "¡Poriera! No puedes asignar un '{}' a la variable '{}' que es de tipo '{}'.",
                    valor.tipo(),
                    nombre,
                    var.tipo
                ));
            }
            var.valor = valor;
            Ok(())
        } else {
            err(format!("¡Poriera! No puedes asignar a '{}': no existe.", nombre))
        }
    }

    fn eval(&mut self, e: &Expr) -> Res<Value> {
        match e {
            Expr::Literal(v) => Ok(v.clone()),
            Expr::Variable(n) => match self.buscar(n) {
                Some(v) => Ok(v.valor.clone()),
                None => err(format!("¡Poriera! La variable '{}' no existe.", n)),
            },
            Expr::Arreglo(elems) => {
                let mut res = Vec::new();
                for elem in elems {
                    res.push(self.eval(elem)?);
                }
                Ok(Value::Pena(Rc::new(RefCell::new(res))))
            }
            Expr::Indexacion(arr_expr, idx_expr) => {
                let arr_val = self.eval(arr_expr)?;
                let idx_val = self.eval(idx_expr)?;
                match (arr_val, idx_val) {
                    (Value::Pena(arr), Value::Guita(idx)) => {
                        let elementos = arr.borrow();
                        if idx < 0 || (idx as usize) >= elementos.len() {
                            return err("¡Poriera! Índice fuera de los límites de la peña.");
                        }
                        Ok(elementos[idx as usize].clone())
                    }
                    _ => err("¡Poriera! Solo se puede indexar una 'peña' usando un número 'guita'."),
                }
            }
            Expr::Neg(x) => match self.eval(x)? {
                Value::Guita(n) => n.checked_neg().map(Value::Guita).map_or_else(desbordamiento, Ok),
                Value::Suelto(f) => Ok(Value::Suelto(-f)),
                _ => err("¡Poriera! No puedes cambiar de signo algo que no es un número."),
            },
            Expr::No(x) => Ok(Value::Fijo(!self.eval(x)?.es_cierto())),
            Expr::Bin(izq, Op::And, der) => {
                if !self.eval(izq)?.es_cierto() {
                    return Ok(Value::Fijo(false));
                }
                Ok(Value::Fijo(self.eval(der)?.es_cierto()))
            }
            Expr::Bin(izq, Op::Or, der) => {
                if self.eval(izq)?.es_cierto() {
                    return Ok(Value::Fijo(true));
                }
                Ok(Value::Fijo(self.eval(der)?.es_cierto()))
            }
            Expr::Bin(izq, op, der) => {
                let a = self.eval(izq)?;
                let b = self.eval(der)?;
                operar(*op, a, b)
            }
            Expr::Llamada(nombre, args) => self.llamar(nombre, args),
            Expr::Dame(prompt) => {
                if let Some(p_expr) = prompt {
                    let p_val = self.eval(p_expr)?;
                    print!("{}", p_val);
                    let _ = io::stdout().flush();
                }
                let mut entrada = String::new();
                if io::stdin().read_line(&mut entrada).is_err() {
                    return err("¡Poriera! Error al leer la entrada del usuario.");
                }
                // Limpiar retorno de carro y saltos de línea de la consola
                let limpia = entrada.trim_end_matches(['\r', '\n']).to_string();
                Ok(Value::Palique(limpia.into()))
            }
        }
    }

    fn llamar(&mut self, nombre: &str, args: &[Expr]) -> Res<Value> {
        let func = match self.funciones.get(nombre) {
            Some(f) => Rc::clone(f),
            None => return err(format!("¡Poriera! La movida '{}' no está definida.", nombre)),
        };
        if func.parametros.len() != args.len() {
            return err(format!(
                "¡Poriera! La movida '{}' espera {} argumento(s) y se le pasaron {}.",
                nombre,
                func.parametros.len(),
                args.len()
            ));
        }
        let mut marco = HashMap::with_capacity(args.len());
        for ((p_tipo, p_nombre), a) in func.parametros.iter().zip(args) {
            let val = self.eval(a)?;
            if val.tipo() != *p_tipo {
                return err(format!(
                    "¡Poriera! La movida '{}' esperaba un '{}' para el parámetro '{}', pero se pasó un '{}'.",
                    nombre,
                    p_tipo,
                    p_nombre,
                    val.tipo()
                ));
            }
            marco.insert(
                p_nombre.clone(),
                Variable {
                    tipo: *p_tipo,
                    valor: val,
                    es_constante: false,
                },
            );
        }
        if self.marcos.len() >= MAX_PROFUNDIDAD {
            return err(format!("¡Poriera! Demasiada recursión en la movida '{}'.", nombre));
        }
        self.marcos.push(marco);
        let r = self.ejecutar_bloque(&func.bloque);
        self.marcos.pop();
        match r {
            Ok(()) => Ok(Value::Na),
            Err(Flujo::Retorno(v)) => Ok(v),
            Err(Flujo::Break) | Err(Flujo::Continue) => {
                err("¡Poriera! 'alCarajoPipa' o 'tiraPalante' fuera de un bucle.")
            }
            Err(e) => Err(e),
        }
    }

    fn ejecutar_bloque(&mut self, sentencias: &[Stmt]) -> Res<()> {
        for s in sentencias {
            self.ejecutar_sentencia(s)?;
        }
        Ok(())
    }

    fn ejecutar_sentencia(&mut self, s: &Stmt) -> Res<()> {
        match s {
            Stmt::DeclFuncion(nombre, f) => {
                self.funciones.insert(nombre.clone(), Rc::clone(f));
            }
            Stmt::DeclVar { tipo, nombre, expr, es_constante } => {
                let v = self.eval(expr)?;
                self.declarar(*tipo, nombre, v, *es_constante)?;
            }
            Stmt::Asignacion(nombre, e) => {
                let v = self.eval(e)?;
                self.asignar(nombre, v)?;
            }
            Stmt::AsignacionIndexada(nombre, idx_expr, val_expr) => {
                let (arr_val, es_constante) = match self.buscar(nombre) {
                    Some(v) => (v.valor.clone(), v.es_constante),
                    None => return err(format!("¡Poriera! La variable '{}' no existe.", nombre)),
                };
                if es_constante {
                    return err(format!("¡Poriera! No se pueden modificar los elementos de la constante 'intocable' '{}'.", nombre));
                }
                let idx_val = self.eval(idx_expr)?;
                let nuevo_val = self.eval(val_expr)?;
                match (arr_val, idx_val) {
                    (Value::Pena(arr), Value::Guita(idx)) => {
                        let mut elementos = arr.borrow_mut();
                        if idx < 0 || (idx as usize) >= elementos.len() {
                            return err("¡Poriera! Índice fuera de los límites de la peña.");
                        }
                        elementos[idx as usize] = nuevo_val;
                    }
                    _ => return err("¡Poriera! Solo se puede modificar elementos de una 'peña' con índice 'guita'."),
                }
            }
            Stmt::Chivato(e) => {
                let v = self.eval(e)?;
                let _ = writeln!(self.out, "{}", v);
            }
            Stmt::Expr(e) => {
                self.eval(e)?;
            }
            Stmt::Suelta(e) => {
                let v = self.eval(e)?;
                return Err(Flujo::Retorno(v));
            }
            Stmt::Break => return Err(Flujo::Break),
            Stmt::Continue => return Err(Flujo::Continue),
            Stmt::Si { condicion, bloque_si, ramas, bloque_osino } => {
                if self.eval(condicion)?.es_cierto() {
                    return self.ejecutar_bloque(bloque_si);
                }
                for (c, b) in ramas {
                    if self.eval(c)?.es_cierto() {
                        return self.ejecutar_bloque(b);
                    }
                }
                if let Some(b) = bloque_osino {
                    return self.ejecutar_bloque(b);
                }
            }
            Stmt::ArteCaso(expr, casos) => {
                let objetivo = self.eval(expr)?;
                let mut cae = false;
                for (valor, bloque) in casos {
                    if !cae {
                        let v = self.eval(valor)?;
                        cae = iguales(&objetivo, &v);
                    }
                    if cae {
                        match self.ejecutar_bloque(bloque) {
                            Err(Flujo::Break) => break,
                            Err(e) => return Err(e),
                            Ok(()) => {}
                        }
                    }
                }
            }
            Stmt::While(cond, bloque) => {
                while self.eval(cond)?.es_cierto() {
                    match self.ejecutar_bloque(bloque) {
                        Err(Flujo::Break) => break,
                        Err(Flujo::Continue) | Ok(()) => {}
                        Err(e) => return Err(e),
                    }
                }
            }
            Stmt::For { init, cond, paso, bloque } => {
                self.ejecutar_sentencia(init)?;
                while self.eval(cond)?.es_cierto() {
                    match self.ejecutar_bloque(bloque) {
                        Err(Flujo::Break) => break,
                        Err(Flujo::Continue) | Ok(()) => {}
                        Err(e) => return Err(e),
                    }
                    self.ejecutar_sentencia(paso)?;
                }
            }
        }
        Ok(())
    }

    fn ejecutar(&mut self, programa: &[Stmt]) -> Result<(), String> {
        for s in programa {
            if let Stmt::DeclFuncion(nombre, f) = s {
                self.funciones.insert(nombre.clone(), Rc::clone(f));
            }
        }
        match self.ejecutar_bloque(programa) {
            Ok(()) => Ok(()),
            Err(Flujo::Error(e)) => Err(e),
            Err(Flujo::Retorno(_)) => Err("¡Poriera! 'suelta' solo se puede usar dentro de una movida.".to_string()),
            Err(Flujo::Break) | Err(Flujo::Continue) => {
                Err("¡Poriera! 'alCarajoPipa' o 'tiraPalante' fuera de un bucle.".to_string())
            }
        }
    }
}

// --- 6. INTERFAZ DE CONSOLA ---
fn ejecutar_archivo(archivo: &str) -> i32 {
    let codigo = match fs::read_to_string(archivo) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("¡Poriera! No existe o no se puede leer el archivo '{}'", archivo);
            return 1;
        }
    };
    let tokens = match Lexer::new(&codigo).tokenizar() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error fatal: {}", e);
            return 1;
        }
    };
    let programa = match Parser::new(tokens).parsear() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error fatal: {}", e);
            return 1;
        }
    };
    let mut interprete = Interprete::new();
    let resultado = interprete.ejecutar(&programa);
    let _ = interprete.out.flush();
    match resultado {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("Error fatal: {}", e);
            1
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Uso: puyascript <archivo.puya>");
        process::exit(1);
    }
    let archivo = args[1].clone();
    if !archivo.ends_with(".puya") {
        eprintln!("¡Poriera! El archivo tiene que ser .puya");
        process::exit(1);
    }
    let a = archivo.clone();
    let codigo = match std::thread::Builder::new().stack_size(TAMANO_PILA).spawn(move || ejecutar_archivo(&a)) {
        Ok(h) => h.join().unwrap_or(1),
        Err(_) => ejecutar_archivo(&archivo),
    };
    process::exit(codigo);
}