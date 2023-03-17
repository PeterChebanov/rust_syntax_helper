#[derive(Debug)]
struct Person {
    name: String,
    surname: String,
    age: i32,
    balance: f64
}

#[derive(Debug)]
struct PersonWithOtherFields {
    first_name: String,
    second_name: String,
    total_age: i32,
    total_money: f64
}

#[derive(Debug)]
struct TupleType(i32, String, f64); //другой тип структуры - кортежная структура

#[derive(Debug)]
struct TupleType_2(i32, String, f64); //ВАЖНО: несмотря на то что обе кортежные структуры содержат в себе
//одинаковые типы данных - это 2 АБСОЛЮТНО разных типа данных с т.з. Rust. Один тип данных - TupleType
// второй тип данных - TupleType_2. 

//Реализации методов структур
// Методы структуры - это методы которые привязаны к конкретной структуре и логически с ней связаны
// Например - у нас есть структура треугольника и мы хотим вычислить его гипотинузу -->> логически
// нам понятно что метод вычисления гипотинузы может относиться к треугольнику, и нигде больше в
// программе этот метод использоваться не будет. В этом случае, необходимо использовать метод структур

struct Triangle {
    cat1: f64,
    cat2: f64
}

impl Triangle { // объявление метода структуры с помошю impl + название структуры
    fn find_hyp(&self) -> f64 { // в сами методы мы только передаем self т.е. экземпляр структуры через
                                // который мы будем получать поля структуры
        (self.cat1 * self.cat1 + self.cat2*self.cat2).sqrt() // возвращаем значение
    }

    fn find_area(&self) -> f64 {
        0.5 * self.cat1*self.cat2
    }

    fn create_isc(cat: f64) -> Triangle { // мы так же можем создать что-то вроде статического метода
        //в данном случае, это метод создающий равнобедренный треугольник и возвращаюший его.
        Triangle{
            cat1: cat,
            cat2: cat
        }

    }
}






fn main() {

    //Структуры

    let person_1 = Person {
        name: "Petr".to_string(),
        surname: "Che".to_string(),
        age: 37,
        balance: 1.400,
    };
    println!("{:#?}", person_1);
    println!("Person name is {:?}", person_1.name);


    let mut mutable_person_1 = Person {
        name: "Chen".to_string(),
        surname: "Su Won".to_string(),
        age: 27,
        balance: 2220.0,
    };

    mutable_person_1.age = 26;
    println!("The mutable Chen's age was fixed to correct one: {}", mutable_person_1.age);

    //Синтаксис обновления структур:

    let total_age = 27;
    let total_money = 4000.12;

    let mut person_11 = PersonWithOtherFields {
        first_name: "Chen Su".to_string(),
        second_name: "Won".to_string(),
        total_age,
        total_money // если у нас есть уже инициализированные переменные которые совпадают по названию
                    // с полями структуры, можно напрямую использовать данный синтаксис
    };

    let mut person_12 = PersonWithOtherFields {
        first_name: "Tan Chu".to_string(),
        second_name: "We".to_string(),
        ..person_11 // таким синтаксисом мы можем воспользоваться если хоитм продублировать значения
                    // полей в person 12, которые есть в person_11 => называется интервальный синтаксис
    };

    println!("This is our friend {} {}, who has the same age: {} and the same total amount of money: {}, \
    as our first friend {} {}",
             person_12.first_name,
             person_12.second_name,
             person_12.total_age,
             person_12.total_money,
             person_11.first_name,
             person_11.second_name);



    //--------------------------------------------------------------//
    //Методы структуры в main -->>
    let egypt_triangle = Triangle { cat1: 3.0, cat2: 4.0 };
    let hypotenuse = egypt_triangle.find_hyp();
    println!("The hypotenuse of egypt triangle is equal to {}", hypotenuse);
    //--------------------------------------------------------------//

    let two_equal_sides_triangle = Triangle::create_isc(2.0); //Обращаемся к глобальной структуре Triangle
    // и через синтаксис связных функций создаем треугольник
    let hipo = two_equal_sides_triangle.find_hyp();
    let area = two_equal_sides_triangle.find_area();

    println!("The hipo of the Two_Equal_Sides_Triangle is {} and it's area is {}", hipo, area);
}