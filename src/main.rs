use rotors::Rotor;

mod rotors;


fn main(){
    
    let default_rotors:[Rotor;8] = Rotor::load_default_rotors();
    print!("{}",default_rotors[2].name);
}