fn main() {

    // Stack - организованная система хранение данных (как книжная полка с которой последняя поставленная книга забирается первой).
    // Данные с стеке находятся в строгом порядке. На этапе компиляции на программе, размер стека уже фиксирован, т.е. известен для программы
    // и операции по работе со стеком являются наиболее быстрыми из всевозможных -> 'вдавливание' и 'выдавливание' элементов из стека
    // (push -> добавить элемент из стека, pop - забрать элемент, peek - посмотреть верхний элемент не меняя структуру стека, isEmpty - проверить наличие элементов в стеке)

    //Heap (Куча) - организована по обратному от стека принципу. Куча нужна для записи больших размеров данных, размера которых мы точно не можем знать на этапе компиляции.
    // Выделение пространства в куче это более медленная операция чем работа со стеком, т.к. операционной системе необходимо время для того чтобы найти и выделить n-amount памяти и потом записать туда данные

    //ПРАВИЛО ВЛАДЕНИЯ: в Rust - в любой момент времени выполнения программы, у каждого значения переменной должен быть владелец и он может быть только 1!
    // let num = 5; // -->> здесь мы не просто создали переменную, здесь мы создали владельца данного числа 5. Владелец числа 5 это num.
    //ПРАВИЛО ВЛАДЕНИЯ: Владелец и значение перестают существовать когда значение выходит из области видимости, компилятор очищает память и значение с владельцем удаляются
    if num == 5 {
        let str = String::from("Hello"); // let str имеет свою область видимости ограниченную блоком if, после выполнения этого блока зеачение владелец будут удалены из памяти
    }

    let a = 1; // значение 1 хранится в стеке т.к. int - пример простого значения, которое копируется, а не передаются по ссылке
    let b = a; // не вызовет ошибки компиляции т.к. в переменную b присваеивается копия значения a а не само значение, и того мы получаем 2 владельцев двух значений лежащих по разным адресам памяти
    println!("{a}, {b}");

    let str1 = String::from("TEXT"); // значение "TEXT" хранится в куче, и данные из кучи не копируются без явного указания, а передаются по ссылке, чтобы избежеть огромных потерь во времени при компиляции
    let str2 = str1; // по факту здесь мы создали 2го владельца на одно и тоже значение --> приведет к ошибке компиляции 'value borrowed here after move'
    println!("{str1}, {str2}"); // иметь 2 переменные на одни и те же данные черевато ошибкой двойного высвобождения, т.к. в данном случае, после выхода переменных из области видимости, память будет высвобождаться дважды,
                                //сначала на переменную str1 и потом на переменную str2. При этом экземпляров данных у нас всего 1! Именно поэтому Rust запрещает подобное поведение и помогает избежать данной проблемы.

    let str3 = str1.clone(); // тут мы явно указываем компилятору что хотим сохранить копию строки, т.е. создать ещё одни такие же данные и присвоить им владельца "str3".
                                    // т.к. эта операция выполняется на куче, она будет достаточно затратна по скорости, и всегда стоит помнить и думать действительно ли целесообразно ее проводить


    //Типы данных хранящиеся в стеке, а значит копируются (т.к. операция копирования в стеке не является затратной в силу того что компилятору уже заранее известен размер стека):
    // Все целые числа (int8, int32, int64
    // Все booleans
    // Все floats  f32, f64
    // Chars
    // Tuples (только те кортежи которые содержат вышеперечисленые простые типы) -> если кортежи содержат строки или иные сложные типы данных, они переезжают на хранение в кучу


    let str4 = String::from("NEW INTERESTING STRING");
    print_value(str4); // Здесь, функуия 'print_value' забрала владенее переменной str4 -> по факту, т.к. мы зеаем что данные хранящиеся на куче перемещаются от 1 владельца к другому, то
                            // при вызове функции print_value(str4) мы по сути передали владение от str4 из области видимости main в значение str4 области видимости функции print_value.
                            // Т.е. больше str4 в области видимости main для компилятора не существует, и так же и переменная str4 области видимости функции и ее значение  пропадут после вызова функции

    let str4 = String::from("AGAIN INTERESTING STRING");
    print_value(str4.clone()); // Кривой но работающий способ не трогать переменную str4 области видимости main


    let number = 9;
    print_number(number);
    println!("{}", number); //число останется тем же что и при присваивании владельца, т.к. все события происходят в стеке и функция print_number() присваивает копию числа своей переменной number


    let s = give(); // Функция может так же возвращать владение, а не только забирать его. В этой строке мы передаем владение строкой "Hello" из функции give для переменной s

    let str5 = String::from("Charming new string");
    let str6  = take_and_give(str5); // Функция принимает и отдает владение

    // -----------------------------------------------------------------------------------------//
    // Ссылки --> используются в большинстве случаев если мы не хотим перемещать значение передавая его в другое владение, а хотим просто работать с ним напрямую.
    // Ссылки могут немутабельными (только на чтение) и их может быть много в один момент времени
    // Ссылки могут быть мутабельными (разрешать перезапись значения) и такая ссылка может быть только одна в определённый момент времени


    let str7 = String::from("It is the most interesting text ever");
    println!("{}", calc_len(&str7)); // Здесь мы передаем не саму переменную, а всего лишь ссылку на неё, в этом случае у нас не происходит передачи владения над переменной в функцию


    let mut str8 = String::from("Now, look: ");
    change_str(&mut str8); // здесь мы по мутабельной ссылке меняем переменную str8 через работу функции change_str
    println!("{}", str8);


    let mut name = String::new(); //создаём мутабельную переменную в которая будет владеть строкой
    std::io::stdin().read_line(&mut name).expect("ERR"); // данная строка будет формироваться путем считывания инпута пользователя

    let mut some_string = String::from("New string is here");
    let ref_1 = &mut some_string;
    let ref_2 = &mut some_string;
    println!("{}, {}", ref_1, ref_2); // подобное поведение ЗАПРЕЩЕНО в Rust, так как может вызвать гонку данных - ситуацию при которых мы имеем более одной мутабельной ссылки на одно и тоже значение,
                                      // а значит, можем поменять это значение из одного конца программы, в то время как на другом конце программы будет ожидаться исходное значение по ссылке находящейся во владении второй переменной
                                      //Либо что еще хуже, программа может попытаться в одно и то же время записать разные значения в одну область памяти, что приведёт к краху программы и серьёзным уязвимостям (короч беда была бы)


    let ref_3 = &some_string;
    let ref_4 = &some_string;
    let ref_5 = &some_string; //
    println!("{}, {}, {}", ref_3, ref_4, ref_5); // Будет работать без каких-либо проблем, т.к. ссылки являются немутабельными, разрешают только читать инфу, и не ограничены в кол-ве в один момент

    ref_6 = &some_string;
    ref_7 = &mut some_string;
    println!("{ref_6}, {ref_7}"); //вызовет ошибку, т.к. имея ссылку на чтение, мы уже не можем открывать ссылку на запись -> в один момент может быть либо сколько угодно ссылок на чтение либо ТОЛЬКО 1 ссылка на запись!


    //------------------------------------------------------------------------------------------//
    //  ДВА хороших примера на ссылки + владение

    //Пример - 1:
    let mut str_10 = String::from("Text");
    let x = true;
    let link_to_ref_10 = &mut str_10;

    if x == true {
        let ref_11 = &mut str_10;
        println!("{}", ref_11);
    }

    println!("{}", ref10); // Приведёт к падению программы т.к. в момент создания переменной в 106 строке мы уже имеем другую переменную владеюшую теми же данными созданными в строке 103



    //Пример - 2:
    let mut str_10 = String::from("Text");
    let x = true;

    if x == true {
        let ref_11 = &mut str_10;
        println!("{}", ref_11);
    }
    let link_to_ref_10 = &mut str_10;

    println!("{}", ref10); // Отработает без ошибок, т.к. после исполнения блока if переменная ref_11 и ее значения перестанут существовать (см. третье правило владения в rust),
                           // поэтому мы спокойно можем передавать мутабельную ссылку на существующие значения в новую переменную




}




fn print_value(value: String) {
    println!("{}", value);
}


fn print_number(num: i32) {
    println!("{}", num);
}

fn give() -> String {
    String::from("Hello")
}

fn take_and_give(s: String) -> String {
    s
}

fn calc_len(s: &String) -> usize {
    s.len()
}

fn change_str(str: &mut String) {
    str.push_str("Here we passed mutable link to change the parameter");
}