use std::io::stdin;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::fs::OpenOptions;
use std::io::Write;


#[derive(Default)]
struct Medicamento{
    codigo: String,
    nombre: String,
    componente: String,
    precio: String,
    lab: String
}


fn read_file(mut file: &File) -> String {
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    return text
}
// Lee el archivo y lo devuelve en String

fn create_blank_file(path: &Path){
    let _file = File::create(path).expect("El archivo no pudo crearse");
    println!("El archivo fue creado");

}
// Crea el archivo

fn is_entero_positivo(numero: &str) -> bool {
    for digit in numero.to_string().trim().chars(){
        if digit.is_numeric(){
            continue
        } else {
            return false
        }
    }
    return true
}
// Revisa si es un numero entero

fn open_file_to_append(path: &Path) -> File{
    open_file(path);
    let mut binding = OpenOptions::new();
    let binding = binding.append(true);
    let file = match binding.open(path){
        Err(_why) => panic!("No se puede abrir el archivo"),
        Ok(file) => file,
    };
    return file
}


fn open_file(path: &Path) -> String{
    let mut text = "".to_string();
    if Path::new(path).exists(){
        let file = match File::open(&path){
            Err(_why) => panic!("El archivo no se puede abrir..."),
            Ok(file) => file,
        };
        text = read_file(&file);
    } else {
        create_blank_file(path);
    }
    return text
}


fn menu() -> u32 {
    let mut entrada: String = String::new();
    loop {
        println!("Elija opción:");
        println!("(1) Agregar un medicamento nuevo");
        println!("(2) Consular precio por código del medicamento");
        println!("(3) Listar medicamentos por laboratorio");
        println!("(4) Listar medicamentos por nombre");
        println!("(0) Salir");
        stdin().read_line(&mut entrada).unwrap();
        //
        if !is_entero_positivo(&entrada) || entrada.trim() == "".to_string() {
            entrada = "".to_string();
            continue
        }
        //
        match entrada.trim().parse().unwrap() {
            0|1|2|3|4 => break,
            _ => entrada = "".to_string()
        }
        println!("\nIntentelo denuevo\n");
        continue
    }   
    let num: u32 = entrada.trim().parse().unwrap();
    return num
}


fn pedir_medicamento() -> String {
    let mut linea = "".to_string();
    //
    for i in 0..5 {
        loop {
            let mut temp: String = String::new();
            match i {
                0 => println!("Escriba el CÓDIGO del producto") ,
                1 => println!("Escriba el NOMBRE del producto"),
                2 => println!("Escriba el COMPONENTE PRINCIPAL del producto"),
                3 => println!("Escriba el PRECIO del producto"),
                4 => println!("Escriba el LABORATORIO del producto"),
                _ => continue
            };
            stdin().read_line(&mut temp).unwrap();
            //
            if temp.trim() == "".to_string() {
                continue
            }
            //
            if i == 3 {
                if is_entero_positivo(&temp) {
                    linea = linea + &format!("{}", &temp.trim());
                    break
                } else {
                    continue
                }
            }
            //
            linea = linea + &format!("{}", &temp.trim().to_uppercase());
            break
        }
        if i != 4 {
            linea = linea + ":";
        }
    }
    return linea
}


fn revisar(text: &str, linea: &str) -> bool {
    for lineas in text.split("\n") {
        for dato in lineas.split(":") {
            for a in linea.split(":") {
                if dato == a {
                    if linea.trim() == lineas.trim(){
                        break
                    } else {
                        return false
                    }
                } else {
                    break
                }
            }
            break
        }
    }
    return true
}


fn agregar_medicamento(path: &Path) {
    loop {
        let linea: String = pedir_medicamento().to_string();
        let text: String = open_file(path);
        if revisar(&text, &linea) {
            let mut file: File = open_file_to_append(path);
            file.write_all(linea.as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
            break
        }
        println!("\nMedicamento no válido\n")
    }    
}


fn consultar_precio(path: &Path) {
    let mut codigo: String = String::new();
    println!("Escriba el CÓDIGO del medicamento");
    stdin().read_line(&mut codigo).unwrap();
    codigo = codigo.to_uppercase();
    let text: String = open_file(path);
    let mut es_medicamento = false;
    //
    for a in text.split("\n") {
        let mut contador = 0;
        es_medicamento = false;
        for b in a.split(":") {
            if contador == 0 && b == codigo.trim() {
                es_medicamento = true;
            }
            if contador == 3 && es_medicamento {
                println!("\nEl precio es: ${}\n", b)
            }
            contador += 1;
        }
        if es_medicamento {
            break
        }
    }
    if !es_medicamento {
        println!("\nMedicamento no encontrado\n")
    }
}


fn listar_laboratorio(path: &Path) {
    println!("Yametekudasai")
}


fn listar_nombre(path: &Path) {
    println!("Yametekudasai")
}


fn main() {
    let path: &Path = Path::new("base_de_datos.txt");
    //
    loop {
        let opcion = menu();
        match opcion {
            1 => agregar_medicamento(path),
            2 => consultar_precio(path),
            3 => listar_laboratorio(path),
            4 => listar_nombre(path),
            _ => break
        }
    }
}
