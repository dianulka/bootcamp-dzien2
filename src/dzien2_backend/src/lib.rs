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

#[ic_cdk::update]
fn dodaj_wpis(wpis: String) {
    WPISY.with(|wpisy| {
        wpisy.borrow_mut().push(wpis)
    });

    // Na WPISY uruchami metode with gdze rzecza ktora daje zmienna 
}

