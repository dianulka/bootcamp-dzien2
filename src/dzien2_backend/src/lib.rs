use std::cell::RefCell;

//sekcja do przechowywania danych
// dane będą przetwarzane lokalnie w naszym programia
// static - miejsce w pamieci tworzymy w momencie uruchmunia naszego kodu i będzie cały czas w pamięci czekało, będzie żyło cały 
// czas
thread_local! {
    static WPISY: RefCell<Vec<String>> = RefCell::default();
    // RefCell pozwala na odwoływanie się do zmiennych, a nie do ich kopii, referencja
}


#[ic_cdk::query]
fn greet(name: String, surname: i8) -> String {
    format!("Hello, {} {}!", name, surname)
}

//update zeby zapisac na wszystkich komputerach
#[ic_cdk::update]
fn dodaj_wpis(wpis: String) {
    WPISY.with(|wpisy| {
        wpisy.borrow_mut().push(wpis)
    });

    // Na WPISY uruchami metode with gdze rzecza ktora daje zmienna 

    // // przykład
    // let zmienna: String = String::from("Cześć");
    // let zmienna2 = zmienna;
    // let zmienna3: String = zmienna; // zmienna się zużyła

    // let zmienna: String = String::from("Cześć");
    // let zmienna2 = &zmienna; // pożyczmy zmienna2 dla zmienna
    // let zmienna2 = &zmienna; // pożyczmy zmienna2 dla zmienna
    // zmienna2.as_mut(); //
    // let zmienna3: String = zmienna;

}

//query do odczytu
#[ic_cdk::query]
fn odczytaj_wpisy() -> Vec<String> {
    WPISY.with(|wpisy| {
        wpisy.borrow().clone() //borrow pozycza zmienna, clone klonuje
    }) 
}

// bierzemy pudelko wpisy wyciagamy wpisy pozyczamy je zeby mozna bylo je uzyc ,
// kopiujemy uzywajac clone

#[ic_cdk::update]
fn usun_wpis(id_wpisu: usize) { 
    WPISY.with(|wpisy|{
        wpisy.borrow_mut().remove(id_wpisu);
    })
}

#[ic_cdk::update]
fn edytuj_wpis(id_wpisu: usize, nowy_wpis: String) {
    WPISY.with(|wpisy|{
        let mut binding = wpisy.borrow_mut();
        let wpis = binding.get_mut(id_wpisu);
        //get_mut pobranie zmiennej jako edytowalna
        let stary_wpis = wpis.unwrap();
        *stary_wpis = nowy_wpis;
    })
}




