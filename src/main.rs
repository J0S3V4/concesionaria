use std::io;
use chrono::Local;
#[derive(Debug, PartialEq, Eq)]
struct Color {
    nombre: String,
}


#[derive(Debug, PartialEq, Eq)]
struct Marca {
    nombre: String,
}


#[derive(Debug, PartialEq, Eq)]
struct Modelo {
    nombre: String,
    marca: String,
}


#[derive(Debug)]
struct Vehiculo {
    modelo: String,
    marca: String,
    color: String,
    precio: f64,
}
#[derive(Debug)]
struct Compra {
    vehiculo: Vehiculo,
    fecha: String,
}

fn main() {
    let mut colores: Vec<Color> = Vec::new();
    let mut marcas: Vec<Marca> = Vec::new();
    let mut modelos: Vec<Modelo> = Vec::new();
    let mut vehiculos: Vec<Vehiculo> = Vec::new();
    let mut compras: Vec<Compra> = Vec::new();

    loop {
        println!("Selecciona una opción:");
        println!("1.Sistema Color");
        println!("2.Sistema Marca");
        println!("3.Sistema Modelo");
        println!("4.Sistema Vehiculos");
        println!("5.Sistema Compra y Venta");
        println!("6.salir");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error al leer la entrada");
        let choice: u32 = input.trim().parse().expect("Por favor ingrese un número");

        match choice {
            1 => {
                loop {
                    println!("Selecciona una opción:");
                    println!("1. Agregar color");
                    println!("2. Eliminar color");
                    println!("3. Mostrar colores");
                    println!("4. Salir");
            
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Error al leer la entrada");
                    let choice: u32 = input.trim().parse().expect("Por favor ingrese un número");
            
                    match choice {
                        1 => {
                            println!("Ingrese el nombre del color:");
                            let mut color_nombre = String::new();
                            io::stdin()
                                .read_line(&mut color_nombre)
                                .expect("Error al leer la entrada");
                            let nuevo_color = Color {
                                nombre: color_nombre.trim().to_string(),
                            };
                            if !colores.contains(&nuevo_color) {
                                colores.push(nuevo_color);
                                println!("Color agregado exitosamente.");
                            } else {
                                println!("El color ya existe.");
                            }
                        }
                        2 => {
                            println!("Ingrese el índice del color a eliminar (comenzando desde 0):");
                            let mut index_str = String::new();
                            io::stdin()
                                .read_line(&mut index_str)
                                .expect("Error al leer la entrada");
                            let index: usize = index_str.trim().parse().expect("Por favor ingrese un índice válido");
                            if index < colores.len() {
                                colores.remove(index);
                                println!("Color eliminado exitosamente.");
                            } else {
                                println!("Índice inválido.");
                            }
                        }
                        3 => {
                            println!("Colores actuales:");
                            for (i, color) in colores.iter().enumerate() {
                                println!("{}: {}", i, color.nombre);
                            }
                        }
                        4 => break,
                        _ => println!("Opción inválida."),
                    }
                }
            }
            2=>{loop {
                println!("Selecciona una opción:");
                println!("1. Agregar marca");
                println!("2. Eliminar marca");
                println!("3. Mostrar marcas");
                println!("4. Salir");
        
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let choice: u32 = input.trim().parse().expect("Please enter a number");
        
                match choice {
                    1 => {
                        println!("Ingrese el nombre de la marca:");
                        let mut marca_nombre = String::new();
                        io::stdin()
                            .read_line(&mut marca_nombre)
                            .expect("Error al leer la entrada");
                        let nueva_marca = Marca {
                            nombre: marca_nombre.trim().to_string(),
                        };
                        if !marcas.contains(&nueva_marca) {
                            marcas.push(nueva_marca);
                            println!("Marca agregada exitosamente.");
                        } else {
                            println!("La marca ya existe.");
                        }
                    }
                    2 => {
                        println!("Ingrese el índice de la marca a eliminar (comenzando desde 0):");
                        let mut index_str = String::new();
                        io::stdin()
                            .read_line(&mut index_str)
                            .expect("Failed to read line");
                        let index: usize = index_str.trim().parse().expect("Please enter a valid index");
                        if index < marcas.len() {
                            marcas.remove(index);
                            println!("Marca eliminada exitosamente.");
                        } else {
                            println!("Índice inválido.");
                        }
                    }
                    3 => {
                        println!("Marcas actuales:");
                        for (i, marca) in marcas.iter().enumerate() {
                            println!("{}: {}", i, marca.nombre);
                        }
                    }
                    4 => break,
                    _ => println!("Opción inválida."),
                }
            }
        }
            3=>{loop {
                println!("Selecciona una opción:");
                println!("1. Agregar modelo");
                println!("2. Eliminar modelo");
                println!("3. Mostrar modelos");
                println!("4. Salir");
        
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Error al leer la entrada");
                let choice: u32 = input.trim().parse().expect("Por favor ingrese un número");
        
                match choice {
                    1 => {
                        println!("Ingrese el nombre del modelo:");
                        let mut modelo_nombre = String::new();
                        io::stdin()
                            .read_line(&mut modelo_nombre)
                            .expect("Error al leer la entrada");
                        println!("Ingrese la marca del modelo (asegúrese de que exista):");
                        let mut marca_modelo = String::new();
                        io::stdin()
                            .read_line(&mut marca_modelo)
                            .expect("Error al leer la entrada");
        
                        // Verificar si la marca existe
                        let marca_existe = marcas.iter().any(|m| m.nombre == marca_modelo.trim());
                        if marca_existe {
                            let nuevo_modelo = Modelo {
                                nombre: modelo_nombre.trim().to_string(),
                                marca: marca_modelo.trim().to_string(),
                            };
        
                            // Verificar si el modelo ya existe
                            if !modelos.contains(&nuevo_modelo) {
                                modelos.push(nuevo_modelo);
                                println!("Modelo agregado exitosamente.");
                            } else {
                                println!("El modelo ya existe.");
                            }
                        } else {
                            println!("La marca no existe.");
                        }
                    }
                    2 => {
                        println!("Ingrese el índice del modelo a eliminar (comenzando desde 0):");
                        let mut index_str = String::new();
                        io::stdin()
                            .read_line(&mut index_str)
                            .expect("Error al leer la entrada");
                        let index: usize = index_str.trim().parse().expect("Por favor ingrese un índice válido");
                        if index < modelos.len() {
                            modelos.remove(index);
                            println!("Modelo eliminado exitosamente.");
                        } else {
                            println!("Índice inválido.");
                        }
                    }
                    3 => {
                        println!("Modelos actuales:");
                        for (i, modelo) in modelos.iter().enumerate() {
                            println!("{}: {} ({})", i, modelo.nombre, modelo.marca);
                        }
                    }
                    4 => break,
                    _ => println!("Opción inválida."),
                }
            }}
            4=>{
                loop {
                    println!("Selecciona una opción:");
                    println!("1. Agregar vehículo");
                    println!("2. Mostrar vehículos");
                    println!("3. Eliminar vehículo");
                    println!("4. Salir");
            
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Error al leer la entrada");
                    let choice: u32 = input.trim().parse().expect("Por favor ingrese un número");
            
                    match choice {
                        1 => {
                            println!("Ingrese el modelo del vehículo:");
                            let mut modelo_nombre = String::new();
                            io::stdin()
                                .read_line(&mut modelo_nombre)
                                .expect("Error al leer la entrada");
            
                            println!("Ingrese la marca del vehículo:");
                            let mut marca_nombre = String::new();
                            io::stdin()
                                .read_line(&mut marca_nombre)
                                .expect("Error al leer la entrada");
            
                            println!("Ingrese el color del vehículo:");
                            let mut color_nombre = String::new();
                            io::stdin()
                                .read_line(&mut color_nombre)
                                .expect("Error al leer la entrada");
            
                            println!("Ingrese el precio del vehículo:");
                            let mut precio_str = String::new();
                            io::stdin()
                                .read_line(&mut precio_str)
                                .expect("Error al leer la entrada");
                            let precio: f64 = precio_str.trim().parse().expect("Error al parsear el precio");
            
                            // Verificar existencia
                            let modelo_existe = modelos.iter().any(|m| m.nombre == modelo_nombre.trim() && m.marca == marca_nombre.trim());
                            let color_existe = colores.iter().any(|c| c.nombre == color_nombre.trim());
            
                            if modelo_existe && color_existe {
                                let nuevo_vehiculo = Vehiculo {
                                    modelo: modelo_nombre.trim().to_string(),
                                    marca: marca_nombre.trim().to_string(),
                                    color: color_nombre.trim().to_string(),
                                    precio,
                                };
                                vehiculos.push(nuevo_vehiculo);
                                println!("Vehículo agregado exitosamente.");
                            } else {
                                println!("El modelo, la marca o el color no existen.");
                            }
                        }
                        2 => {
                            println!("Vehículos actuales:");
                            for (i, vehiculo) in vehiculos.iter().enumerate() {
                                println!("{}: {} {} {} $ {:.2}", i, vehiculo.marca, vehiculo.modelo, vehiculo.color, vehiculo.precio);
                            }
                        }
                        3 => {
                            println!("Ingrese el índice del vehículo a eliminar (comenzando desde 0):");
                            let mut index_str = String::new();
                            io::stdin().read_line(&mut index_str).expect("Error al leer la entrada");
                            let index: usize = index_str.trim().parse().expect("Por favor ingrese un índice válido");
                            if index < vehiculos.len() {
                                vehiculos.remove(index);
                                println!("Vehículo eliminado exitosamente.");
                            } else {
                                println!("Índice inválido.");
                            }
                        }
                        4 => break,
                        _ => println!("Opción inválida."),
                    }
                }
            }
            5=>{loop {
                println!("Selecciona una opción:");
                println!("1. Ver vehículos disponibles");
                println!("2. Comprar vehículo");
                println!("3. Ver historial de compras");
                println!("4. Salir");
        
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Error al leer la entrada");
                let choice: u32 = input.trim().parse().expect("Por favor ingrese un número");
        
                match choice {
                    1 => {
                        println!("Vehículos disponibles:");
                        for (i, vehiculo) in vehiculos.iter().enumerate() {
                            println!("{}: {} {} {} $ {:.2}", i, vehiculo.marca, vehiculo.modelo, vehiculo.color, vehiculo.precio);
                        }
                    }
                    2 => {
                        println!("Ingrese el índice del vehículo a comprar:");
                        let mut index_str = String::new();
                        io::stdin()
                            .read_line(&mut index_str)
                            .expect("Error al leer la entrada");
                        let index: usize = index_str.trim().parse().expect("Por favor ingrese un índice válido");
        
                        if index < vehiculos.len() {
                            let vehiculo_comprado = vehiculos.remove(index);
                            let compra = Compra {
                                vehiculo: vehiculo_comprado,
                                fecha: Local::now().to_string(),
                            };
                            compras.push(compra);
                            println!("Compra realizada exitosamente.");
                        } else {
                            println!("Índice inválido.");
                        }
                    }
                    3 => {
                        println!("Historial de compras:");
                        for compra in &compras {
                            println!("{:?}", compra);
                        }
                    }
                    4 => break,
                    _ => println!("Opción inválida."),
                }
            }}
            6 => break,
            _ => println!("Opción inválida."),
        }
    }
}
