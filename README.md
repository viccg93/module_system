Un crate es la unidad minima de un progrma que puede ser compilada
aunque usemos rustc, pasando un solo archivo, este se considera un crate

un crate puede contener uno o mas modulos.

existen 2 tipos de crate
- binary crate: programas que se compilan en ejecutables y deben tener un main
- library crate: son librerias que tienen la finalidad de proveer de funcionalidad y no de ejecutables, rand es un ejemplo

Generalmente los rustaceans usan el termino crate para referirse a librerias, es decir binary crates

El crate root es un archivo fuente que indica al compilador donde marcar el modulo root de tu crate

Un paquete es un conjunto de uno o mas crates y contiene un Cargo.toml que define como construir esos crates

Cargo es un paquete que contiene el binary crate para el CLI que usamos para construir nuestros proyectos
ademas el paquete de Cargo tambien contiene un library crate con las dependencias de ese CLI
otros proyectos pueden depender del library crate de cargo para implementar funcionalidad similar

Un paquete debe contener al menos un crate, sea binary o library

Ademas un paquete puede contener tantos binary crates como sea necesario pero a lo mucho un library crate

Cuando creamos un nuevo proyecto con cargo crea src/main.rs, y aunque no existe referencia en el Cargo.toml
esta es una convencion para un binary crate y dicha direccion es el crate root

cuando es un library crate, la direccion por convencion es src/lib.rs y esta ruta es el crate root.

El crate root es la ruta que cargo pasa a rustc para contruir el binario o la libreria.

Cuando un proyecto tiene un binary y un library, tiene como resultante 2 crates distintos y en el caso
que tuviera n binary crates, por cada uno se genera un crate distinto.


* Modules Cheatsheet

- inicio en el crate root: el compilador busca primero en src/main.rs y src/lib.rs a la hora de compilar
- declaracion de modulos: el compilador busca los modulos de acuerdo a los siguientes criterios
    para un modulo declarado como mod garden

    - inline, con curly brackets despues de mod garden, emplazando el semicolon
    - en src/garden.rs
    - en src/garden/mod.rs

- declaracion de sub modulos
    Es necesario declarar el modulo mod vegetables en src/garden.rs, la declaracion es posible en cualquier modulo menos el root crate
    y el compilador buscara el sub-modulo de acuerdo a los siguientes criterios:

    - inline, despues de mod vegetables dentro de curly brackets reemplazando el semicolon
    - en src/garden/vegetables.rs
    - en src/garden/vegetables/mod.rs

- rutas al codigo de los modulos
    una vez que se ha creado un modulo y es parte del crate, puede ser utilizado desde cualquier lugar de ese crate siempre y cuando
    las reglas de privaciad lo permitan, por ejemplo el tipo Arparagus puede ser llamado desde crate::garden::vegetables::Asparagus

- privado vs publico
    el codigo en un modulo es privado para los modulos parent del mismo por defecto, para declarar un modulo publico se debe usar
    pub mod en lugar de mod, los miembros de un modulo publico deben marcarse con pub en sus declaraciones.

- palabra clave use
    en un scope determinado podemos usar use para reducir la repeticion de paths largos por ejemplo
    al usar:

        use crate::garden::vegetables::Asparagus;

    podemos llamar a Asparagus sin tener que repetir toda la ruta, en el scope del use

El codigo de esta estructura se encuentra en el proyecto backyard
    
