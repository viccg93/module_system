//Hasta el moemnto solo hemos programado proyectos de un modulo y en un archivo
//los modulos nos permiten agrupar funcionalidad
//un paquete puede contener multiples binarios crate y adicionalmente una libreria crate
//tambien se usan namespaces y scope de los mismos para mantener un orden de los objetos que se utilizan evitando suplicidades o errores
//esta prganizacion permite mostrar u ocultar detalles de implementaciones, agrupar y externalizar funcionalidades
//traer determinados nombres al scope o retirarlos.

//los elementos principales de Rust son:
//packages: permiten build, test y compartir crates
//crates: arbol de modulos que produce una libreria o ejecutable
//modulos y use: permiten controlar la organizacion, scope y provacidad de los paths
//paths: forma de nombrado de items como structs, funciones o modulos.
fn main() {
    println!("Hello, world!");
}
