use std::io;
use std::str::FromStr;
use std::io::Read;
use std::fs::File;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::*;
use std::borrow::*;
use std::iter::zip;
use std::env;

#[allow(non_camel_case_types)]
enum Var{
    int(i32),
    vec(Vec<Rc<RefCell<Var>>>),
}


trait Eval{
    fn new(s: &mut &str) -> Self;
    fn run(&self, vars : &mut HashMap<String, Rc<RefCell<Var>>>, funcs : &HashMap<String, FuncDef>) -> Rc<RefCell<Var>>;
}

struct Program{ // no need to create different lifetime because they are quite same 
    func_table : HashMap<String, FuncDef>,
    exp : Exp,
}

impl Eval for Program
{
    fn new(s: &mut &str) -> Self
    {
        let mut p = Program { func_table : HashMap::new(), exp : Exp::new(&mut "0")};
        let mut has_val = false;
        loop {
            *s = s.trim_start();
            if s.starts_with("(") && s[1..].trim_start().starts_with("define") {
                //*s = &s[1..];
                let func = FuncDef::new(s);
                p.func_table.insert(func.name.clone(), func);
                //*s = s.trim_start();
                //assert!(s.start_with(")"));
                //*s = &s[1..];
            }
            else if s.is_empty() {
                break p;
            }
            else {
                assert!(!has_val);
                has_val = true;
                p.exp = Exp::new(s);
            }
        }
    }
    #[allow(unused_variables)] // this is an unused function
    fn run(&self, vars : &mut HashMap<String, Rc<RefCell<Var>>>, funcs : &HashMap<String, FuncDef>) -> Rc<RefCell<Var>>
    {
        Rc::new(RefCell::new(Var::int(0)))
    }
}

impl Program 
{
    fn start(&self) 
    {
        self.exp.run(&mut HashMap::new(), & self.func_table);
    }
}

struct FuncDef{
    name : String,
    params : Vec<String>,
    exp : Exp,
}  

impl Eval for FuncDef
{
    fn new(s: &mut &str) -> Self
    {
        *s = s.trim_start()[1..].trim_start()[6..].trim_start();// ( define 
        let name_end = s.find(|c:char| c.is_ascii_whitespace()).unwrap();
        let name = & s[..name_end];
        assert!( name.starts_with(|c:char| c.is_ascii_alphabetic() || c == '_') );

        *s = s[name_end..].trim_start();
        assert!( s.starts_with("(") );
        *s = s[1..].trim_start();
        let mut params = Vec::new();
        loop {
            if s.starts_with(")") {
                *s = s[1..].trim_start();
                break;
            }
            let pname_end = s.find(|c:char| c.is_ascii_whitespace() || c == ')').unwrap();
            params.push( s[..pname_end].to_string() );
            //println!("{}", &s[..pname_end]);
            *s = s[pname_end..].trim_start();
        }
        let exp = Exp::new(s);
        *s = s.trim_start();
        //println!("{}", s);
        if !s.starts_with(")") {
            println!("function {} error: it may contain multiple expressions without (begin ...)", name);
            panic!("func err");
        }
        *s = & s[1..];
        FuncDef {name:name.to_string(), params:params, exp:exp}
    }
    #[allow(unused_variables)]// this is an unused function
    fn run(&self, vars : &mut HashMap<String, Rc<RefCell<Var>>>, funcs : &HashMap<String, FuncDef>) -> Rc<RefCell<Var>>
    {
        Rc::new(RefCell::new(Var::int(0)))
    }
}

enum Exp {
    Const(Const),
    Lval(Lval),
    FuncCall(FuncCall),
}

impl Eval for Exp
{
    fn new(s: &mut &str) -> Self
    {
        *s = s.trim_start();
        if s.starts_with("(") {
            Exp::FuncCall(FuncCall::new(s))
        }
        else if s.starts_with(|c:char| c.is_ascii_digit()) {
            Exp::Const(Const::new(s))
        }
        else if s.starts_with(|c:char| c == '_' || c.is_ascii_alphabetic()) {
            Exp::Lval(Lval::new(s))
        }
        else {
            panic!("Eval Exp Error");
            //Exp::Const(Const::new(&mut "0"))
        }
    }
    fn run(&self, vars : &mut HashMap<String, Rc<RefCell<Var>>>, funcs : &HashMap<String, FuncDef>) -> Rc<RefCell<Var>>
    {
        match self {
            Exp::Const(int) => int.run(vars, funcs),
            Exp::Lval(var) => var.run(vars, funcs),
            Exp::FuncCall(func) => func.run(vars, funcs),
        }
    }
}

