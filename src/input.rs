//input.rs
use std::process::Command;

macro_rules! temizle {
    () => {
        let cmd = if cfg!(target_os = "windows") { "cls" } else { "clear" };
        Command::new(cmd).status().ok();
    };
}

pub fn input() {
    loop {
            //alr bet bir greeter ekleyelim ski lesgo
        println!("
        ====================================
        HOŞ GELDIN 1-5 ARASI BIR SAYI GIR...
        ====================================
        1-Ping aracı
        2-Disk Bağlayıcı
        3,4,5-under construction
        exit-çıkış
        ");
        let Some(input) = crate::functions::oku("Seçimin?") else { break; };
        //umarım bu fonksiyonları bir gün yazabilirm :/
        match input.trim() {
            "1" => { temizle!(); crate::functions::ping_funcu(); }
            "2" => { temizle!(); crate::functions::disk_bağlayıcı(); }
            "3" => {  }
            "4" => {  }
            "5" => {  }
            _   => println!("Geçersiz giriş, tekrar dene"),
        }
    }
}
