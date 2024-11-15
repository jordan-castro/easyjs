// Compiled by EasyJS version 0.1.8




function expect (method,error_msg){() => {{
            try {
                return method();

            } catch (e) {
                return error_msg;

            }
        }
    };
};


class Person{constructor (name,age){this.name = name;
this.age = age;
}greet (){}static a_static_method (){}async grow_up (){this.age = (1 + this.age);
}}


    const person = new Person("jordan", 22);

    person.greet();


    Person.a_static_method();



let new_person=Person('Evelyn',19);

new_person.greet();

Person.a_static_method();