struct Const{
    val : i32,
}

impl Eval for Const
{
    fn new(s: &mut &str) -> Self
    {
        let sp = s.find(|c:char| !c.is_ascii_digit()).unwrap_or(s.len());
        let ret = Const { val : i32::from_str(& s[..sp]).unwrap() };
        *s = & s[sp..];
        //println!("{}", & ret.val);
        ret
    }
    #[allow(unused_variables)]
    fn run(&self, vars : &mut HashMap<String, Rc<RefCell<Var>>>, funcs : &HashMap<String, FuncDef>) -> Rc<RefCell<Var>>
    {
        Rc::new(RefCell::new(Var::int(self.val)))
    }
}

struct Lval{
    name : String,
}

impl Eval for Lval
{
    fn new(s: &mut &str) -> Self
    {
        let sp = s.find(|c:char| !(c == '_' || c.is_ascii_alphabetic())).unwrap_or(s.len());
        let ret = Lval { name : s[..sp].to_string() };
        *s = & s[sp..];
        //println!("{}", & ret.name);
        ret
    }
    #[allow(unused_variables)]
    fn run(&self, vars : &mut HashMap<String, Rc<RefCell<Var>>>, funcs : &HashMap<String, FuncDef>) -> Rc<RefCell<Var>>
    {
        vars.get(&self.name).expect(("no variable named ".to_owned() + &self.name).as_str() ).clone()
    }
}

struct FuncCall{// ???
    name : String,
    params : Vec<Exp>,
}

impl Eval for FuncCall
{
    fn new(s: &mut &str) -> Self
    {
        *s = s[1..].trim_start();// (
        let name_end = s.find(|c:char| c.is_ascii_whitespace() || c == ')').unwrap();
        let mut func = FuncCall { name : s[..name_end].to_string(), params : Vec::new() };
        *s = & s[name_end..];
        //println!("{}", &func.name);
        loop {
            *s = s.trim_start();
            if s.starts_with(")") {
                *s = & s[1..];
                break func;
            }
            else {
                func.params.push(Exp::new(s));
            }
        }
    }
    fn run(&self, vars : &mut HashMap<String, Rc<RefCell<Var>>>, funcs : &HashMap<String, FuncDef>) -> Rc<RefCell<Var>>
    {
        let ret_zero = Rc::new(RefCell::new(Var::int(0)));
        if self.name == "if" {
            assert!(self.params.len() == 3);
            let ret_raw = self.params[0].run(vars, funcs);
            let ret = (ret_raw.borrow() as &RefCell<Var>).borrow();
            return match *ret {
                Var::int(0) => self.params[2].run(vars, funcs),
                Var::int(_) => self.params[1].run(vars, funcs),
                _ => panic!("condition in 'if' is not an integer"),
            }
        }
        if self.name == "=" {
            assert!(self.params.len() == 2);
            match &self.params[0] {
                Exp::Lval(lval) => {
                    let val = self.params[1].run(vars, funcs);
                    vars.insert(lval.name.clone(), val); // add or replace
                },
                _ => panic!("assign to a non-leftval"),
            }
            return ret_zero;
        }

        // all other operations' parameter can be pre-evalued 
        // we don't support short-cut operation

        let mut params_value = Vec::new();
        for exp in &self.params {
            params_value.push(exp.run(vars, funcs));
        }
        match self.name.as_str() {
            "begin" => {
                params_value.last().unwrap_or(& ret_zero).clone()
            }
            printfunc @ ("printint" | "printchar") => {
                assert!(params_value.len() == 1);
                let ret = (params_value[0].borrow() as &RefCell<Var>).borrow();
                match *ret {
                    Var::int(x) => 
                        if printfunc == "printint" { 
                            print!("{}", x)
                        } else {
                            print!("{}", char::from_u32(x as u32).expect("not a utf-8 character!"))
                        }
                    _ => panic!("printint/char: receive a non-int argument!")
                }
                ret_zero
            }
            "readint" => {
                assert!(params_value.len() == 0);
                let ret;
                unsafe{
                    #[allow(non_upper_case_globals)]
                    static mut input_line: String = String::new();
                    loop {
                        if input_line.is_empty() {
                            io::stdin().read_line(&mut input_line).unwrap();
                        }
                        else {
                            input_line = input_line.trim().to_string();
                            if input_line.is_empty() {
                                continue;
                            }
                            let pos = input_line.find(|c:char| !(c.is_ascii_digit() || c == '-')).unwrap_or(input_line.len());
                            ret = i32::from_str(&input_line[..pos]).expect("input is not an integer");
                            input_line = input_line[pos..].to_string();
                            break;
                        }
                    }
                }
                Rc::new(RefCell::new(Var::int(ret)))
            }
            "list" => Rc::new(RefCell::new(Var::vec(params_value))),
            "append" => {
                assert!(params_value.len() == 2);
                let var = &mut*(params_value[0].borrow() as &RefCell<Var>).borrow_mut();
                let var2 = &params_value[1];
                match var {
                    Var::vec(ref mut arr) => arr.push(var2.clone()),
                    _ => panic!("append to a non-list variable"),
                }
                ret_zero
            }
            ufunc if ufunc.starts_with(|c:char| c.is_ascii_alphabetic() || c == '_') => {
                let func = funcs.get(ufunc).expect(("no func called ".to_string() + &ufunc.to_string()).as_str());
                assert!(params_value.len() == func.params.len(), "number of func argument mismatch");
                func.exp.run(&mut HashMap::from_iter(zip(func.params.iter().cloned(), params_value.iter().cloned())), funcs)
            }
            _ => run_op(&self.name, params_value),
        }
    }
}

