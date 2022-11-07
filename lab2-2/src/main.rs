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


fn create_blank_file(path: &Path){
    let _file = File::create(path).expect("El archivo no pudo crearse");
    println!("El archivo fue creado");

}


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
    let mut entrada: String = "".to_string();
    let mut _numero_de_entrada: u32 = 0;
    println!("mucho texto");
    
    loop {
        stdin().read_line(&mut entrada).unwrap();
        _numero_de_entrada = entrada.trim().parse().unwrap();

        match _numero_de_entrada {
            0|1|2|3|4 => break,
            _ => continue
        }
        println!("error")
    }
    
    return _numero_de_entrada;
}


fn menu_again() -> u32 {
    let mut entrada: String = "".to_string();
    let mut _numero_de_entrada: u32 = 0;
    println!("otro");
    
    loop {
        stdin().read_line(&mut entrada).unwrap();
        _numero_de_entrada = entrada.trim().parse().unwrap();

        match _numero_de_entrada {
            0|1|2|3|4 => break,
            _ => continue
        }
        println!("error")
    }
    /*

     */
    return _numero_de_entrada;
}


fn agregar_medicamento(path: &Path) {
    let file = open_file_to_append(path);
    let mut temp: String = String::new();
    }   

    loop {
        for i in 0..5 {
            stdin().read_line(&mut temp).unwrap();

            match i {
                0 => format!("")
                
            }
            /*
            codigo: String,
            nombre: String,
            componente: String,
            precio: u32,
            lab: String
            */
        }
        break;
    }

    println!("denme dinero")
}


fn consultar_precio(path: &Path) {

}


fn listar_laboratorio(path: &Path) {

}


fn listar_nombre(path: &Path) {

}


fn main() {
    let path: &Path = Path::new("base_de_datos.txt");
    let mut opcion = menu();

    loop {
        match opcion {
            1 => agregar_medicamento(path),
            2 => consultar_precio(path),
            3 => listar_laboratorio(path),
            4 => listar_nombre(path),
            _ => break
        }

        opcion = menu_again()
    }
}
