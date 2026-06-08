/*
    Ownership RUST!! 
*/

fn main() {
    println!("Bienvenido al Ownership!!!!!!!!!!!");

    /* Esto es una variable */
    let palabra: &str = "Holis";

    /* Esto es un bloque en donde si todo esta dentro, pues nada sale. */
    {
        let palabra2: &str ="Holis";
        println!("{palabra2}")
    }

    println!("{}", palabra);

    /* Otro bloque pero lo sacamos */
    let palabra3: &str;
    {
        palabra3 = "WOOOOW";
    }

    println!("Imprimido, oh yeah: {}", palabra3);

    println!("-----------------------------");

    /* String */

    /* Se define una variable mutable, con string::from se asigna un lugar en la memoria */
    let mut holamundo =String::from("hola");
    /* el .push_str agarra ese string y le agrega con otra palabra str o string*/
    holamundo.push_str(", mundo :)");
    /* aqui se imprime */
    println!("{}", holamundo);

    /* Alcance y asignacion */
    let mut ok = String::from("Hola");
    /* Se modifica el string por uno nuevo */
    ok = String::from("Neh");
    /* Se imprime aqui */
    println!("{ok}, mundo");


    /* Igual ocurre lo mismo, solo que se clona "hola" */
    let s1: String = String::from("hola");
    let s2: String = s1.clone();

    /* igual se imprime */
    println!("s1 = {s1}, s2 = {s2}");


    /* ahora copiamos aqui, se copia el mismo numero aunque sea variable inmutable*/

    let numero1: i32 = 6;
    let numero2: i32 = numero1;

    println!("{numero2}");

    println!("-----------------------------");

    /* Siguiente apartado con ownership pero con funciones ;) */

    let a: &str = "holabandamax000000";
    holabandamax(a);
}

/* Funciones personalizadas */

/* se crea una funcion llamada holabandamax con valores string para imprimir
    el valor de a en let a: &str = "holabandamax000000";. por eso se crea fn holabandamax y despues se agrega el ();
*/
fn holabandamax(mi_string: &str) {
    println!("{mi_string}");
}