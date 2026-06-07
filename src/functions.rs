use std::process::Command;
use std::io;
use std::io::Write;

macro_rules! temizle {
    () => {
        let cmd = if cfg!(target_os = "windows") { "cls" } else { "clear" };
        Command::new(cmd).status().ok();
    };
}

pub fn oku(prompt: &str) -> Option<String> {
    println!("{}", prompt);
    let mut girdi = String::new();
    io::stdin().read_line(&mut girdi).expect("standard input hatası");
    let girdi = girdi.trim().to_string();
    if girdi == "exit" { None } else { Some(girdi) }
}
pub fn ping_funcu() {
    loop {
            println!("Bir adres gir (genelde '8.8.8.8' veya resolver problemin olduğunu düşünüyorsan 'google.com') yazman önerilir...)\n");
            println!("Çıkmak için exit yaz...\n\n");
            let Some(adres) = oku("secimin:") else { println!("Çıkılıyor"); break; };
            if adres == "clear" { temizle!(); continue; }
            Command::new("ping").arg("-c").arg("4").arg(&adres).status().expect("ping err");
        }
}
pub fn disk_bağlayıcı() {
    println!("Hoş geldin sanırım disk bağlamak istiyorsun seçimin ne?");
    loop {
        temizle!();
        println!("Işte diskin ve partisyonların:");
        Command::new("lsblk")
            .arg("-f")
            .status()
            .expect("KOMUT YOK");
        //bura da bitti geçelim diğer işe...

        let Some(disk) = oku("Hangi diski seçeceksin? ( Başına /mnt yazmana gerek yok.)") else { break; };
        let disk_yolu = format!("/dev/{}", disk);
        if !std::path::Path::new(&disk_yolu).exists() {
            println!("Disk bulunamadı...\n");
            continue;
        }

        temizle!();
        let Some(klasor) = oku("Hangi klasöre bağlamak istiyorsun?") else { break; };
        let klasor_yolu = format!("/mnt/{}", klasor);

        if !std::path::Path::new(&klasor_yolu).exists() {
            Command::new("mkdir").arg("-v").arg(&klasor_yolu).status().expect("klasör oluşturalamadı");
        }
        temizle!();

        match Command::new("mount").arg("-v").arg(&disk_yolu).arg(&klasor_yolu).status() {
            Ok(s) if s.success() => { println!("Herşey Tamamlandı!"); break; }
            Ok(_) => println!("Bağlanamadı.Programı sudo ile çalıştır."),
            Err(e) => println!("Bağlanamadı.Hata: {}", e),
        }
    }
}
pub fn shell() {
    loop {
        print!("%");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Couldnt read vro ::/(");

        let input = input.trim();

        if input.is_empty(){
            print!("HATA");
            continue;
        }

        if input == "exit" {
            break;
        }

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match Command::new(command).args(args).status() {
            Ok(_) => {},
            Err(_) => eprintln!("Komut bulunamadı. {}", command)
        }
    }
}