fn my_to_int(var : &Var) -> i32
{
    match var {
        Var::int(x) => *x,
        _ => panic!("operator need parameter type : int")
    }
}

fn run_op(name: &String, params_value: Vec<Rc<RefCell<Var>>>) -> Rc<RefCell<Var>>
{
    let ret_zero = Rc::new(RefCell::new(Var::int(0)));
    match name.as_str() {
        "[]=" => {
            let x = &mut *(params_value[0].borrow() as &RefCell<Var>).borrow_mut();
            let y = &*(params_value[1].borrow() as &RefCell<Var>).borrow();
            match (x, y) {
                (Var::vec(ref mut arr), Var::int(idx)) => {
                    let z = &params_value[2];
                    arr[idx.clone() as usize] = z.clone();
                }
                _ => panic!("assigne with index to a non-list variable, or index is non-integer"),
            }
            ret_zero
        }
        "[]" => {
            let x = &*(params_value[0].borrow() as &RefCell<Var>).borrow();
            let y = &*(params_value[1].borrow() as &RefCell<Var>).borrow();
            match (x, y) {
                (Var::vec(ref arr), Var::int(idx)) => arr[idx.clone() as usize].clone(),
                _ => panic!("get list item on a non-list variable, or index is non-integer"),
            }
        }
        op => {
            let x = my_to_int(&*(params_value[0].borrow() as &RefCell<Var>).borrow());
            let int = match op {
                "~" => !x,
                "!" => (x == 0) as i32,
                _ => {
                    let y = my_to_int(&*(params_value[1].borrow() as &RefCell<Var>).borrow());
                    let res = match op {
                        "+" => x + y,
                        "-" => x - y,
                        "*" => x * y,
                        "/" => x / y,
                        "%" => x % y,
                        "&" => x & y,
                        "|" => x | y,
                        "^" => x ^ y,
                        "<<" => x << y,
                        ">>" => x >> y,
                        bool_op => {
                            let bool_val = match bool_op {
                                ">" => x > y,
                                "<" => x < y,
                                ">=" => x >= y,
                                "<=" => x <= y,
                                "!=" => x != y,
                                "==" => x == y,
                                "&&" => (x != 0) & (y != 0),
                                "||" => (x != 0) | (y != 0),
                                unknown_op => panic!("unknown op {}", unknown_op),
                            };
                            bool_val as i32
                        }
                    };
                    res
                }
            };
            Rc::new(RefCell::new(Var::int(int)))
        }   
    }
}

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    assert!(args.len() == 2, "use: cargo run file_name");
    let mut f = File::open(&args[1]).expect("cannot open file");
    let mut sin = String::new();
    f.read_to_string(&mut sin)?;
    let program = Program::new(&mut sin.as_str());
    program.start();
    Ok(())
}
