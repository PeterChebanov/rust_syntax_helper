use std::fmt::Debug;

#[derive(Debug)]
enum Types {
    Int(i32),
    Float(f64),
    Bool(bool),
    Text(String)
}


fn main() {
    // Векторы -> по сути динамический массив, хранится в куче, т.к. мы не знаем заранее его размер
    // Векторы могут хранить данные только одного типа

    // 1-ый способ объявления вектора (наиболее используемый)
    let mut list = vec![1,3,5]; // при объявлении вектора мы должны создать его с элементами, для
    //того чтобы компилятор знал тип его значений
    println!("{:?}", &list);
    println!("{:?}", list);

    list.push(11);
    list.remove(1);
    println!("{:?}", list);


    //чтобы создать пустой вектор необходимо явно указать тип его данных в начале
    let mut list1:Vec<i32> = Vec::new();
    list1.push(121);
    list1.push(14);
    list1.push(152);
    println!("{:?}", list1);


    let list2 = vec![11, 12, 14, 15, 13, 16, 17, 22];
    println!("{}", &list2[0]);
    println!("{}", &list2[3]);
    println!("{:?}", &list2[..=3]); //в случае подобных вызовов элемента вектора, если мы укажем номер
    //элемента которого не существует в векторе, будет вызвана паника, и программа аварийно завершится
    //c ошибкой

    // если мы не хотим получить аварийное завершение программы то стоит использовать функцию get()
    // которая позволяет обрабатывать подобные исключения, и аппелирует 2мя сущностями Some и None
    // в случае если елемент существует по переданному индексу, будет возвращён Some(12) - где 12 -
    // значение элемента по указанному индексу, в противном случае будет возвращён None

    println!("{:?}", list2.get(1)); // Вернет 12
    println!("{:?}", list2.get(123)); // Вернет None

    match list2.get(6) {
        Some(el) => {
            println!("Element with index 6 is {}", el);
        },
        None => todo!()
    }


    for el in &list2 { //не забываем передать ссылку на коллекцию а не саму коллекцию, иначе
                            //больше мы не сможем ее использовать в дальнейшем по правилам владения
        println!("{}", el);
    }

    for el in list2.iter() {//либо можем использовать метод iter, который под капотом принимает
                                  //передаёт ту же ссылку
        println!("{}", el);
    }

    //--------------------------------------------------------------------------------------------//
    // Хранение различных типов данных в векторе через enum
    let list_of_different_types = vec![
        Types::Bool(true),
        Types::Float(3.129),
        Types::Int(177),
        Types::Bool(true),
        Types::Text("Hello, amazing new world!!".to_string())
    ];
    println!("{:?}", &list_of_different_types[2]); //Int(177) -> непонятно пока что можно ли это конвертировать в просто значение 177...



   fn retreive_value_of_vect(value: &Types) -> (i32, f64, bool, String) {
       match value {
           Types::Int(a) => {
               (*a, 0.00, false, " ".parse().unwrap())
           },
           Types::Float(b) => {
               //some logic is here
           }
           Types::Bool(c) => {
               //some logic is here
           }
           Types::Text(d) => {
               //some logic is here
           }
       }
        //Странная получилась функция извлечения элементов из вектора перечислений, специально ее пока не дописываю в надежде позже найти более элегантный вариант
   }



}