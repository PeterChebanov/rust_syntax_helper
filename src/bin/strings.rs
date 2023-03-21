fn main() {

    let s: String = String::new(); //создание строкового экземпляра в который можно записывать впоследующем
    let s1: String = String::from("Hello"); //создаем тип данных строка, со значением переданным в from
    let s2: String = "Hello, again".to_string();  // тоже самое что в предыдушей строчке

    println!("{}, {}, {}", s, s1, s2);

    let mut s3: String = "Hello, again for ".to_string();

    s3.push_str("everybody here "); //добавляем строку в конец строки
    println!("{}", s3);

    let g: &str = " who has this beautifully symbols: "; //&строковый литерал - срез от глобальной String
    s3.push_str(g);
    println!("{}", s3);

    s3.push('$'); //добавляет один чар в конец строки (не может добавить строку)
    println!("{}", s3);


    //2 способа конкатенации строк:
    let str1 = "Let's have a quick ".to_string();
    let str2 = "dialog".to_string();
    let str3 = "you little LoL".to_string();

    // 1 способ:
    let concat_result = str1 + &str2; //для конкатенации по правилам Rust мы должны взять первую строчку
    // к которой добавляем конкатенацию во владение, после чего просто прибавить к ней срез той строки
    // которую собираемся добавить
    println!("{}", concat_result);

    let concat_result_1 = concat_result + ", and do not even try to say 'No', " + &str3;
    println!("{}", concat_result_1);

    //2 способ:
    let str4 = "This is".to_string();
    let str5 = "result of".to_string();
    let str6 = "which you can see right here ".to_string();

    let res: String = format!("{} {} concatination {}", str4, str5, str6); //macros format! не берет переменные во владение
    //данный вид создания строк является приоритетным в Rust
    println!("{}", res);

    //обращение к элементам строки -->> !!! в Rust нельзя так же как в Python обращаться напрямую к
    // строке, т.к. строка представленна в виде байтов и срез мы можем брать так же в виде байтов,
    // но если каждый символ латиницы занимает 1 байт, то каждый символ кириллицы занимает уже 2
    // байта, и чтобы взять букву L из &str = LOL -> &str[..1], но уже в "ЛОЛ" чтобы получить первую
    // Л нам нужен будет срез &str[..2]. Взятие нечентного среза в этом случае прведет к ошибке


    //Работа с чарами:

    let str7 = "String to be cut on chars".to_string();

    let chars_of_string = str7.chars();
    println!("{:?}", chars_of_string);

    for char in chars_of_string {
        println!("{}", char);
    }

    let str8 = "ТЕКСТ".to_string();
    let str9 = "TEXT".to_string();

    let bytes_of_string_kyr = str8.bytes();
    let bytes_of_string_lat = str9.bytes();

    println!("{:?}", bytes_of_string_kyr); //{ it: Iter([208, 162, 208, 149, 208, 154, 208, 161, 208, 162]) } 10 byte => kyryllic
    println!("{:?}", bytes_of_string_lat); // { it: Iter([84, 69, 88, 84]) } 4 byte -> latin





}