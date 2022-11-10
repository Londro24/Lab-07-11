use std::fs::File;
use std::path::Path;
use std::io::Read;

//lee el archivo, entregando un texto
fn read_file(mut f: &File) -> String {
    let mut text = String::new();
    f.read_to_string(&mut text).unwrap();
    return text
}

//crean un archivo en blanco
fn create_blank_file(p: &Path){
    let _file = File::create(p).expect("El archivo no pudo crearse");
    println!("El archivo fue creado");

}

//abre el archivo entrgando un texto
fn open_file(p: &Path) -> String{
    let mut text = "".to_string();
    if Path::new(p).exists(){
        let file = match File::open(&p){
            Err(_why) => panic!("El archivo no se puede abrir..."),
            Ok(file) => file,
        };
        text = read_file(&file);
    } else {
        create_blank_file(p);
    }
    return text
}

//hace match
fn match_text(text: &str) -> u32 {
    let mut puntaje: u32 = 0;
    for a in text.tolowercase().chars() {
        match a {
            'a'|'e'|'i'|'o'|'u'|'l'|'n'|'r'|'s'|'t'|'ñ'|'á'|'é'|'í'|'ó'|'ú' => puntaje += 1,
            'd'|'g' => puntaje += 2,
            'b'|'c'|'m'|'p' => puntaje += 3,
            'f'|'h'|'v'|'w'|'y' => puntaje += 4,
            'k' => puntaje += 5,
            'j'|'x' => puntaje += 8,
            'q'|'z' => puntaje += 10,
            => continue,
        }
    }
    println!("");
    return puntaje
}


fn main() {
    let path: &Path = Path::new("archivo.txt");
    let text = open_file(path);

    let puntaje = match_text(&text);
    println!("{}", puntaje)
}