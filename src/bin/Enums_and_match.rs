enum Person {
    Adult,
    Underage
}


enum Say { //мы можем связывать перечисления с конкретным типом данных
    Hi(String),
    Bye(String),
    GM(String),
    GN(String)
}

enum MathOperations { //мы можем указывать что хотим работать с несколькими типами данных в enum
    Add(f64, f64),
    Sub(f64, f64),
    Mul(f64, f64),
    Div(f64, f64)
}

impl MathOperations { //имплементация методов перечислений
    fn math_operations(&self) -> f64 {
        match self {
            MathOperations::Add(a, b) => a + b,
            MathOperations::Sub(a, b) => a - b,//выполняем данную логику
            MathOperations::Mul(a, b) => a * b,
            MathOperations::Div(a, b) => a / b
        }
    }
}


fn main() {
    let person_adult = Person::Adult;

    match person_adult {
        Person::Adult => println!("You have the access to the club!"),
        Person::Underage => {
            println!("To young to enter, sorry!");
            println!("Wait a couple of years and come back!");
        }
    }

    let say = Say::Hi("Hello".to_string()); // здесь, мы привязали кокретную строку к конкретному
    // enum Hi, при выполнении этого enum нам будет возвращаться данная строка

    match say {
        Say::Hi(h) => println!("{}", h),//мы явно указали строку в let say выше
        //данным синтаксисом мы можем сохранить ее в переменную h и далее использовать уже в логике match
        Say::Bye(b) => println!("{}", b),
        Say::GM(gm) => println!("{}", gm),
        Say::GN(gn) => println!("{}", gn)
    }


    let mo = MathOperations::Sub(20.0, 5.0);

    match mo {
        MathOperations::Add(a, b) => println!("{}", a + b),
        MathOperations::Sub(a, b) => println!("{}", a - b),//выполняем данную логику
        MathOperations::Mul(a, b) => println!("{}", a * b),
        MathOperations::Div(a, b) => println!("{}", a / b)
    }


    //Fn math_operations в голом виде
    fn math_operations(mo: &MathOperations) -> f64 {
        match mo { //в данном случае мы будем возвращать результат операции напрямую из функции
            MathOperations::Add(a, b) => a + b,
            MathOperations::Sub(a, b) => a - b,//выполняем данную логику
            MathOperations::Mul(a, b) => a * b,
            MathOperations::Div(a, b) => a / b
        }
    }

    let another_mo = MathOperations::Mul(22.0, 11.0);
    let result_of_another_mo = math_operations(&another_mo);
    println!("{}", result_of_another_mo);
    //---------------------------------------------------------------------------//

    //Правильный код с имплементацией методов перечислений (на подобие методов структур)

    let result_with_correct_imple = another_mo.math_operations();
    println!("{}", result_with_correct_imple);

}